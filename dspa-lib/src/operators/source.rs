use bincode::deserialize;
use timely::dataflow::operators::generic::operator::{empty, source};
use timely::dataflow::operators::{Map, Partition};
use timely::dataflow::{Scope, Stream};
use zmq::{Context, SocketType};

use crate::{
    records::{CommentRecord, LikeRecord, PostRecord},
    StreamEvent, Topic, DATA_SOCKET, MAX_DELAY, SOCKET_TIMEOUT,
};

pub fn streams<G>(
    scope: &G,
    idx: usize,
    ctx: &Context,
) -> (
    Stream<G, PostRecord>,
    Stream<G, CommentRecord>,
    Stream<G, LikeRecord>,
)
where
    G: Scope<Timestamp = u64>,
{
    if idx == 0 {
        let events = source(scope, "Stream Source", |capability, info| {
            let activator = scope.activator_for(&info.address[..]);
            let mut cap = Some(capability);

            let socket = ctx.socket(SocketType::SUB).unwrap();
            socket.connect(&format!("ipc://{}", DATA_SOCKET)).unwrap();
            socket
                .set_subscribe(Topic::Post.to_string().as_bytes())
                .unwrap();
            socket
                .set_subscribe(Topic::Comment.to_string().as_bytes())
                .unwrap();
            socket
                .set_subscribe(Topic::Like.to_string().as_bytes())
                .unwrap();
            socket
                .set_subscribe(Topic::EOS.to_string().as_bytes())
                .unwrap();
            socket.set_rcvtimeo(SOCKET_TIMEOUT).unwrap();

            move |output| {
                let mut done = false;
                if let Some(cap) = cap.as_mut() {
                    if let Ok(Ok(topic)) = socket.recv_bytes(0).map(String::from_utf8) {
                        if topic == Topic::EOS.to_string() {
                            done = true;
                        } else {
                            loop {
                                if let Ok(data) = socket.recv_bytes(0) {
                                    let event = match topic.as_str() {
                                        "post" => StreamEvent::Post(deserialize(&data).unwrap()),
                                        "comment" => {
                                            StreamEvent::Comment(deserialize(&data).unwrap())
                                        }
                                        "like" => StreamEvent::Like(deserialize(&data).unwrap()),
                                        _ => unreachable!(),
                                    };

                                    let event_time = event.timestamp() as u64;

                                    if event_time >= *cap.time() {
                                        let max_delay_time = event_time - MAX_DELAY;
                                        if max_delay_time > *cap.time() {
                                            // Downgrade shared timestamp
                                            cap.downgrade(&max_delay_time);
                                        }
                                        // println!("INSERT {} record {{ time: {}, timestamp: {}, id: {:?} }}", topic, *cap.time(), event.timestamp(), event.id());

                                        output.session(&cap.delayed(&event_time)).give(event);
                                    } else {
                                        // println!("DISCARD {} record {{ time: {}, timestamp: {}, id: {:?} }}", topic, *cap.time(), event.timestamp(), event.id());
                                    }
                                    break;
                                }
                            }
                        }
                    }
                }

                if done {
                    cap = None;
                } else {
                    activator.activate();
                }
            }
        });

        let streams = events.partition(3, |event| match event {
            StreamEvent::Post(_) => (0, event),
            StreamEvent::Comment(_) => (1, event),
            StreamEvent::Like(_) => (2, event),
        });

        (
            streams[0].map(|event| {
                if let StreamEvent::Post(record) = event {
                    record
                } else {
                    unreachable!()
                }
            }),
            streams[1].map(|event| {
                if let StreamEvent::Comment(record) = event {
                    record
                } else {
                    unreachable!()
                }
            }),
            streams[2].map(|event| {
                if let StreamEvent::Like(record) = event {
                    record
                } else {
                    unreachable!()
                }
            }),
        )
    } else {
        (empty(scope), empty(scope), empty(scope))
    }
}
