use std::collections::BTreeMap;
use std::sync::Arc;
use std::mem::swap;
use std::collections::HashMap;
use std::collections::HashSet;
use std::iter::FromIterator;

use diesel::prelude::*;
use diesel::r2d2::ConnectionManager;
use diesel::PgConnection;
use r2d2::Pool;
use timely::dataflow::{Scope, Stream};
use timely::dataflow::operators::{Operator};
use timely::Data;
use timely::dataflow::channels::pact::{Pipeline, Exchange};

use dspa_lib::records::PersonRecord;
use dspa_lib::schema::*;

use crate::RecommendationEvent;

#[inline]
fn round_to_next(current: u64, multiple: u64) -> u64 {
    let rem = current % multiple;

    if rem == 0 {
        current
    } else {
        current + multiple - rem
    }
}

pub trait Window<G, D>
    where
        G: Scope<Timestamp=u64>,
        D: Data,
{
    fn window(&self, size: u64, frequency: u64) -> Stream<G, D>;
}


impl<G> Window<G, RecommendationEvent> for Stream<G, RecommendationEvent>
    where
        G: Scope<Timestamp=u64>,
{
    fn window(&self, size: u64, frequency: u64) -> Stream<G, RecommendationEvent> {
        let mut active: BTreeMap<u64, Vec<RecommendationEvent>> = BTreeMap::new();

        let mut vec = Vec::new();
        self.unary_notify(Pipeline, "Window", None, move |input, output, notificator| {
            input.for_each(|cap, data| {
                data.swap(&mut vec);

                active.entry(*cap.time()).or_default().extend(vec.drain(..));
                notificator.notify_at(cap.delayed(&(round_to_next(*cap.time(), frequency))));
            });

            notificator.for_each(|cap, _, notificator| {
                // Remove all events out of given window
                let mut remaining = active.split_off(&(*cap.time() - size - 1));

                // Make remaining the new active
                swap(&mut active, &mut remaining);

                output.session(&cap).give_iterator(active.values().flatten().cloned());

                // If data is available, queue for next iteration
                if !remaining.is_empty() {
                    notificator.notify_at(cap.delayed(&(round_to_next(*cap.time() + 1, frequency))))
                }
            });
        })
    }
}


pub trait Recommendations<G>
    where
        G: Scope<Timestamp=u64>,
{
    fn recommendations(&self, pool: &Arc<Pool<ConnectionManager<PgConnection>>>, users: &[i32]) -> Stream<G, String>;
}

//pub struct RecommendationFeatures {
//    pub person_id: i32,
//    pub person_id: i32
//}

impl<G> Recommendations<G> for Stream<G, RecommendationEvent>
    where
        G: Scope<Timestamp=u64>,
{
    fn recommendations(&self, pool: &Arc<Pool<ConnectionManager<PgConnection>>>, users: &[i32]) -> Stream<G, String> {
        let pool = pool.clone();
        let connection = pool.get().unwrap();

//        let users = users.iter().map(|id| person::table.filter(person::id.eq(*id)).first::<PersonRecord>(&connection).unwrap()).collect::<Vec<_>>();
        let users: HashSet<i32> = users.iter().cloned().collect(); // HashSet::from_iter(users); // .iter().collect::<HashSet<_>>();
        let friends: HashMap<i32, HashSet<i32>> = HashMap::new();


        // Counts how many items two users have in common
        self.unary(Pipeline, "Recommendations", |_, _| {
            let mut vec = Vec::new();
            move |input, output| {

                let mut user_to_user_recommendation: HashMap<(i32, i32), i32> = HashMap::new();

                let mut map_post_id: HashMap<i32, Vec<i32>> = HashMap::new();
                let mut map_forum_id: HashMap<i32, Vec<i32>> = HashMap::new();
                let mut map_place_id: HashMap<i32, Vec<i32>> = HashMap::new();

                let mut user_related_post_ids: HashMap<i32, Vec<i32>> = HashMap::new();
                let mut user_related_forum_ids: HashMap<i32, Vec<i32>> = HashMap::new();
                let mut user_related_place_ids: HashMap<i32, Vec<i32>> = HashMap::new();

                // Hashmap, such that we can extract the top 5 users later on
                let mut user_vec: HashMap<i32, Vec<(i32, i32)>> = HashMap::new();

                input.for_each(|cap, data| {
                    data.swap(&mut vec);

//                    println!("{:?}", vec);

                    vec.drain(..).for_each(
                        |x| {
                            let map = match x {
                                RecommendationEvent::Post(post) => post.hashmap(),
                                RecommendationEvent::Like(like) => like.hashmap(),
                                RecommendationEvent::Comment(comment) => comment.hashmap(&connection),
                            };

                            map.keys().for_each(
                                |x| {
                                    let user_id = *map.get("person_id").unwrap();
                                    // Append the user into the item vector
                                    match *x {
                                        "post_id" => {
                                            map_post_id.entry(*map.get(x).unwrap()).or_default().push(user_id);
                                            if users.contains(&user_id) {
                                                user_related_post_ids.entry(user_id).or_default().push(*map.get(x).unwrap());
                                            }
                                        },
                                        "forum_id" => {
                                            map_forum_id.entry(*map.get(x).unwrap()).or_default().push(user_id);
                                            if users.contains(&user_id) {
                                                user_related_forum_ids.entry(user_id).or_default().push(*map.get(x).unwrap());
                                            }
                                        },
                                        "place_id" => {
                                            map_place_id.entry(*map.get(x).unwrap()).or_default().push(user_id);
                                            if users.contains(&user_id) {
                                                user_related_place_ids.entry(user_id).or_default().push(*map.get(x).unwrap());
                                            }
                                        },
                                        _ => ()
                                    };
                                }
                            );
                        }
                    );

                    // For each use in the above item

                    // Iterate through posts
                    // Iterate through all users (and the respective posts they liked
                    for (user_id, all_post_ids) in user_related_post_ids.iter() {
                        // Iterate through all the posts that this user, has
                        for post_id in all_post_ids.iter() {
                            // Get the corresponding vector
                            let similar_user_vector = map_post_id.get(post_id).unwrap();
                            // Iterate through all the posts than similar_users
                            for similar_user_id in similar_user_vector {
                                *user_to_user_recommendation.entry((*user_id, *similar_user_id)).or_default() += 1;
                            }
                        }
                    }

                    // Iterate through forums
                    for (user_id, all_forum_ids) in user_related_forum_ids.iter() {
                        // Iterate through all the posts that this user, has
                        for forum_id in all_forum_ids.iter() {
                            // Get the corresponding vector
                            let similar_user_vector = map_forum_id.get(forum_id).unwrap();
                            // Iterate through all the posts than similar_users
                            for similar_user_id in similar_user_vector {
                                *user_to_user_recommendation.entry((*user_id, *similar_user_id)).or_default() += 1;
                            }
                        }
                    }

                    // Iterate through places
                    for (user_id, all_place_ids) in user_related_place_ids.iter() {
                        // Iterate through all the posts that this user, has
                        for place_id in all_place_ids.iter() {
                            // Get the corresponding vector
                            let similar_user_vector = map_place_id.get(place_id).unwrap();
                            // Iterate through all the posts than similar_users
                            for similar_user_id in similar_user_vector {
                                *user_to_user_recommendation.entry((*user_id, *similar_user_id)).or_default() += 1;
                            }
                        }
                    }

                    // Iterate over all user-pairs
                    let mut transformed_map: HashMap<i32, HashMap<i32, i32>> = HashMap::new();
                    user_to_user_recommendation.iter().for_each(
                        |((u1, u2), count)| {
                            *transformed_map.entry(*u1).or_default()
                                .entry(*u2).or_default() = *count;
                        }
                    );

                    let mut recommendation_vector: HashMap<i32, Vec<i32>> = HashMap::new();

                    transformed_map.iter().for_each(
                        |(u1, user_hashmap)| {
                            let mut vec_u2similarity_count = user_hashmap
                                .iter()
                                .map(|(similar_user_id, count)| (count, similar_user_id))
                                .collect::<Vec<_>>();

                            // Vec<(count, similar_user_id)>
                            vec_u2similarity_count.sort_by_key(|k| -1 * k.0);

                            let top_5_users: Vec<i32> = vec_u2similarity_count
                                .iter()
                                .map(|x| *x.1)
                                .filter(|uid| !friends.get(u1).unwrap().contains(uid) )
                                .take(5)
                                .collect();

                            *recommendation_vector.entry(*u1).or_default() = top_5_users;

                        }
                    );

                    println!("{:?}", recommendation_vector);

//                    recommendation_vector

//                    recommendation_vector

                    // output.session(&cap).give(id);
                });
            }
        })
    }
}
