
use rand::{Rng, thread_rng};
use timely::dataflow::operators::Delay;
use timely::dataflow::{Scope, Stream};

use dspa_lib::records::{StreamRecord};

pub trait BoundedDelay<G, D>
where
    G: Scope,
    D: StreamRecord,
{
    fn bounded_delay(&self, bound: u64) -> Stream<G, D>;
}

impl<G, D> BoundedDelay<G, D> for Stream<G, D>
where
    G: Scope<Timestamp = u64>,
    D: StreamRecord,
{
    fn bounded_delay(&self, bound: u64) -> Stream<G, D> {
        self.delay(move |_, timestamp| {
            let mut rng = thread_rng();

            if bound == 0 {
                *timestamp as u64
            } else {
                *timestamp as u64 + rng.gen_range(0, bound)
            }
        })
    }
}

