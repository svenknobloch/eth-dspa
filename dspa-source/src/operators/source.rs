use std::path::PathBuf;
use std::time::{Duration, Instant};

use csv::ReaderBuilder;
use itertools::Itertools;
use timely::dataflow::operators::generic::operator::{empty, source};
use timely::dataflow::operators::{Map, Partition};
use timely::dataflow::{Scope, Stream};

use dspa_lib::records::{CommentRecord, LikeRecord, PostRecord, Record, StreamRecord};
use dspa_lib::StreamEvent;

use crate::ARGS;

pub fn csv_source<G, D>(scope: &G, idx: usize, path: &PathBuf) -> Stream<G, D>
where
    G: Scope<Timestamp = u64>,
    D: Record,
{
    if idx == 0 {
        source(scope, "CSV Source", |capability, info| {
            let path = path.join(<D as Record>::FILENAME);

            let activator = scope.activator_for(&info.address[..]);
            let mut cap = Some(capability);

            let mut records = ReaderBuilder::new()
                .delimiter(b'|')
                .has_headers(true)
                .from_path(path)
                .expect("Failed to open file")
                .into_deserialize::<D>()
                .map(Result::unwrap);

            move |output| {
                let mut done = false;

                if let Some(cap) = cap.as_mut() {
                    if let Some(record) = records.next() {
                        output.session(&cap).give(record);
                    } else {
                        done = true;
                    }
                }

                if done {
                    cap = None;
                } else {
                    activator.activate();
                }
            }
        })
    } else {
        empty(scope)
    }
}

fn records<D>(path: &PathBuf) -> impl Iterator<Item = D>
where
    D: StreamRecord,
{
    ReaderBuilder::new()
        .delimiter(b'|')
        .has_headers(true)
        .from_path(path)
        .expect("Failed to open file")
        .into_deserialize::<D>()
        .map(Result::unwrap)
}

pub fn csv_stream_source<G>(
    scope: &G,
    idx: usize,
    path: &PathBuf,
) -> (
    Stream<G, PostRecord>,
    Stream<G, CommentRecord>,
    Stream<G, LikeRecord>,
)
where
    G: Scope<Timestamp = u64>,
{
    if idx == 0 {
        let events = source(scope, "CSV Stream Source", |capability, info| {
            // let path = path.join(<D as Record>::FILENAME);

            let activator = scope.activator_for(&info.address[..]);
            let mut cap = Some(capability);

            let posts = records::<PostRecord>(&path.join(PostRecord::FILENAME))
                .map(StreamEvent::Post)
                .peekable();
            let comments = records::<CommentRecord>(&path.join(CommentRecord::FILENAME))
                .map(StreamEvent::Comment)
                .peekable();
            let likes = records::<LikeRecord>(&path.join(LikeRecord::FILENAME))
                .map(StreamEvent::Like)
                .peekable();

            // Merge records according to timestamp
            let mut events = posts
                .merge_by(comments, |a, b| a.timestamp() < b.timestamp())
                .merge_by(likes, |a, b| a.timestamp() < b.timestamp())
                .peekable();

            let timestamp_logical = events.peek().unwrap().timestamp() as u64;
            let timestamp_physical = Instant::now();
            move |output| {
                let mut done = false;

                if let Some(cap) = cap.as_mut() {
                    if let Some(event) = events.peek() {
                        // Next event exists

                        // Get timestamp of next record
                        let timestamp = event.timestamp() as u64;

                        // Get logical time since timestamp for next record
                        let logical_duration = Duration::from_secs(timestamp - timestamp_logical);

                        // Get logical time since timestamp for elapsed time with speedup
                        let physical_duration = timestamp_physical.elapsed() * ARGS.speedup as u32;

                        if logical_duration < physical_duration {
                            // Logical event timestamp surpassed, emit event and update frontier to that event
                            cap.downgrade(&(timestamp_logical + logical_duration.as_secs()));
                            output.session(&cap).give(events.next().unwrap())
                        } else {
                            // Logical event timestamp still in future, update frontier
                            cap.downgrade(&(timestamp_logical + physical_duration.as_secs()));
                        }
                    // println!("CAP: {:?}", *cap.time());
                    } else {
                        // No more events
                        done = true;
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
