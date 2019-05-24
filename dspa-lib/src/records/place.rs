use diesel::Insertable;
use serde_derive::{Deserialize, Serialize};

use crate::records::{Record, TableRecord};
use crate::schema::{place, place_is_part_of};

#[derive(Clone, Debug, Serialize, Deserialize, Insertable)]
#[table_name = "place_is_part_of"]
pub struct PlaceIsPartOfRecord {
    #[serde(rename = "Place.id")]
    place_id: i32,
    #[serde(rename = "Parent.id")]
    parent_id: i32,
}

impl Record for PlaceIsPartOfRecord {
    const FILENAME: &'static str = "place_isPartOf_place.csv";
}

impl TableRecord for PlaceIsPartOfRecord {
    type Table = place_is_part_of::table;

    fn table() -> Self::Table {
        place_is_part_of::table
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, Insertable)]
#[table_name = "place"]
pub struct PlaceRecord {
    id: i32,
    name: String,
    url: String,
    #[serde(rename = "type")]
    type_: String,
}

impl Record for PlaceRecord {
    const FILENAME: &'static str = "place.csv";
}

impl TableRecord for PlaceRecord {
    type Table = place::table;

    fn table() -> Self::Table {
        place::table
    }
}
