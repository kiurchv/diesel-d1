// Since D1 is a SQlite-compatible DB, we have borrowed time/date/timestamp
// formats, conversion functions and base impls of the FromSql/ToSql traits
// from the upstream diesel::sqlite::types::date_and_time::time module

extern crate time;

use diesel::{
    deserialize::{self, FromSql},
    serialize::{self, IsNull, Output, ToSql},
    sql_types::{Date, Time, Timestamp},
};

use crate::{backend::D1Backend, value::D1Value};

use self::time::{
    error::ComponentRange, macros::format_description, Date as NaiveDate, OffsetDateTime,
    PrimitiveDateTime, Time as NaiveTime, UtcOffset,
};

// the non-deprecated variant does not exist in our minimal supported version
#[allow(deprecated)]
use self::time::format_description::FormatItem;

// the non-deprecated variant does not exist in our minimal supported version
#[allow(deprecated)]
const DATE_FORMAT: &[FormatItem<'_>] = format_description!("[year]-[month]-[day]");

// the non-deprecated variant does not exist in our minimal supported version
#[allow(deprecated)]
const ENCODE_TIME_FORMAT_WHOLE_SECOND: &[FormatItem<'_>] =
    format_description!("[hour]:[minute]:[second]");

// the non-deprecated variant does not exist in our minimal supported version
#[allow(deprecated)]
const ENCODE_TIME_FORMAT_SUBSECOND: &[FormatItem<'_>] =
    format_description!("[hour]:[minute]:[second].[subsecond]");

// the non-deprecated variant does not exist in our minimal supported version
#[allow(deprecated)]
const TIME_FORMATS: [&[FormatItem<'_>]; 9] = [
    // Most likely formats
    format_description!("[hour]:[minute]:[second].[subsecond]"),
    format_description!("[hour]:[minute]:[second]"),
    // All other valid formats in order of increasing specificity
    format_description!("[hour]:[minute]"),
    format_description!("[hour]:[minute]Z"),
    format_description!("[hour]:[minute][offset_hour sign:mandatory]:[offset_minute]"),
    format_description!("[hour]:[minute]:[second]Z"),
    format_description!("[hour]:[minute]:[second][offset_hour sign:mandatory]:[offset_minute]"),
    format_description!("[hour]:[minute]:[second].[subsecond]Z"),
    format_description!(
        "[hour]:[minute]:[second].[subsecond][offset_hour sign:mandatory]:[offset_minute]"
    ),
];

// the non-deprecated variant does not exist in our minimal supported version
#[allow(deprecated)]
const ENCODE_PRIMITIVE_DATETIME_FORMAT_WHOLE_SECOND: &[FormatItem<'_>] =
    format_description!("[year]-[month]-[day] [hour]:[minute]:[second]");

// the non-deprecated variant does not exist in our minimal supported version
#[allow(deprecated)]
const ENCODE_PRIMITIVE_DATETIME_FORMAT_SUBSECOND: &[FormatItem<'_>] =
    format_description!("[year]-[month]-[day] [hour]:[minute]:[second].[subsecond]");

// the non-deprecated variant does not exist in our minimal supported version
#[allow(deprecated)]
const PRIMITIVE_DATETIME_FORMATS: [&[FormatItem<'_>]; 18] = [
    // Most likely formats
    format_description!("[year]-[month]-[day] [hour]:[minute]:[second].[subsecond]"),
    format_description!("[year]-[month]-[day] [hour]:[minute]:[second].[subsecond][offset_hour sign:mandatory]:[offset_minute]"),
    format_description!("[year]-[month]-[day] [hour]:[minute]:[second]"),
    format_description!("[year]-[month]-[day] [hour]:[minute]:[second][offset_hour sign:mandatory]:[offset_minute]"),
    // All other formats in order of increasing specificity
    format_description!("[year]-[month]-[day] [hour]:[minute]"),
    format_description!("[year]-[month]-[day] [hour]:[minute]Z"),
    format_description!("[year]-[month]-[day] [hour]:[minute][offset_hour sign:mandatory]:[offset_minute]"),
    format_description!("[year]-[month]-[day] [hour]:[minute]:[second]Z"),
    format_description!("[year]-[month]-[day] [hour]:[minute]:[second].[subsecond]Z"),
    format_description!("[year]-[month]-[day]T[hour]:[minute]"),
    format_description!("[year]-[month]-[day]T[hour]:[minute]Z"),
    format_description!("[year]-[month]-[day]T[hour]:[minute][offset_hour sign:mandatory]:[offset_minute]"),
    format_description!("[year]-[month]-[day]T[hour]:[minute]:[second]"),
    format_description!("[year]-[month]-[day]T[hour]:[minute]:[second]Z"),
    format_description!("[year]-[month]-[day]T[hour]:[minute]:[second][offset_hour sign:mandatory]:[offset_minute]"),
    format_description!("[year]-[month]-[day]T[hour]:[minute]:[second].[subsecond]"),
    format_description!("[year]-[month]-[day]T[hour]:[minute]:[second].[subsecond]Z"),
    format_description!("[year]-[month]-[day]T[hour]:[minute]:[second].[subsecond][offset_hour sign:mandatory]:[offset_minute]"),
];

fn naive_utc(dt: OffsetDateTime) -> PrimitiveDateTime {
    let dt = dt.to_offset(UtcOffset::UTC);
    PrimitiveDateTime::new(dt.date(), dt.time())
}

fn parse_julian(julian_days: f64) -> Result<PrimitiveDateTime, ComponentRange> {
    const EPOCH_IN_JULIAN_DAYS: f64 = 2_440_587.5;
    const SECONDS_IN_DAY: f64 = 86400.0;
    let timestamp = (julian_days - EPOCH_IN_JULIAN_DAYS) * SECONDS_IN_DAY;
    #[allow(clippy::cast_possible_truncation)] // we multiply by 1E9 to prevent that
    OffsetDateTime::from_unix_timestamp_nanos((timestamp * 1E9) as i128).map(naive_utc)
}

// Date

impl FromSql<Date, D1Backend> for NaiveDate {
    fn from_sql(value: D1Value) -> deserialize::Result<Self> {
        let text = value.read_string();
        Self::parse(&text, DATE_FORMAT).map_err(Into::into)
    }
}

impl ToSql<Date, D1Backend> for NaiveDate {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, D1Backend>) -> serialize::Result {
        out.set_value(self.format(DATE_FORMAT).map_err(|err| err.to_string())?);
        Ok(IsNull::No)
    }
}

// Time

impl FromSql<Time, D1Backend> for NaiveTime {
    fn from_sql(value: D1Value) -> deserialize::Result<Self> {
        let text = value.read_string();

        for format in TIME_FORMATS {
            if let Ok(time) = Self::parse(&text, format) {
                return Ok(time);
            }
        }

        Err(format!("Invalid time {text}").into())
    }
}

impl ToSql<Time, D1Backend> for NaiveTime {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, D1Backend>) -> serialize::Result {
        let format = if self.microsecond() == 0 {
            ENCODE_TIME_FORMAT_WHOLE_SECOND
        } else {
            ENCODE_TIME_FORMAT_SUBSECOND
        };
        out.set_value(self.format(format).map_err(|err| err.to_string())?);
        Ok(IsNull::No)
    }
}

// Timestamp

impl FromSql<Timestamp, D1Backend> for PrimitiveDateTime {
    fn from_sql(value: D1Value) -> deserialize::Result<Self> {
        let text = value.read_string();

        for format in PRIMITIVE_DATETIME_FORMATS {
            if let Ok(dt) = Self::parse(&text, format) {
                return Ok(dt);
            }
        }

        if let Ok(julian_days) = text.parse::<f64>() {
            if let Ok(timestamp) = parse_julian(julian_days) {
                return Ok(timestamp);
            }
        }

        Err(format!("Invalid datetime {text}").into())
    }
}

impl ToSql<Timestamp, D1Backend> for PrimitiveDateTime {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, D1Backend>) -> serialize::Result {
        let format = if self.nanosecond() == 0 {
            ENCODE_PRIMITIVE_DATETIME_FORMAT_WHOLE_SECOND
        } else {
            ENCODE_PRIMITIVE_DATETIME_FORMAT_SUBSECOND
        };
        out.set_value(self.format(format).map_err(|err| err.to_string())?);
        Ok(IsNull::No)
    }
}
