// Since D1 is a SQlite-compatible DB, we have borrowed time/date/timestamp
// formats, conversion functions and base impls of the FromSql/ToSql traits
// from the upstream diesel::sqlite::types::date_and_time::chrono module

extern crate chrono;

use diesel::{
    deserialize::{self, FromSql},
    serialize::{self, IsNull, Output, ToSql},
    sql_types::{Date, Time, Timestamp},
};

use crate::{backend::D1Backend, value::D1Value};

use self::chrono::{NaiveDate, NaiveDateTime, NaiveTime};

const DATE_FORMAT: &str = "%F";

const ENCODE_TIME_FORMAT: &str = "%T%.f";

const TIME_FORMATS: [&str; 9] = [
    // Most likely formats
    "%T%.f", "%T", // All other valid formats in order of increasing specificity
    "%R", "%RZ", "%R%:z", "%TZ", "%T%:z", "%T%.fZ", "%T%.f%:z",
];

const ENCODE_NAIVE_DATETIME_FORMAT: &str = "%F %T%.f";

const NAIVE_DATETIME_FORMATS: [&str; 18] = [
    // Most likely formats
    "%F %T%.f",
    "%F %T%.f%:z",
    "%F %T",
    "%F %T%:z",
    // All other formats in order of increasing specificity
    "%F %R",
    "%F %RZ",
    "%F %R%:z",
    "%F %TZ",
    "%F %T%.fZ",
    "%FT%R",
    "%FT%RZ",
    "%FT%R%:z",
    "%FT%T",
    "%FT%TZ",
    "%FT%T%:z",
    "%FT%T%.f",
    "%FT%T%.fZ",
    "%FT%T%.f%:z",
];

fn parse_julian(julian_days: f64) -> Option<NaiveDateTime> {
    const EPOCH_IN_JULIAN_DAYS: f64 = 2_440_587.5;
    const SECONDS_IN_DAY: f64 = 86400.0;
    let timestamp = (julian_days - EPOCH_IN_JULIAN_DAYS) * SECONDS_IN_DAY;
    #[allow(clippy::cast_possible_truncation)] // we want to truncate
    let seconds = timestamp.trunc() as i64;
    // that's not true, `fract` is always > 0
    #[allow(clippy::cast_sign_loss, clippy::cast_possible_truncation)]
    let nanos = (timestamp.fract() * 1E9) as u32;
    #[allow(deprecated)] // otherwise we would need to bump our minimal chrono version
    NaiveDateTime::from_timestamp_opt(seconds, nanos)
}

// Date

impl FromSql<Date, D1Backend> for NaiveDate {
    fn from_sql(value: D1Value) -> deserialize::Result<Self> {
        let text = value.read_string();
        Self::parse_from_str(&text, DATE_FORMAT).map_err(Into::into)
    }
}

impl ToSql<Date, D1Backend> for NaiveDate {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, D1Backend>) -> serialize::Result {
        out.set_value(self.format(DATE_FORMAT).to_string());
        Ok(IsNull::No)
    }
}

// Time

impl FromSql<Time, D1Backend> for NaiveTime {
    fn from_sql(value: D1Value) -> deserialize::Result<Self> {
        let text = value.read_string();

        for format in TIME_FORMATS {
            if let Ok(time) = Self::parse_from_str(&text, format) {
                return Ok(time);
            }
        }

        Err(format!("Invalid time {text}").into())
    }
}

impl ToSql<Time, D1Backend> for NaiveTime {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, D1Backend>) -> serialize::Result {
        out.set_value(self.format(ENCODE_TIME_FORMAT).to_string());
        Ok(IsNull::No)
    }
}

// Timestamp

impl FromSql<Timestamp, D1Backend> for NaiveDateTime {
    fn from_sql(value: D1Value) -> deserialize::Result<Self> {
        let text = value.read_string();

        for format in NAIVE_DATETIME_FORMATS {
            if let Ok(dt) = Self::parse_from_str(&text, format) {
                return Ok(dt);
            }
        }

        if let Ok(julian_days) = text.parse::<f64>() {
            if let Some(timestamp) = parse_julian(julian_days) {
                return Ok(timestamp);
            }
        }

        Err(format!("Invalid datetime {text}").into())
    }
}

impl ToSql<Timestamp, D1Backend> for NaiveDateTime {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, D1Backend>) -> serialize::Result {
        out.set_value(self.format(ENCODE_NAIVE_DATETIME_FORMAT).to_string());
        Ok(IsNull::No)
    }
}
