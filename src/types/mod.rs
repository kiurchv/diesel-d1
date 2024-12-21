mod date_and_time;

use diesel::{
    deserialize::{self, FromSql},
    serialize::{self, IsNull, Output, ToSql},
    sql_types::{self, HasSqlType},
};
use js_sys::Uint8Array;

use crate::{
    backend::{D1Backend, D1Type},
    value::D1Value,
};

// Boolean
impl HasSqlType<sql_types::Bool> for D1Backend {
    fn metadata(_lookup: &mut ()) -> D1Type {
        D1Type::Integer
    }
}

impl FromSql<sql_types::Bool, D1Backend> for bool {
    fn from_sql(value: D1Value) -> deserialize::Result<Self> {
        let bool_number = value.read_number();
        if !(bool_number == 0.0 || bool_number == 1.0) {
            panic!("this shouldn't happen bool is not a bool");
        }
        Ok(bool_number != 0.0)
    }
}

impl ToSql<sql_types::Bool, D1Backend> for bool {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, D1Backend>) -> serialize::Result {
        out.set_value(*self as i64 as f64);
        Ok(IsNull::No)
    }
}

// SMALL INT

impl HasSqlType<sql_types::SmallInt> for D1Backend {
    fn metadata(_lookup: &mut ()) -> D1Type {
        D1Type::Integer
    }
}

impl FromSql<sql_types::SmallInt, D1Backend> for i16 {
    fn from_sql(value: D1Value) -> deserialize::Result<Self> {
        let text = value.read_number();
        Ok(text as i16)
    }
}

impl ToSql<sql_types::SmallInt, D1Backend> for i16 {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, D1Backend>) -> serialize::Result {
        out.set_value(*self);
        Ok(IsNull::No)
    }
}

// ------

// Int

impl HasSqlType<sql_types::Integer> for D1Backend {
    fn metadata(_lookup: &mut ()) -> D1Type {
        D1Type::Integer
    }
}

impl FromSql<sql_types::Integer, D1Backend> for i32 {
    fn from_sql(value: D1Value) -> deserialize::Result<Self> {
        let text = value.read_number();
        Ok(text as i32)
    }
}

impl ToSql<sql_types::Integer, D1Backend> for i32 {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, D1Backend>) -> serialize::Result {
        out.set_value(*self);
        Ok(IsNull::No)
    }
}

// ------

// BigInt

impl HasSqlType<sql_types::BigInt> for D1Backend {
    fn metadata(_lookup: &mut ()) -> D1Type {
        D1Type::Integer
    }
}

impl FromSql<sql_types::BigInt, D1Backend> for i64 {
    fn from_sql(value: D1Value) -> deserialize::Result<Self> {
        let text = value.read_number();
        Ok(text as i64)
    }
}

impl ToSql<sql_types::BigInt, D1Backend> for i64 {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, D1Backend>) -> serialize::Result {
        out.set_value(*self as f64);
        Ok(IsNull::No)
    }
}

// ------

// Float

impl HasSqlType<sql_types::Float> for D1Backend {
    fn metadata(_lookup: &mut ()) -> D1Type {
        D1Type::Double
    }
}

impl FromSql<sql_types::Float, D1Backend> for f32 {
    fn from_sql(value: D1Value) -> deserialize::Result<Self> {
        let text = value.read_number();
        Ok(text as f32)
    }
}

impl ToSql<sql_types::Float, D1Backend> for f32 {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, D1Backend>) -> serialize::Result {
        out.set_value(*self);
        Ok(IsNull::No)
    }
}

// ------

// Double

impl HasSqlType<sql_types::Double> for D1Backend {
    fn metadata(_lookup: &mut ()) -> D1Type {
        D1Type::Double
    }
}

impl FromSql<sql_types::Double, D1Backend> for f64 {
    fn from_sql(value: D1Value) -> deserialize::Result<Self> {
        let text = value.read_number();
        Ok(text)
    }
}

impl ToSql<sql_types::Double, D1Backend> for f64 {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, D1Backend>) -> serialize::Result {
        out.set_value(*self);
        Ok(IsNull::No)
    }
}

// ------

// Text

impl HasSqlType<sql_types::Text> for D1Backend {
    fn metadata(_lookup: &mut ()) -> D1Type {
        D1Type::Text
    }
}

impl FromSql<sql_types::Text, D1Backend> for String {
    fn from_sql(value: D1Value) -> deserialize::Result<Self> {
        let text = value.read_string();
        Ok(text)
    }
}

impl ToSql<sql_types::Text, D1Backend> for String {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, D1Backend>) -> serialize::Result {
        out.set_value(self);
        Ok(IsNull::No)
    }
}

// ------

// Blob

impl HasSqlType<sql_types::Binary> for D1Backend {
    fn metadata(_lookup: &mut ()) -> D1Type {
        D1Type::Binary
    }
}

impl FromSql<sql_types::Binary, D1Backend> for *const [u8] {
    fn from_sql(value: D1Value) -> deserialize::Result<Self> {
        let text = value.read_blob();
        Ok(text.as_slice() as *const [u8])
    }
}

impl ToSql<sql_types::Binary, D1Backend> for *const [u8] {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, D1Backend>) -> serialize::Result {
        // SAFETY: safe to do because we don't expect for buffer to change size, `as_ref` should always pass anyway
        let value = unsafe { js_sys::Uint8Array::new(&Uint8Array::view(self.as_ref().unwrap())) };
        out.set_value(value);
        Ok(IsNull::No)
    }
}
