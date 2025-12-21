#[cfg(feature = "serde")]
use time::{Date, PrimitiveDateTime, Time};

#[cfg(feature = "serde")]
pub mod serde_time {
    use super::*;
    use serde::{self, Deserialize, Deserializer, Serializer};
    use time::format_description::FormatItem;
    use time::macros::format_description;

    const TIME_FORMAT: &[FormatItem] = format_description!("[hour]:[minute]:[second][optional [.[subsecond]]]");

    pub fn serialize<S>(time: &Time, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let s = time.format(&TIME_FORMAT).map_err(serde::ser::Error::custom)?;
        serializer.serialize_str(&s)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Time, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Time::parse(&s, &TIME_FORMAT).map_err(serde::de::Error::custom)
    }
}

#[cfg(feature = "serde")]
pub mod serde_date {
    use super::*;
    use serde::{self, Deserialize, Deserializer, Serializer};
    use time::format_description::FormatItem;
    use time::macros::format_description;

    const DATE_FORMAT: &[FormatItem] = format_description!("[year]-[month]-[day]");

    pub fn serialize<S>(date: &Date, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let s = date.format(&DATE_FORMAT).map_err(serde::ser::Error::custom)?;
        serializer.serialize_str(&s)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Date, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Date::parse(&s, &DATE_FORMAT).map_err(serde::de::Error::custom)
    }
}

#[cfg(feature = "serde")]
pub mod serde_datetime {
    use super::*;
    use serde::{self, Deserialize, Deserializer, Serializer};
    use time::format_description::FormatItem;
    use time::macros::format_description;

    const DATETIME_FORMAT: &[FormatItem] = format_description!("[year]-[month]-[day]T[hour]:[minute]:[second]");

    pub fn serialize<S>(datetime: &PrimitiveDateTime, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let s = datetime.format(&DATETIME_FORMAT).map_err(serde::ser::Error::custom)?;
        serializer.serialize_str(&s)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<PrimitiveDateTime, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        PrimitiveDateTime::parse(&s, &DATETIME_FORMAT).map_err(serde::de::Error::custom)
    }
}
