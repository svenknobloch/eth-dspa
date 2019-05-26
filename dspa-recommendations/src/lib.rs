#[macro_use]
extern crate lazy_static;

use serde_derive::{Deserialize, Serialize};
use structopt::StructOpt;

use dspa_lib::records::{CommentRecord, LikeRecord, PostRecord};

pub mod operators;

lazy_static! {
    pub static ref ARGS: Args = Args::from_args();
}

#[derive(Debug, StructOpt)]
#[structopt(name = "dspa-recommendations")]
pub struct Args {
    #[structopt(short = "u", long = "users")]
    pub users: Vec<i32>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum RecommendationEvent {
    Post(PostRecord),
    Comment(CommentRecord),
    Like(LikeRecord),
}

impl RecommendationEvent {
    pub fn user(&self) -> i32 {
        match self {
            RecommendationEvent::Post(record) => record.person_id,
            RecommendationEvent::Comment(record) => record.person_id,
            RecommendationEvent::Like(record) => record.person_id,
        }
    }
}
