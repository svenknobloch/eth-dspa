use std::collections::HashSet;
use std::iter::Iterator;

use regex::Regex;
use timely::dataflow::channels::pact::Pipeline;
use timely::dataflow::operators::Operator;
use timely::dataflow::{Scope, Stream};

use crate::statistics::OnlineStatistic;
use crate::AnomalyEvent;
use crate::ARGS;

fn unique_words_per_word(s: &str) -> f32 {
    lazy_static! {
        static ref REGEX: Regex = Regex::new(r#"[.!?\-,]"#).unwrap();
    }
    let mut count = 0;
    let regex = REGEX.replace_all(s, "");
    let unique = regex
        .split(' ')
        .inspect(|_| count += 1)
        .collect::<HashSet<_>>();
    (unique.len() as f32 + ARGS.alpha) / (count as f32 + ARGS.alpha)
}

pub trait Anomalies<G>
where
    G: Scope<Timestamp = u64>,
{
    fn anomalies(&self) -> Stream<G, (i32, String, f32)>;
}

impl<G> Anomalies<G> for Stream<G, AnomalyEvent>
where
    G: Scope<Timestamp = u64>,
{
    fn anomalies(&self) -> Stream<G, (i32, String, f32)> {
        self.unary(Pipeline, "Anomalies", |_, _| {
            // Post statistics
            let mut post_word_unique_stats = OnlineStatistic::new(ARGS.samples);
            let mut num_tags_stats = OnlineStatistic::new(ARGS.samples);

            // Comment statistics
            let mut comment_word_unique_stats = OnlineStatistic::new(ARGS.samples);

            move |input, output| {
                let mut vec = Vec::new();
                input.for_each(|cap, data| {
                    data.swap(&mut vec);

                    let mut session = output.session(&cap);
                    vec.drain(..).for_each(|event| {
                        match event {
                            AnomalyEvent::Post(record) => {
                                // Post Unique words
                                {
                                    if let Some(content) = &record.content {
                                        let n_unique = unique_words_per_word(content);

                                        if post_word_unique_stats.saturated() {
                                            if let Some(stddevs) = post_word_unique_stats
                                                .is_anomaly(ARGS.threshold, n_unique)
                                            {
                                                session.give((
                                                    record.person_id,
                                                    "Post - Unique Words".to_owned(),
                                                    stddevs,
                                                ));
                                            }
                                        }
                                        post_word_unique_stats.update(n_unique);
                                    }
                                }

                                // Post Number of Tags
                                {
                                    let n_tags = record.tags.len() as f32;

                                    if num_tags_stats.saturated() {
                                        if let Some(stddevs) =
                                            num_tags_stats.is_anomaly(ARGS.threshold, n_tags)
                                        {
                                            session.give((
                                                record.person_id,
                                                "Post - Number of Tags".to_owned(),
                                                stddevs,
                                            ));
                                        }
                                    }
                                    num_tags_stats.update(n_tags);
                                }
                            }
                            AnomalyEvent::Comment(record) => {
                                // Comment Unique Words
                                {
                                    let n_unique = unique_words_per_word(&record.content);

                                    if comment_word_unique_stats.saturated() {
                                        if let Some(stddevs) = comment_word_unique_stats
                                            .is_anomaly(ARGS.threshold, n_unique)
                                        {
                                            session.give((
                                                record.person_id,
                                                "Comment - Unique Words".to_owned(),
                                                stddevs,
                                            ));
                                        }
                                    }

                                    comment_word_unique_stats.update(n_unique);
                                }
                            }
                            _ => {}
                        }
                    });
                });
            }
        })
    }
}
