#[cfg(feature = "serde")]
use time::{Date, PrimitiveDateTime, Time};

#[cfg(feature = "serde")]
pub mod serde_time {
    use super::*;
    use serde::{self, Deserialize, Deserializer, Serializer};
    use time::format_description::well_known::Iso8601;

    pub fn serialize<S>(time: &Time, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let s = time.format(&Iso8601::DEFAULT).map_err(serde::ser::Error::custom)?;
        serializer.serialize_str(&s)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Time, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Time::parse(&s, &Iso8601::DEFAULT).map_err(serde::de::Error::custom)
    }
}

#[cfg(feature = "serde")]
pub mod serde_date {
    use super::*;
    use serde::{self, Deserialize, Deserializer, Serializer};
    use time::format_description::well_known::Iso8601;

    pub fn serialize<S>(date: &Date, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let s = date.format(&Iso8601::DEFAULT).map_err(serde::ser::Error::custom)?;
        serializer.serialize_str(&s)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Date, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Date::parse(&s, &Iso8601::DEFAULT).map_err(serde::de::Error::custom)
    }
}

#[cfg(feature = "serde")]
pub mod serde_datetime {
    use super::*;
    use serde::{self, Deserialize, Deserializer, Serializer};
    use time::format_description::well_known::Iso8601;

    pub fn serialize<S>(datetime: &PrimitiveDateTime, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let s = datetime.format(&Iso8601::DEFAULT).map_err(serde::ser::Error::custom)?;
        serializer.serialize_str(&s)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<PrimitiveDateTime, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        PrimitiveDateTime::parse(&s, &Iso8601::DEFAULT).map_err(serde::de::Error::custom)
    }
}
