use std::fmt;

use chrono::{DateTime, Duration, Utc};
use diesel::{Identifiable, Insertable, PgConnection};
use regex::Regex;
use serde::de::{self, Visitor};
use serde::{Deserializer, Serializer};
use serde_derive::{Deserialize, Serialize};

use crate::records::{Record, StreamRecord, TableRecord};
use crate::schema::post;
use crate::Topic;

mod tags {
    use super::*;

    struct TagsVisitor;

    impl<'de> Visitor<'de> for TagsVisitor {
        type Value = Vec<i32>;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("An array of tags of the form: \"[tag(, tag)*]\"")
        }

        fn visit_str<E: de::Error>(self, v: &str) -> Result<Self::Value, E> {
            if v.is_empty() {
                Ok(Default::default())
            } else {
                Regex::new(r"\[([0-9]*)(?:, ([0-9]*))*\]")
                    .unwrap()
                    .captures(v)
                    .ok_or_else(|| E::custom("Failed to match tags regex"))?
                    .iter()
                    .skip(1)
                    .filter_map(|x| x)
                    .map(|c| c.as_str().parse::<i32>().map_err(E::custom))
                    .collect::<Result<_, _>>()
            }
        }

        fn visit_string<E: de::Error>(self, v: String) -> Result<Self::Value, E> {
            self.visit_str(&v)
        }
    }

    pub fn serialize<S>(data: &Vec<i32>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&if data.is_empty() {
            String::new()
        } else {
            format!(
                "[{}]",
                data.iter()
                    .map(ToString::to_string)
                    .collect::<Vec<_>>()
                    .join(", ")
            )
        })
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Vec<i32>, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_str(TagsVisitor)
    }
}

#[derive(
    Clone, Debug, Serialize, Deserialize, Insertable, Identifiable, Associations, Queryable,
)]
#[serde(rename_all = "camelCase")]
#[table_name = "post"]
pub struct PostRecord {
    pub id: i32,
    pub person_id: i32,
    pub creation_date: DateTime<Utc>,
    pub image_file: Option<String>,
    #[serde(rename = "locationIP")]
    pub location_ip: String,
    pub browser_used: String,
    pub language: Option<String>,
    pub content: Option<String>,
    #[serde(with = "tags")]
    pub tags: Vec<i32>,
    pub forum_id: i32,
    pub place_id: i32,
}

impl Record for PostRecord {
    const FILENAME: &'static str = "post_event_stream.csv";
}

impl TableRecord for PostRecord {
    type Table = post::table;

    fn table() -> Self::Table {
        post::table
    }
}

impl StreamRecord for PostRecord {
    const TOPIC: Topic = Topic::Post;

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
        true
    }
}
