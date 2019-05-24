use std::sync::Arc;

use diesel::associations::HasTable;
use diesel::insertable::{BatchInsert, CanInsertInSingleQuery, OwnedBatchInsert};
use diesel::pg::Pg;
use diesel::prelude::*;
use diesel::query_builder::IntoUpdateTarget;
use diesel::query_builder::{InsertStatement, QueryId};
use diesel::query_builder::{QueryFragment, UndecoratedInsertRecord};
use diesel::query_dsl::methods::ExecuteDsl;
use diesel::query_dsl::RunQueryDsl;
use diesel::r2d2::ConnectionManager;
use diesel::{Insertable, PgConnection, QuerySource, Table};
use r2d2::Pool;
use timely::dataflow::operators::Inspect;
use timely::dataflow::{Scope, Stream};

use dspa_lib::records::TableRecord;

pub trait Insert<G, D>
where
    G: Scope<Timestamp = u64>,
{
    fn insert(&self, pool: Arc<Pool<ConnectionManager<PgConnection>>>) -> Stream<G, D>;
}

impl<'a, G, D> Insert<G, D> for Stream<G, D>
where
    G: Scope<Timestamp = u64>,
    D: TableRecord + Insertable<<D as TableRecord>::Table>,
    <<D as TableRecord>::Table as QuerySource>::FromClause: QueryFragment<Pg>,
    <D as Insertable<<D as TableRecord>::Table>>::Values: QueryFragment<Pg>,
    <D as Insertable<<D as TableRecord>::Table>>::Values: CanInsertInSingleQuery<Pg>,
{
    fn insert(&self, pool: Arc<Pool<ConnectionManager<PgConnection>>>) -> Stream<G, D> {
        self.inspect(move |record| {
            let connection = pool.get().unwrap();

            diesel::insert_into(D::table())
                .values(record.clone())
                .execute(&connection)
                .unwrap();
        })
    }
}
