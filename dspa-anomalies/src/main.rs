use std::sync::Arc;

use diesel::r2d2::ConnectionManager;
use diesel::PgConnection;
use r2d2::Pool;
use timely::dataflow::operators::{Exchange, Map, Concat, Inspect};

use zmq::Context;

use dspa_lib::{DATABASE_URL};
use dspa_lib::operators::{streams, Ordered};

use dspa_anomalies::{AnomalyEvent, ARGS};
use dspa_anomalies::operators::{};

fn main() {
    lazy_static::initialize(&ARGS);

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

            let (posts, comments, likes) = streams(scope, idx, &ctx);

            let comment_events = comments
                .exchange(|comment| comment.id as u64)
                .ordered(idx, peers, pool.clone(), &posts)
                .map(|comment| AnomalyEvent::Comment(comment));
            let like_events = likes
                .exchange(|like| like.post_id as u64)
                .ordered(idx, peers, pool.clone(), &posts)
                .map(|like| AnomalyEvent::Like(like));
            let post_events = posts
                .exchange(|post| post.id as u64)
                .map(|post| AnomalyEvent::Post(post));

            post_events
                .concat(&comment_events)
                .concat(&like_events);

        });
    }).unwrap();
}
