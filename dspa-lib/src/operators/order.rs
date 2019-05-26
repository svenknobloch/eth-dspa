use std::collections::HashMap;
use std::sync::Arc;

use crate::schema::post;
use diesel::prelude::*;
use diesel::r2d2::ConnectionManager;
use diesel::PgConnection;
use either::Either;
use r2d2::Pool;
use serde_derive::{Deserialize, Serialize};
use timely::dataflow::channels::pact::Pipeline;
use timely::dataflow::operators::{Broadcast, Capability, Concat, Map, Operator};
use timely::dataflow::{Scope, Stream};

use crate::records::{CommentRecord, LikeRecord, PostRecord};

pub trait Ordered<G, D1, D2>
where
    G: Scope<Timestamp = u64>,
{
    fn ordered(
        &self,
        idx: usize,
        peers: usize,
        pool: Arc<Pool<ConnectionManager<PgConnection>>>,
        dependency: &Stream<G, D2>,
    ) -> Stream<G, D1>;
}

#[derive(Clone, Debug, Serialize, Deserialize)]
enum CommentOrderEvent {
    Post(PostRecord),
    Comment(CommentRecord),
}

impl<G> Ordered<G, CommentRecord, PostRecord> for Stream<G, CommentRecord>
where
    G: Scope<Timestamp = u64>,
{
    fn ordered(
        &self,
        idx: usize,
        peers: usize,
        pool: Arc<Pool<ConnectionManager<PgConnection>>>,
        dependency: &Stream<G, PostRecord>,
    ) -> Stream<G, CommentRecord> {
        // ! Broadcast before map to prevent timely panic
        let posts = dependency.broadcast().map(CommentOrderEvent::Post);
        let comments = self.broadcast().map(CommentOrderEvent::Comment);

        posts
            .concat(&comments)
            .unary(Pipeline, "Comment Ordered", move |_, _| {
                // TODO: Memory leak if parent comment/post dropped
                let mut pending: HashMap<i32, Vec<CommentRecord>> = HashMap::new();
                let mut pending_children: HashMap<i32, Vec<CommentRecord>> = HashMap::new();

                move |input, output| {
                    let connection = pool.get().unwrap();

                    let mut vec = Vec::new();
                    input.for_each(|cap, data| {
                        data.swap(&mut vec);

                        vec.drain(..).for_each(|event| {
                            match event {
                                CommentOrderEvent::Post(post) => {
                                    if let Some(mut records) = pending.remove(&post.id) {
                                        output.session(&cap).give_vec(&mut records);
                                    }
                                }
                                CommentOrderEvent::Comment(comment) => {
                                    match comment.available_parent(&connection) {
                                        // Known post
                                        Some(Either::Left(post)) => {
                                            // Ordered, ouput self and children
                                            let id = comment.id;
                                            let mut session = output.session(&cap);

                                            // Output self if responsible
                                            if comment.id % peers as i32 == idx as i32 {
                                                session.give(comment);
                                            }

                                            // Output children
                                            if let Some(mut records) = pending_children.remove(&id)
                                            {
                                                session.give_vec(&mut records);
                                            }
                                        }
                                        // Known parent comment
                                        Some(Either::Right(parent)) => {
                                            let children = pending_children.remove(&comment.id);

                                            let vec =
                                                pending_children.entry(parent.id).or_default();

                                            // Push self onto parent chain if responsible
                                            if comment.id % peers as i32 == idx as i32 {
                                                vec.push(comment);
                                            }

                                            // Push children onto parent chain
                                            if let Some(records) = children {
                                                vec.extend(records);
                                            }
                                        }
                                        // No existing parent
                                        None => {
                                            // See if parent should be post or comment
                                            match comment.parent_id() {
                                                // Post, insert into pending
                                                Either::Left(parent_id) => {
                                                    if comment.id % peers as i32 == idx as i32 {
                                                        pending
                                                            .entry(parent_id)
                                                            .or_default()
                                                            .push(comment);
                                                    }
                                                }
                                                // Comment, insert into pending_children
                                                Either::Right(parent_id) => {
                                                    if comment.id % peers as i32 == idx as i32 {
                                                        pending_children
                                                            .entry(parent_id)
                                                            .or_default()
                                                            .push(comment);
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        });
                    });
                }
            })
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
enum LikeOrderEvent {
    Post(PostRecord),
    Like(LikeRecord),
}

impl<G> Ordered<G, LikeRecord, PostRecord> for Stream<G, LikeRecord>
where
    G: Scope<Timestamp = u64>,
{
    fn ordered(
        &self,
        idx: usize,
        peers: usize,
        pool: Arc<Pool<ConnectionManager<PgConnection>>>,
        dependency: &Stream<G, PostRecord>,
    ) -> Stream<G, LikeRecord> {
        // ! Broadcast before map to prevent timely panic
        let posts = dependency.broadcast().map(LikeOrderEvent::Post);
        let likes = self.map(LikeOrderEvent::Like);

        posts
            .concat(&likes)
            .unary(Pipeline, "Like Ordered", move |cap, _| {
                let mut pending: HashMap<i32, Vec<LikeRecord>> = HashMap::new();

                move |input, output| {
                    let connection = pool.get().unwrap();
                    let mut vec = Vec::new();
                    input.for_each(|cap, data| {
                        data.swap(&mut vec);

                        vec.drain(..).for_each(|event| match event {
                            LikeOrderEvent::Post(post) => {
                                if let Some(mut records) = pending.remove(&post.id) {
                                    output.session(&cap).give_vec(&mut records);
                                }
                            }
                            LikeOrderEvent::Like(like) => {
                                if like.root(&connection).is_some() {
                                    output.session(&cap).give(like);
                                } else {
                                    pending.entry(like.post_id).or_default().push(like);
                                }
                            }
                        });
                    });
                }
            })
    }
}
