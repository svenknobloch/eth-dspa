use std::sync::atomic::AtomicU64;
use std::sync::Arc;
use std::time::Instant;

use diesel::prelude::*;
use diesel::r2d2::ConnectionManager;
use r2d2::Pool;
use rand::{thread_rng, RngCore};
use timely;
use timely::dataflow::operators::{Exchange, Inspect};
use zmq::Context;

use dspa_lib::records::*;
use dspa_lib::schema::*;
use dspa_lib::DATABASE_URL;

use dspa_source::operators::{csv_source, csv_stream_source, BoundedDelay, Insert, Publish};
use dspa_source::ARGS;

fn main() {
    let pool = Arc::new(
        Pool::builder()
            .max_size(16)
            .build(ConnectionManager::new(DATABASE_URL))
            .unwrap(),
    );

    if ARGS.tables {
        eprintln!("Dropping existing tables!");
        {
            let connection = pool.get().unwrap();

            // Drop relations
            diesel::delete(forum_has_member::table)
                .execute(&connection)
                .unwrap();
            diesel::delete(forum_has_moderator::table)
                .execute(&connection)
                .unwrap();
            diesel::delete(forum_has_tag::table)
                .execute(&connection)
                .unwrap();
            diesel::delete(organization_is_located_in::table)
                .execute(&connection)
                .unwrap();
            diesel::delete(person_email::table)
                .execute(&connection)
                .unwrap();
            diesel::delete(person_has_interest::table)
                .execute(&connection)
                .unwrap();
            diesel::delete(person_is_located_in::table)
                .execute(&connection)
                .unwrap();
            diesel::delete(person_knows::table)
                .execute(&connection)
                .unwrap();
            diesel::delete(person_speaks::table)
                .execute(&connection)
                .unwrap();
            diesel::delete(person_study_at::table)
                .execute(&connection)
                .unwrap();
            diesel::delete(person_work_at::table)
                .execute(&connection)
                .unwrap();
            diesel::delete(tag_has_type::table)
                .execute(&connection)
                .unwrap();
            diesel::delete(tag_class_is_subclass_of::table)
                .execute(&connection)
                .unwrap();
            diesel::delete(place_is_part_of::table)
                .execute(&connection)
                .unwrap();

            // Drop data
            diesel::delete(forum::table).execute(&connection).unwrap();
            diesel::delete(organization::table)
                .execute(&connection)
                .unwrap();
            diesel::delete(person::table).execute(&connection).unwrap();
            diesel::delete(place::table).execute(&connection).unwrap();
            diesel::delete(tag::table).execute(&connection).unwrap();
            diesel::delete(tag_class::table)
                .execute(&connection)
                .unwrap();
        }
        eprintln!("Done dropping existing tables!");

        {
            let pool = pool.clone();
            let path = ARGS.path.join("tables/");

            eprintln!("Inserting data records!");
            timely::execute(
                timely::Configuration::Process(num_cpus::get()),
                move |worker| {
                    let idx = worker.index();

                    worker.dataflow(|scope| {
                        csv_source::<_, ForumRecord>(scope, idx, &path)
                            .exchange(|_| thread_rng().next_u64())
                            .insert(pool.clone());
                        csv_source::<_, OrganizationRecord>(scope, idx, &path)
                            .exchange(|_| thread_rng().next_u64())
                            .insert(pool.clone());
                        csv_source::<_, PersonRecord>(scope, idx, &path)
                            .exchange(|_| thread_rng().next_u64())
                            .insert(pool.clone());
                        csv_source::<_, PlaceRecord>(scope, idx, &path)
                            .exchange(|_| thread_rng().next_u64())
                            .insert(pool.clone());
                        csv_source::<_, TagRecord>(scope, idx, &path)
                            .exchange(|_| thread_rng().next_u64())
                            .insert(pool.clone());
                        csv_source::<_, TagClassRecord>(scope, idx, &path)
                            .exchange(|_| thread_rng().next_u64())
                            .insert(pool.clone());
                    });
                },
            )
            .unwrap();
            eprintln!("Done inserting data records!");
        }

        {
            let pool = pool.clone();
            let path = ARGS.path.join("tables/");

            eprintln!("Inserting relation records!");
            timely::execute(
                timely::Configuration::Process(num_cpus::get()),
                move |worker| {
                    let idx = worker.index();

                    worker.dataflow(|scope| {
                        csv_source::<_, ForumHasMemberRecord>(scope, idx, &path)
                            .exchange(|_| thread_rng().next_u64())
                            .insert(pool.clone());
                        csv_source::<_, ForumHasModeratorRecord>(scope, idx, &path)
                            .exchange(|_| thread_rng().next_u64())
                            .insert(pool.clone());
                        csv_source::<_, ForumHasTagRecord>(scope, idx, &path)
                            .exchange(|_| thread_rng().next_u64())
                            .insert(pool.clone());
                        csv_source::<_, OrganizationIsLocatedInRecord>(scope, idx, &path)
                            .exchange(|_| thread_rng().next_u64())
                            .insert(pool.clone());
                        csv_source::<_, PersonEmailRecord>(scope, idx, &path)
                            .exchange(|_| thread_rng().next_u64())
                            .insert(pool.clone());
                        csv_source::<_, PersonHasInterestRecord>(scope, idx, &path)
                            .exchange(|_| thread_rng().next_u64())
                            .insert(pool.clone());
                        csv_source::<_, PersonIsLocatedInRecord>(scope, idx, &path)
                            .exchange(|_| thread_rng().next_u64())
                            .insert(pool.clone());
                        csv_source::<_, PersonKnowsRecord>(scope, idx, &path)
                            .exchange(|_| thread_rng().next_u64())
                            .insert(pool.clone());
                        csv_source::<_, PersonSpeaksRecord>(scope, idx, &path)
                            .exchange(|_| thread_rng().next_u64())
                            .insert(pool.clone());
                        csv_source::<_, PersonStudyAtRecord>(scope, idx, &path)
                            .exchange(|_| thread_rng().next_u64())
                            .insert(pool.clone());
                        csv_source::<_, PersonWorkAtRecord>(scope, idx, &path)
                            .exchange(|_| thread_rng().next_u64())
                            .insert(pool.clone());
                        csv_source::<_, TagHasTypeRecord>(scope, idx, &path)
                            .exchange(|_| thread_rng().next_u64())
                            .insert(pool.clone());
                        csv_source::<_, TagClassIsSubclassOfRecord>(scope, idx, &path)
                            .exchange(|_| thread_rng().next_u64())
                            .insert(pool.clone());
                        csv_source::<_, PlaceIsPartOfRecord>(scope, idx, &path)
                            .exchange(|_| thread_rng().next_u64())
                            .insert(pool.clone());
                    });
                },
            )
            .unwrap();
            eprintln!("Done inserting relation records!")
        }
    }

    if ARGS.streams {
        let pool = pool.clone();
        let path = ARGS.path.join("streams/");

        {
            let connection = pool.get().unwrap();

            // Drop relations
            diesel::delete(post::table).execute(&connection).unwrap();
            diesel::delete(comment::table).execute(&connection).unwrap();
            diesel::delete(like_::table).execute(&connection).unwrap();
        }

        eprintln!("Inserting stream records!");
        timely::execute(timely::Configuration::Thread, move |worker| {
            // timely::execute(timely::Configuration::Process(num_cpus::get()), move |worker| {
            let idx = worker.index();
            let ctx = Context::new();

            worker.dataflow(|scope| {
                let (posts, comments, likes) = csv_stream_source(scope, idx, &path);

                posts
                    .exchange(|record| record.timestamp() as u64)
                    .bounded_delay(ARGS.delay)
                    .insert(pool.clone())
                    .publish(&ctx);
                comments
                    .exchange(|record| record.timestamp() as u64)
                    .bounded_delay(ARGS.delay)
                    .insert(pool.clone())
                    .publish(&ctx);
                likes
                    .exchange(|record| record.timestamp() as u64)
                    .bounded_delay(ARGS.delay)
                    .insert(pool.clone())
                    .publish(&ctx);
            });
        })
        .unwrap();
        eprintln!("Done inserting stream records!");
    }
}
