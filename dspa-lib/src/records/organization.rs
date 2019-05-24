use diesel::Insertable;
use serde_derive::{Deserialize, Serialize};

use crate::records::{Record, TableRecord};
use crate::schema::{organization, organization_is_located_in};

#[derive(Clone, Debug, Serialize, Deserialize, Insertable)]
#[table_name = "organization_is_located_in"]
pub struct OrganizationIsLocatedInRecord {
    #[serde(rename = "Organisation.id")]
    organization_id: i32,
    #[serde(rename = "Place.id")]
    place_id: i32,
}

impl Record for OrganizationIsLocatedInRecord {
    const FILENAME: &'static str = "organisation_isLocatedIn_place.csv";
}

impl TableRecord for OrganizationIsLocatedInRecord {
    type Table = organization_is_located_in::table;

    fn table() -> Self::Table {
        organization_is_located_in::table
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, Insertable)]
#[table_name = "organization"]
pub struct OrganizationRecord {
    id: i32,
    #[serde(rename = "type")]
    type_: String,
    name: String,
    url: String,
}

impl Record for OrganizationRecord {
    const FILENAME: &'static str = "organisation.csv";
}

impl TableRecord for OrganizationRecord {
    type Table = organization::table;

    fn table() -> Self::Table {
        organization::table
    }
}
