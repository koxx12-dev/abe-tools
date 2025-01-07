use crate::proto::bcl;
use crate::proto::bcl::date_time::{DateTimeKind, TimeSpanScale};
use anyhow::anyhow;
use chrono::{DateTime, FixedOffset, NaiveDateTime, Utc};

#[cfg(feature = "serde")]
use serde::{de::Visitor,Serialize};

static MIN_TIMESTAMP: i64 = -8334601228800;
static MAX_TIMESTAMP: i64 = 8210266876799;

#[allow(dead_code)]
static DATE_FORMAT: &str = "%Y-%m-%dT%H:%M:%S";

impl TryFrom<bcl::DateTime> for DateTime<Utc> {
    type Error = &'static str;
    fn try_from(value: bcl::DateTime) -> Result<Self, Self::Error> {
        let scale = value.scale();
        let value = value.value;

        let seconds = match scale {
            TimeSpanScale::Seconds => value,
            TimeSpanScale::Minutes => value * 60,
            TimeSpanScale::Hours => value * 3600,
            TimeSpanScale::Days => value * 86400,
            TimeSpanScale::Milliseconds => value / 1000,
            TimeSpanScale::Ticks => value / 10000000,
            TimeSpanScale::Minmax => {
                if value == -1 {
                    MIN_TIMESTAMP
                } else if value == 1 {
                    MAX_TIMESTAMP
                } else {
                    return Err("invalid minmax value");
                }
            }
        };

        let dt = DateTime::from_timestamp(seconds, 0).ok_or("invalid or out-of-range datetime")?;

        Ok(dt)
    }
}

impl From<DateTime<Utc>> for bcl::DateTime {
    fn from(value: DateTime<Utc>) -> Self {
        let mut value = value.timestamp();

        let scale = if value == MIN_TIMESTAMP || value == MAX_TIMESTAMP {
            if value == MIN_TIMESTAMP {
                value = -1;
            } else {
                value = 1;
            }

            TimeSpanScale::Minmax
        } else if value % 86400 == 0 {
            value /= 86400;
            TimeSpanScale::Days
        } else if value % 3600 == 0 {
            value /= 3600;
            TimeSpanScale::Hours
        } else if value % 60 == 0 {
            value /= 60;
            TimeSpanScale::Minutes
        } else if value % 1 == 0 {
            //value is already in seconds
            TimeSpanScale::Seconds
        } else if value % 1000 == 0 {
            value *= 1000;
            TimeSpanScale::Milliseconds
        } else {
            value *= 10000000;
            TimeSpanScale::Ticks
        };

        bcl::DateTime {
            value,
            scale: Some(scale as i32),
            kind: Some(DateTimeKind::Unspecified as i32),
        }
    }
}
#[cfg(feature = "serde")]
impl Serialize for bcl::DateTime {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let t: DateTime<Utc> = self.clone().try_into().map_err(serde::ser::Error::custom)?;
        serializer.serialize_str(t.to_rfc3339().as_str())
    }
}

//Taken from pbjson-types
#[allow(dead_code)]
struct DatetimeVisitor;

#[allow(dead_code)]
fn parse_custom_timestamp(timestamp: &str) -> anyhow::Result<DateTime<FixedOffset>> {
    let (datetime_str, offset_str) = timestamp
        .rsplit_once('+')
        .ok_or(anyhow!("Failed to split timestamp"))?;

    let naive_dt = NaiveDateTime::parse_from_str(datetime_str, DATE_FORMAT)
        .expect("Failed to parse date and time");

    let offset = FixedOffset::east_opt(
        offset_str
            .split(':')
            .enumerate()
            .map(|(i, v)| {
                v.parse::<i32>().expect("failed to parse i32") * if i == 0 { 3600 } else { 60 }
            })
            .sum(),
    )
    .unwrap();

    Ok(DateTime::<FixedOffset>::from_naive_utc_and_offset(
        naive_dt, offset,
    ))
}

#[cfg(feature = "serde")]
impl<'de> Visitor<'de> for DatetimeVisitor {
    type Value = bcl::DateTime;

    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str("a date string")
    }

    fn visit_str<E>(self, s: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        let d = parse_custom_timestamp(s).map_err(serde::de::Error::custom)?;
        let d: DateTime<Utc> = d.into();
        Ok(d.into())
    }
}
#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for bcl::DateTime {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_str(DatetimeVisitor)
    }
}
