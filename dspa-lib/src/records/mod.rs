use std::fmt::Debug;

use diesel::insertable::CanInsertInSingleQuery;
use diesel::pg::Pg;
use diesel::query_builder::InsertStatement;
use diesel::query_builder::IntoUpdateTarget;
use diesel::query_builder::QueryFragment;
use diesel::query_dsl::methods::ExecuteDsl;
use diesel::query_dsl::RunQueryDsl;
use diesel::{Insertable, PgConnection, Table};
use serde::de::DeserializeOwned;
use serde::Serialize;
use timely::ExchangeData;

use crate::Topic;

mod comment;
mod forum;
mod like;
mod organization;
mod person;
mod place;
mod post;
mod tag;
mod tag_class;

pub use comment::*;
pub use forum::*;
pub use like::*;
pub use organization::*;
pub use person::*;
pub use place::*;
pub use post::*;
pub use tag::*;
pub use tag_class::*;

pub trait Record
where
    Self: DeserializeOwned + Debug + ExchangeData,
{
    const FILENAME: &'static str;
}

pub trait FilteredRecord
where
    Self: Record,
{
    const FILENAME: &'static str;

    type FilterData: DeserializeOwned + Debug + Into<i32>;

    fn id(&self) -> i32;
}

pub trait TableRecord
where
    Self: Record,
{
    type Table: diesel::Table + Copy;

    fn table() -> Self::Table;
}

pub trait StreamRecord
where
    Self: TableRecord + Serialize + DeserializeOwned,
{
    const TOPIC: Topic;

    fn id(&self) -> Option<i32>;
    fn timestamp(&self) -> i64;
    fn add_timestamp(&mut self, value: i64);

    fn ordered(&self, connection: &PgConnection) -> bool;
}
