use chrono::{DateTime, Utc};
use diesel::Insertable;
use serde_derive::{Deserialize, Serialize};

use crate::records::{Record, TableRecord};
use crate::schema::{forum, forum_has_member, forum_has_moderator, forum_has_tag};

#[derive(Clone, Debug, Serialize, Deserialize, Insertable)]
#[table_name = "forum_has_member"]
pub struct ForumHasMemberRecord {
    #[serde(rename = "Forum.id")]
    forum_id: i32,
    #[serde(rename = "Person.id")]
    person_id: i32,
    #[serde(rename = "joinDate")]
    join_date: DateTime<Utc>,
}

impl Record for ForumHasMemberRecord {
    const FILENAME: &'static str = "forum_hasMember_person.csv";
}

impl TableRecord for ForumHasMemberRecord {
    type Table = forum_has_member::table;

    fn table() -> Self::Table {
        forum_has_member::table
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, Insertable)]
#[table_name = "forum_has_moderator"]
pub struct ForumHasModeratorRecord {
    #[serde(rename = "Forum.id")]
    forum_id: i32,
    #[serde(rename = "Person.id")]
    person_id: i32,
}

impl Record for ForumHasModeratorRecord {
    const FILENAME: &'static str = "forum_hasModerator_person.csv";
}

impl TableRecord for ForumHasModeratorRecord {
    type Table = forum_has_moderator::table;

    fn table() -> Self::Table {
        forum_has_moderator::table
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, Insertable)]
#[table_name = "forum_has_tag"]
pub struct ForumHasTagRecord {
    #[serde(rename = "Forum.id")]
    forum_id: i32,
    #[serde(rename = "Tag.id")]
    tag_id: i32,
}

impl Record for ForumHasTagRecord {
    const FILENAME: &'static str = "forum_hasTag_tag.csv";
}

impl TableRecord for ForumHasTagRecord {
    type Table = forum_has_tag::table;

    fn table() -> Self::Table {
        forum_has_tag::table
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, Insertable)]
#[serde(rename_all = "camelCase")]
#[table_name = "forum"]
pub struct ForumRecord {
    id: i32,
    title: String,
    creation_date: DateTime<Utc>,
}

impl Record for ForumRecord {
    const FILENAME: &'static str = "forum.csv";
}

impl TableRecord for ForumRecord {
    type Table = forum::table;

    fn table() -> Self::Table {
        forum::table
    }
}
