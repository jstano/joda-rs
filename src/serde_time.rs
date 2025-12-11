use time::Time;

#[cfg(feature = "serde")]
mod serde_time {
    use super::*;
    use serde::{self, Deserialize, Deserializer, Serializer};

    const FORMAT: &str = "%H:%M:%S%.f"; // HH:MM:SS with optional subseconds

    pub fn serialize<S>(time: &Time, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let s = time.format(FORMAT);
        serializer.serialize_str(&s)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Time, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Time::parse(&s, FORMAT).map_err(serde::de::Error::custom)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(transparent))]
pub struct LocalTime(
    #[cfg_attr(feature = "serde", serde(with = "serde_time"))]
    pub Time,
);
