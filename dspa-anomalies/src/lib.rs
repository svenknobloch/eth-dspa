#[macro_use]
extern crate lazy_static;

use serde_derive::{Serialize, Deserialize};
use structopt::StructOpt;

use dspa_lib::records::{PostRecord, CommentRecord, LikeRecord};

pub mod operators;


lazy_static! {
    pub static ref ARGS: Args = Args::from_args();
}

#[derive(Debug, StructOpt)]
#[structopt(name = "dspa-anomalies")]
pub struct Args {
    #[structopt(long = "threshold")]
    pub threshold: f32,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum AnomalyEvent {
    Post(PostRecord),
    Comment(CommentRecord),
    Like(LikeRecord),
}

impl AnomalyEvent {
    
}