#[cfg(feature = "chrono")]
mod chrono;
#[cfg(feature = "time")]
mod time;

use diesel::{
    deserialize::{self, FromSql},
    serialize::{self, Output, ToSql},
    sql_types::{self, HasSqlType},
};

use crate::{
    backend::{D1Backend, D1Type},
    value::D1Value,
};

// Date

impl HasSqlType<sql_types::Date> for D1Backend {
    fn metadata(_lookup: &mut ()) -> D1Type {
        D1Type::Text
    }
}

impl FromSql<sql_types::Date, D1Backend> for String {
    fn from_sql(value: D1Value) -> deserialize::Result<Self> {
        FromSql::<sql_types::Text, D1Backend>::from_sql(value)
    }
}

impl ToSql<sql_types::Date, D1Backend> for String {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, D1Backend>) -> serialize::Result {
        ToSql::<sql_types::Text, D1Backend>::to_sql(self, out)
    }
}

// Time

impl HasSqlType<sql_types::Time> for D1Backend {
    fn metadata(_lookup: &mut ()) -> D1Type {
        D1Type::Text
    }
}

impl FromSql<sql_types::Time, D1Backend> for String {
    fn from_sql(value: D1Value) -> deserialize::Result<Self> {
        FromSql::<sql_types::Text, D1Backend>::from_sql(value)
    }
}

impl ToSql<sql_types::Time, D1Backend> for String {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, D1Backend>) -> serialize::Result {
        ToSql::<sql_types::Text, D1Backend>::to_sql(self, out)
    }
}

// Timestamp

impl HasSqlType<sql_types::Timestamp> for D1Backend {
    fn metadata(_lookup: &mut ()) -> D1Type {
        D1Type::Text
    }
}

impl FromSql<sql_types::Timestamp, D1Backend> for String {
    fn from_sql(value: D1Value) -> deserialize::Result<Self> {
        FromSql::<sql_types::Text, D1Backend>::from_sql(value)
    }
}

impl ToSql<sql_types::Timestamp, D1Backend> for String {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, D1Backend>) -> serialize::Result {
        ToSql::<sql_types::Text, D1Backend>::to_sql(self, out)
    }
}
