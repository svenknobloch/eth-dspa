use std::collections::BTreeMap;
use std::sync::Arc;
use std::mem::swap;

use diesel::prelude::*;
use diesel::r2d2::ConnectionManager;
use diesel::PgConnection;
use r2d2::Pool;
use timely::dataflow::{Scope, Stream};
use timely::dataflow::operators::{Operator, Map};
use timely::Data;
use timely::dataflow::channels::pact::{Pipeline, Exchange};

use dspa_lib::records::{PersonRecord};
use dspa_lib::schema::*;

use crate::{RecommendationEvent};

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
    G: Scope<Timestamp = u64>,
    D: Data,
{
    fn window(&self, size: u64, frequency: u64) -> Stream<G, D>;
}


impl<G> Window<G, RecommendationEvent> for Stream<G, RecommendationEvent>
where
    G: Scope<Timestamp = u64>,
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
    G: Scope<Timestamp = u64>,
{
    fn recommendations(&self, pool: &Arc<Pool<ConnectionManager<PgConnection>>>, users: &[i32]) -> Stream<G, String>;
}

impl<G> Recommendations<G> for Stream<G, RecommendationEvent>
where
    G: Scope<Timestamp = u64>,
{
    fn recommendations(&self, pool: &Arc<Pool<ConnectionManager<PgConnection>>>, users: &[i32]) -> Stream<G, String> {
        let pool = pool.clone();
        let connection = pool.get().unwrap();

        let users = users.iter().map(|id| person::table.filter(person::id.eq(*id)).first::<PersonRecord>(&connection).unwrap()).collect::<Vec<_>>();

        self.unary(Pipeline, "Recommendations", |_, _| {

            let mut vec = Vec::new();
            move |input, output| {
                input.for_each(|cap, data| {
                    data.swap(&mut vec);

                    // TODO

                });
            }
        })
    }
}
