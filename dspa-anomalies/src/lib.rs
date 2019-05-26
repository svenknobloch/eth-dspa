#[macro_use]
extern crate lazy_static;

use serde_derive::{Deserialize, Serialize};
use structopt::StructOpt;

use dspa_lib::records::{CommentRecord, LikeRecord, PostRecord};

pub mod operators;
pub mod statistics;

lazy_static! {
    pub static ref ARGS: Args = Args::from_args();
}

#[derive(Debug, StructOpt)]
#[structopt(name = "dspa-anomalies")]
pub struct Args {
    #[structopt(long = "threshold", default_value = "3")]
    pub threshold: f32,
    #[structopt(short = "s", long = "sample_size", default_value = "256")]
    pub samples: usize,
    #[structopt(long = "smoothing", default_value = "3")]
    pub alpha: f32,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum AnomalyEvent {
    Post(PostRecord),
    Comment(CommentRecord),
    Like(LikeRecord),
}

impl AnomalyEvent {}
