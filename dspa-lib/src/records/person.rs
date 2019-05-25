use chrono::{DateTime, NaiveDate, Utc};
use diesel::Insertable;
use serde_derive::{Deserialize, Serialize};

use crate::schema::{
    person, person_email, person_has_interest, person_is_located_in, person_knows, person_speaks,
    person_study_at, person_work_at,
};

use crate::records::{Record, TableRecord};

#[derive(Clone, Debug, Serialize, Deserialize, Insertable)]
#[table_name = "person_email"]
pub struct PersonEmailRecord {
    #[serde(rename = "Person.id")]
    person_id: i32,
    email: String,
}

impl Record for PersonEmailRecord {
    const FILENAME: &'static str = "person_email_emailaddress.csv";
}

impl TableRecord for PersonEmailRecord {
    type Table = person_email::table;

    fn table() -> Self::Table {
        person_email::table
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, Insertable)]
#[table_name = "person_has_interest"]
pub struct PersonHasInterestRecord {
    #[serde(rename = "Person.id")]
    person_id: i32,
    #[serde(rename = "Tag.id")]
    tag_id: i32,
}

impl Record for PersonHasInterestRecord {
    const FILENAME: &'static str = "person_hasInterest_tag.csv";
}

impl TableRecord for PersonHasInterestRecord {
    type Table = person_has_interest::table;

    fn table() -> Self::Table {
        person_has_interest::table
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, Insertable)]
#[table_name = "person_is_located_in"]
pub struct PersonIsLocatedInRecord {
    #[serde(rename = "Person.id")]
    person_id: i32,
    #[serde(rename = "Place.id")]
    place_id: i32,
}

impl Record for PersonIsLocatedInRecord {
    const FILENAME: &'static str = "person_isLocatedIn_place.csv";
}

impl TableRecord for PersonIsLocatedInRecord {
    type Table = person_is_located_in::table;

    fn table() -> Self::Table {
        person_is_located_in::table
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, Insertable, Queryable)]
#[table_name = "person_knows"]
pub struct PersonKnowsRecord {
    #[serde(rename = "Person.id")]
    pub person_id: i32,
    #[serde(rename = "Acquaintance.id")]
    pub acquaintance_id: i32,
}

impl Record for PersonKnowsRecord {
    const FILENAME: &'static str = "person_knows_person.csv";
}

impl TableRecord for PersonKnowsRecord {
    type Table = person_knows::table;

    fn table() -> Self::Table {
        person_knows::table
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, Insertable)]
#[table_name = "person_speaks"]
pub struct PersonSpeaksRecord {
    #[serde(rename = "Person.id")]
    person_id: i32,
    language: String,
}

impl Record for PersonSpeaksRecord {
    const FILENAME: &'static str = "person_speaks_language.csv";
}

impl TableRecord for PersonSpeaksRecord {
    type Table = person_speaks::table;

    fn table() -> Self::Table {
        person_speaks::table
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, Insertable)]
#[table_name = "person_study_at"]
pub struct PersonStudyAtRecord {
    #[serde(rename = "Person.id")]
    person_id: i32,
    #[serde(rename = "Organisation.id")]
    organization_id: i32,
    #[serde(rename = "classYear")]
    class_year: i32,
}

impl Record for PersonStudyAtRecord {
    const FILENAME: &'static str = "person_studyAt_organisation.csv";
}

impl TableRecord for PersonStudyAtRecord {
    type Table = person_study_at::table;

    fn table() -> Self::Table {
        person_study_at::table
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, Insertable)]
#[table_name = "person_work_at"]
pub struct PersonWorkAtRecord {
    #[serde(rename = "Person.id")]
    person_id: i32,
    #[serde(rename = "Organisation.id")]
    organization_id: i32,
    #[serde(rename = "workFrom")]
    work_from: i32,
}

impl Record for PersonWorkAtRecord {
    const FILENAME: &'static str = "person_workAt_organisation.csv";
}

impl TableRecord for PersonWorkAtRecord {
    type Table = person_work_at::table;

    fn table() -> Self::Table {
        person_work_at::table
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, Insertable, Queryable, Identifiable)]
#[serde(rename_all = "camelCase")]
#[table_name = "person"]
pub struct PersonRecord {
    id: i32,
    first_name: String,
    last_name: String,
    gender: String,
    birthday: NaiveDate,
    creation_date: DateTime<Utc>,
    #[serde(rename = "locationIP")]
    location_ip: String,
    browser_used: String,
}

impl Record for PersonRecord {
    const FILENAME: &'static str = "person.csv";
}

impl TableRecord for PersonRecord {
    type Table = person::table;

    fn table() -> Self::Table {
        person::table
    }
}
