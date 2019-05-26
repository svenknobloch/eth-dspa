#![feature(atomic_min_max)]

#[macro_use]
extern crate diesel;
use serde_derive::{Deserialize, Serialize};

use records::{CommentRecord, LikeRecord, PostRecord, StreamRecord};

pub mod operators;
pub mod records;
pub mod schema;

pub const SOURCE_SOCKET: &str = "/tmp/dspa/source";
pub const DATA_SOCKET: &str = "/tmp/dspa/data";

pub const DATABASE_URL: &str = "postgres://root@localhost/dspa";

pub const SOCKET_TIMEOUT: i32 = 100;

// Max delay of one day
pub const MAX_DELAY: u64 = 60 * 60 * 24;

#[derive(Copy, Clone, Debug)]
pub enum Topic {
    Post,
    Comment,
    Like,
    EOS,
}

impl ToString for Topic {
    fn to_string(&self) -> String {
        match self {
            Topic::Post => "post".to_owned(),
            Topic::Comment => "comment".to_owned(),
            Topic::Like => "like".to_owned(),
            Topic::EOS => "eos".to_owned(),
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum StreamEvent {
    Post(PostRecord),
    Comment(CommentRecord),
    Like(LikeRecord),
}

impl StreamEvent {
    pub fn id(&self) -> Option<i32> {
        match self {
            StreamEvent::Post(record) => record.id(),
            StreamEvent::Comment(record) => record.id(),
            StreamEvent::Like(record) => record.id(),
        }
    }

    pub fn timestamp(&self) -> i64 {
        match self {
            StreamEvent::Post(record) => record.timestamp(),
            StreamEvent::Comment(record) => record.timestamp(),
            StreamEvent::Like(record) => record.timestamp(),
        }
    }
}

impl From<PostRecord> for StreamEvent {
    fn from(record: PostRecord) -> Self {
        StreamEvent::Post(record)
    }
}

impl From<CommentRecord> for StreamEvent {
    fn from(record: CommentRecord) -> Self {
        StreamEvent::Comment(record)
    }
}

impl From<LikeRecord> for StreamEvent {
    fn from(record: LikeRecord) -> Self {
        StreamEvent::Like(record)
    }
}
