use std::collections::HashMap;
use std::convert::identity;

use chrono::{DateTime, Duration, Utc};
use diesel::prelude::*;
use diesel::{Identifiable, Insertable, PgConnection};
use either::Either;
use serde_derive::{Deserialize, Serialize};

use crate::records::{FilteredRecord, PostRecord, Record, StreamRecord, TableRecord};
use crate::schema::{comment, post};
use crate::Topic;

#[derive(Clone, Debug, Deserialize)]
pub struct CommentBlacklistRecord {
    comment_id: i32,
    reply_to_comment_id: i32,
    root_comment_id: i32,
}

impl Into<i32> for CommentBlacklistRecord {
    fn into(self) -> i32 {
        self.comment_id
    }
}

#[derive(
    Clone, Debug, Serialize, Deserialize, Insertable, Identifiable, Associations, Queryable,
)]
#[belongs_to(PostRecord, foreign_key = "reply_to_post_id")]
#[belongs_to(CommentRecord, foreign_key = "reply_to_comment_id")]
#[serde(rename_all = "camelCase")]
#[table_name = "comment"]
pub struct CommentRecord {
    pub id: i32,
    pub person_id: i32,
    pub creation_date: DateTime<Utc>,
    #[serde(rename = "locationIP")]
    pub location_ip: String,
    pub browser_used: String,
    pub content: String,
    #[serde(rename = "reply_to_postId")]
    pub reply_to_post_id: Option<i32>,
    #[serde(rename = "reply_to_commentId")]
    pub reply_to_comment_id: Option<i32>,
    pub place_id: i32,
}

impl CommentRecord {
    pub fn root(&self, connection: &PgConnection) -> Option<PostRecord> {
        match self.parent(connection) {
            Some(Either::Left(post)) => Some(post),
            Some(Either::Right(comment)) => comment.root(connection),
            None => None,
        }
    }

    pub fn parent(&self, connection: &PgConnection) -> Option<Either<PostRecord, CommentRecord>> {
        if let Some(comment_id) = self.reply_to_comment_id {
            comment::table
                .filter(comment::id.eq(&comment_id))
                .first::<CommentRecord>(connection)
                .ok()
                .map(|comment| Either::Right(comment))
        } else if let Some(post_id) = self.reply_to_post_id {
            post::table
                .filter(post::id.eq(&post_id))
                .first::<PostRecord>(connection)
                .ok()
                .map(|post| Either::Left(post))
        } else {
            // Comments must have a parent
            unreachable!();
        }
    }

    pub fn available_parent(
        &self,
        connection: &PgConnection,
    ) -> Option<Either<PostRecord, CommentRecord>> {
        // Check parent
        if let Some(parent) = self.parent(connection) {
            match parent {
                // If post, return
                Either::Left(post) => Some(Either::Left(post)),
                // If comment, try get ancestor
                Either::Right(comment) => {
                    match comment.available_parent(connection) {
                        // If ancestor, return ancestor
                        Some(ancestor) => Some(ancestor),
                        // Else, return self
                        None => Some(Either::Right(comment)),
                    }
                }
            }
        } else {
            None
        }
    }

    pub fn parent_id(&self) -> Either<i32, i32> {
        if let Some(id) = self.reply_to_post_id {
            Either::Left(id)
        } else if let Some(id) = self.reply_to_comment_id {
            Either::Right(id)
        } else {
            // Comments must have a parent
            unreachable!()
        }
    }

    pub fn hashmap(&self, connection: &PgConnection) -> HashMap<&'static str, i32> {
        let mut out: HashMap<&str, i32> = HashMap::new();
        out.insert("person_id", self.person_id);
        out.insert("post_id", self.root(connection).expect("autismo").id);
        out.insert("place_id", self.place_id);
        out
    }
}

impl Record for CommentRecord {
    const FILENAME: &'static str = "comment_event_stream.csv";
}

impl FilteredRecord for CommentRecord {
    const FILENAME: &'static str = "comment_blacklist.csv";

    type FilterData = CommentBlacklistRecord;

    fn id(&self) -> i32 {
        self.id
    }
}

impl TableRecord for CommentRecord {
    type Table = comment::table;

    fn table() -> Self::Table {
        comment::table
    }
}

impl StreamRecord for CommentRecord {
    const TOPIC: Topic = Topic::Comment;

    fn id(&self) -> Option<i32> {
        Some(self.id)
    }

    fn timestamp(&self) -> i64 {
        self.creation_date.timestamp()
    }

    fn add_timestamp(&mut self, seconds: i64) {
        self.creation_date = self.creation_date + Duration::seconds(seconds);
    }

    fn ordered(&self, connection: &PgConnection) -> bool {
        self.root(connection).is_some()
    }
}
