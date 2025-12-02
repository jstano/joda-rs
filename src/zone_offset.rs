#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ZoneOffset(time::UtcOffset);

impl ZoneOffset {
    /// A constant representing the UTC (Coordinated Universal Time) zone offset.
    ///
    /// # Details
    /// - `UTC` is defined as a `ZoneOffset` with an offset value of `0` seconds
    ///   from the Coordinated Universal Time (UTC).
    ///
    /// # Example Usage
    /// ```rust
    /// let utc_offset = ZoneOffset::UTC;
    /// println!("{:?}", utc_offset);
    /// ```
    ///
    /// This can be useful when working with time zones or when needing
    /// a reference to UTC in your application.
    pub const UTC: ZoneOffset = ZoneOffset {
        0: time::UtcOffset::UTC,
    };

    /// Creates a new `ZoneOffset` instance with the given number of hours as the offset from UTC.
    ///
    /// # Parameters
    /// - `hours`: The offset, in hours, to be applied to UTC. Must be within the valid range of
    ///   -23 to +23 (inclusive).
    ///
    /// # Returns
    /// A `ZoneOffset` instance representing the provided offset.
    ///
    /// # Panics
    /// This function will panic if the provided `hours` value is outside the range of valid UTC offsets.
    ///
    /// # Examples
    /// ```
    /// use joda_rs::ZoneOffset;
    ///
    /// let offset = ZoneOffset::of_hours(5); // UTC+5
    /// ```
    pub fn of_hours(hours: i8) -> Self {
        ZoneOffset(time::UtcOffset::from_hms(hours, 0, 0).expect("invalid offset hours"))
    }

    /// Constructs a `ZoneOffset` from the specified hours and minutes.
    ///
    /// # Arguments
    ///
    /// * `hours` - An `i8` value representing the number of hours for the offset.
    ///   This can range from -23 to 23, where negative values represent time behind
    ///   UTC and positive values represent time ahead of UTC.
    ///
    /// * `minutes` - An `i8` value representing the number of minutes for the offset.
    ///   This can range from 0 to 59. Combined with `hours`, the total offset is
    ///   calculated as hours:minutes relative to UTC.
    ///
    /// # Returns
    ///
    /// A new instance of `ZoneOffset` that represents the provided UTC offset.
    ///
    /// # Panics
    ///
    /// This function will panic if the provided `hours` and `minutes` values result
    /// in an invalid UTC offset that cannot be represented by `time::UtcOffset`.
    ///
    /// # Example
    ///
    /// ```rust
    /// let offset1 = ZoneOffset::of_hours_minutes(5, 30); // Represents a UTC offset of +05:30
    /// let offset2 = ZoneOffset::of_hours_minutes(-8, 0); // Represents a UTC offset of -08:00
    /// ```
    pub fn of_hours_minutes(hours: i8, minutes: i8) -> Self {
        ZoneOffset(time::UtcOffset::from_hms(hours, minutes, 0).expect("invalid offset"))
    }

    /// Calculates the total number of seconds represented by the zone offset.
    ///
    /// The function extracts the hours, minutes, and seconds from the time stored in the instance,
    /// then converts each component into seconds and computes their total sum.
    ///
    /// # Returns
    /// * `i32` - The total number of seconds as an integer.
    ///
    /// # Example
    /// ```
    /// let time = ZoneOffset::of_hours_minutes(8, 30);
    /// let total_secs = time.total_seconds(); // 8 hours, 30 minutes, 0 seconds => 30,600 seconds
    /// assert_eq!(total_secs, 30_600);
    /// ```
    pub fn total_seconds(self) -> i32 {
        let (h, m, s) = self.0.as_hms();
        (h as i32) * 3600 + (m as i32) * 60 + (s as i32)
    }

    pub(crate) fn inner(self) -> time::UtcOffset {
        self.0
    }
}

impl From<time::UtcOffset> for ZoneOffset {
    fn from(value: time::UtcOffset) -> Self {
        ZoneOffset(value)
    }
}

impl From<ZoneOffset> for time::UtcOffset {
    fn from(value: ZoneOffset) -> Self {
        value.0
    }
}
