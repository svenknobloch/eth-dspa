use chrono::{DateTime, Duration, Utc};
use diesel::prelude::*;
use diesel::{Identifiable, Insertable, PgConnection};
use serde_derive::{Deserialize, Serialize};

use crate::records::{PostRecord, Record, StreamRecord, TableRecord};
use crate::schema::{like_ as like, post};
use crate::Topic;

#[derive(
    Clone, Debug, Serialize, Deserialize, Insertable, Identifiable, Associations, Queryable,
)]
#[primary_key(person_id, post_id)]
#[belongs_to(PostRecord, foreign_key = "post_id")]
#[serde(rename_all = "camelCase")]
#[table_name = "like"]
pub struct LikeRecord {
    #[serde(rename = "Person.id")]
    pub person_id: i32,
    #[serde(rename = "Post.id")]
    pub post_id: i32,
    pub creation_date: DateTime<Utc>,
}

impl LikeRecord {
    pub fn root(&self, connection: &PgConnection) -> Option<PostRecord> {
        post::table
            .filter(post::id.eq(&self.post_id))
            .first::<PostRecord>(connection)
            .ok()
    }
}

impl Record for LikeRecord {
    const FILENAME: &'static str = "likes_event_stream.csv";
}

impl TableRecord for LikeRecord {
    type Table = like::table;

    fn table() -> Self::Table {
        like::table
    }
}

impl StreamRecord for LikeRecord {
    const TOPIC: Topic = Topic::Like;

    fn id(&self) -> Option<i32> {
        None
    }

    fn timestamp(&self) -> i64 {
        self.creation_date.timestamp()
    }

    fn add_timestamp(&mut self, seconds: i64) {
        self.creation_date = self.creation_date + Duration::seconds(seconds);
    }

    fn ordered(&self, connection: &PgConnection) -> bool {
        post::table
            .filter(post::id.eq(self.post_id))
            .first::<PostRecord>(connection)
            .is_ok()
    }
}
