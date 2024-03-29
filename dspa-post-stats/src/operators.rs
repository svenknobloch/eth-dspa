use std::collections::{HashMap, HashSet};
use std::sync::Arc;

use diesel::r2d2::ConnectionManager;
use diesel::PgConnection;
use r2d2::Pool;
use timely::dataflow::channels::pact::Exchange;
use timely::dataflow::operators::Operator;
use timely::dataflow::{Scope, Stream};

use crate::{ActivePost, ActivePostEvent};

const MIN_30: u64 = 30 * 60;
const HR_12: u64 = 60 * 60 * 12;

#[inline]
fn round_to_next(current: u64) -> u64 {
    let rem = current % MIN_30;

    if rem == 0 {
        current
    } else {
        current + MIN_30 - rem
    }
}

pub trait PostStats<G>
where
    G: Scope<Timestamp = u64>,
{
    fn post_stats(&self, pool: Arc<Pool<ConnectionManager<PgConnection>>>) -> Stream<G, String>;
}

impl<G> PostStats<G> for Stream<G, ActivePostEvent>
where
    G: Scope<Timestamp = u64>,
{
    fn post_stats(&self, pool: Arc<Pool<ConnectionManager<PgConnection>>>) -> Stream<G, String> {
        let pool = pool.clone();

        let mut seen: HashSet<i32> = HashSet::new();

        // Post Id => Post Data
        let mut active: HashMap<i32, ActivePost> = HashMap::new();

        // Post Id => Expiry Time
        let mut expiry: HashMap<i32, u64> = HashMap::new();

        let mut vec = Vec::new();
        self.unary_notify(
            Exchange::new(|event: &ActivePostEvent| event.id() as u64),
            "PostStats",
            None,
            move |input, output, notificator| {
                input.for_each(|cap, data| {
                    // TODO: Fetch existing active posts from DB
                    data.swap(&mut vec);

                    vec.drain(..).for_each(|event| {
                        let id = event.id();

                        // Only evaluate if not seen or currently active
                        if !seen.contains(&id) || active.contains_key(&id) {
                            active
                                .entry(id)
                                .or_insert_with(|| ActivePost::new(id))
                                .update(event);
                            expiry
                                .entry(id)
                                .and_modify(|time| *time = *cap.time() + HR_12)
                                .or_insert_with(|| *cap.time() + HR_12);

                            seen.insert(id);
                        }
                    });

                    notificator.notify_at(cap.delayed(&round_to_next(*cap.time())));
                });

                notificator.for_each(|cap, _, notificator| {
                    // Remove inactive posts
                    expiry.retain(|id, expiry_time| {
                        if *expiry_time < *cap.time() {
                            active.remove(&id);
                            false
                        } else {
                            true
                        }
                    });

                    // Output formatted strings
                    output
                        .session(&cap)
                        .give_iterator(active.values().map(|post| format!("{}", post)));

                    if !active.is_empty() {
                        notificator.notify_at(cap.delayed(&round_to_next(cap.time() + 1)));
                    }
                });
            },
        )
    }
}
