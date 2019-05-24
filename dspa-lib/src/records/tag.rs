use diesel::Insertable;
use serde_derive::{Deserialize, Serialize};

use crate::records::{Record, TableRecord};
use crate::schema::{tag, tag_has_type};

#[derive(Clone, Debug, Serialize, Deserialize, Insertable)]
#[table_name = "tag_has_type"]
pub struct TagHasTypeRecord {
    #[serde(rename = "Tag.id")]
    tag_id: i32,
    #[serde(rename = "TagClass.id")]
    tag_class_id: i32,
}

impl Record for TagHasTypeRecord {
    const FILENAME: &'static str = "tag_hasType_tagclass.csv";
}

impl TableRecord for TagHasTypeRecord {
    type Table = tag_has_type::table;

    fn table() -> Self::Table {
        tag_has_type::table
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, Insertable)]
#[table_name = "tag"]
pub struct TagRecord {
    id: i32,
    name: String,
    url: String,
}

impl Record for TagRecord {
    const FILENAME: &'static str = "tag.csv";
}

impl TableRecord for TagRecord {
    type Table = tag::table;

    fn table() -> Self::Table {
        tag::table
    }
}
