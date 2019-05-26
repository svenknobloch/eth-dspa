use std::collections::BTreeMap;

use bincode::serialize;
use timely::dataflow::channels::pact::Pipeline;
use timely::dataflow::operators::Operator;
use timely::dataflow::{Scope, Stream};
use zmq::{Context, SocketType, SNDMORE};

use dspa_lib::records::StreamRecord;
use dspa_lib::SOURCE_SOCKET;

pub trait Publish<G, D>
where
    G: Scope,
    D: StreamRecord,
{
    fn publish(&self, ctx: &Context);
}

impl<G, D> Publish<G, D> for Stream<G, D>
where
    G: Scope<Timestamp = u64>,
    D: StreamRecord,
{
    fn publish(&self, ctx: &Context) {
        let socket = ctx.socket(SocketType::PUSH).unwrap();
        socket
            .connect(&format!("ipc://{}", SOURCE_SOCKET))
            .expect("Failed to connect!");

        let topic = D::TOPIC.to_string();

        let mut vec = Vec::new();
        self.sink(Pipeline, "Publish", move |input| {
            input.for_each(|cap, data| {
                data.swap(&mut vec);

                vec.drain(..).for_each(|record| {
                    println!(
                        "{} record {{ time: {}, timestamp: {}, id: {:?}}} sent!",
                        D::TOPIC.to_string(),
                        *cap.time(),
                        record.timestamp(),
                        record.id()
                    );

                    socket.send(&topic, SNDMORE).unwrap();
                    socket.send(&serialize(&record).unwrap(), 0).unwrap();
                });
            });
        });
    }
}
