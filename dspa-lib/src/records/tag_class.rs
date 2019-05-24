use diesel::Insertable;
use serde_derive::{Deserialize, Serialize};

use crate::records::{Record, TableRecord};
use crate::schema::{tag_class, tag_class_is_subclass_of};

#[derive(Clone, Debug, Serialize, Deserialize, Insertable)]
#[table_name = "tag_class_is_subclass_of"]
pub struct TagClassIsSubclassOfRecord {
    #[serde(rename = "TagClass.id")]
    tag_class_id: i32,
    #[serde(rename = "Parent.id")]
    parent_id: i32,
}

impl Record for TagClassIsSubclassOfRecord {
    const FILENAME: &'static str = "tagclass_isSubclassOf_tagclass.csv";
}

impl TableRecord for TagClassIsSubclassOfRecord {
    type Table = tag_class_is_subclass_of::table;

    fn table() -> Self::Table {
        tag_class_is_subclass_of::table
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, Insertable)]
#[table_name = "tag_class"]
pub struct TagClassRecord {
    id: i32,
    name: String,
    url: String,
}

impl Record for TagClassRecord {
    const FILENAME: &'static str = "tagclass.csv";
}

impl TableRecord for TagClassRecord {
    type Table = tag_class::table;

    fn table() -> Self::Table {
        tag_class::table
    }
}
