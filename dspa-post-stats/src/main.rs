#![allow(unused_imports)]

use std::cmp::Ordering;
use std::collections::{BTreeMap, HashMap, HashSet};
use std::env::args;
use std::fmt;
use std::mem::swap;
use std::sync::Arc;
use std::sync::atomic::AtomicU64;

use chrono::{DateTime, TimeZone, Utc};
use diesel::prelude::*;
use diesel::r2d2::ConnectionManager;
use diesel::PgConnection;
use r2d2::Pool;
use serde_derive::{Deserialize, Serialize};
use timely::dataflow::channels::pact::Pipeline;
use timely::dataflow::operators::{
    Broadcast, Capability, Concat, Concatenate, Exchange, Inspect, Map, Operator, Probe,
    UnorderedInput,
};
use timely::dataflow::{ProbeHandle, Scope, Stream};
use zmq::Context;

use dspa_lib::operators::{Ordered, streams};
use dspa_lib::records::{CommentRecord, LikeRecord, PostRecord, StreamRecord};
use dspa_lib::schema::{comment, like_ as like, post};
use dspa_lib::{Topic, MAX_DELAY, DATABASE_URL};

use dspa_post_stats::{ActivePost, ActivePostEvent};
use dspa_post_stats::operators::{PostStats, DisplayPostStats};

fn main() {
    let pool = Arc::new(
        Pool::builder()
            .max_size(16)
            .build(ConnectionManager::<PgConnection>::new(
                DATABASE_URL,
            ))
            .unwrap(),
    );

    let ctx = Context::new();

    timely::execute(timely::Configuration::Thread, move |worker| {
    // timely::execute(timely::Configuration::Process(num_cpus::get()), move |worker| {
        let idx = worker.index();
        let peers = worker.peers();

        worker.dataflow(|scope| {
            let pool = pool.clone();

            let (posts, comments, likes) = streams(scope, idx, &ctx);

            let comment_events = {
                let pool = pool.clone();
                comments
                    .exchange(|comment| comment.id as u64)
                    .ordered(idx, peers, pool.clone(), &posts)
                    .map(move |comment| {
                        let connection = pool.get().unwrap();
    
                        ActivePostEvent::Comment {
                            post_id: comment.root(&connection).unwrap().id,
                            person_id: comment.person_id,
                        }
                    })
            };

            let like_events = likes
                    .exchange(|like| like.post_id as u64)
                    .ordered(idx, peers, pool.clone(), &posts)
                    .map(|like| ActivePostEvent::Like {
                        post_id: like.post_id,
                        person_id: like.person_id,
                    });

            comment_events
                .concat(&like_events)
                // .exchange(|event| event.id() as u64)
                .post_stats(pool.clone())
                .display();
        });

    })
    .unwrap();
}

