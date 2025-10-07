use crate::{Clock, OffsetDateTime, TemporalInstant, ZoneId, ZoneOffset, ZonedDateTime};
use std::ops::{Add, Sub};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Instant(time::OffsetDateTime);

impl Instant {
    /// Returns the current instant as an `Instant`.
    ///
    /// This function creates a new instance of `Instant` representing the current
    /// system time, retrieved using `SystemTime::now()`.
    ///
    /// # Returns
    /// An `Instant` initialized with the current time.
    ///
    /// # Note
    /// The behavior and accuracy of the time retrieval may depend on the underlying
    /// operating system and hardware.
    ///
    /// # Examples
    /// ```rust
    /// let current_instant = Instant::now();
    /// println!("{:?}", current_instant);
    /// ```
    pub fn now() -> Self {
        Self(time::OffsetDateTime::now_utc())
    }

    pub fn now_clock(clock: &Clock) -> Self {
        clock.instant()
    }

    /// Constructs an `Instant` from the given number of seconds since the Unix epoch.
    ///
    /// # Parameters
    /// - `epoch_second`: A 64-bit integer representing the number of seconds elapsed since
    ///   January 1, 1970 (Unix epoch). If the value is negative, it implicitly represents
    ///   a time before the Unix epoch.
    ///
    /// # Returns
    /// - Returns a new `Instant` instance representing the point in time specified by the
    ///   `epoch_second`.
    ///
    /// # Panics
    /// - This function will panic if the computation of the resulting duration exceeds the valid range
    ///   for `std::time::Duration` (e.g., if adding `UNIX_EPOCH` and `duration` results in an
    ///   overflow).
    ///
    /// # Example
    /// ```
    /// let instant = Instant::from_epoch_second(1625077800);
    /// ```
    pub fn of_epoch_second(epoch_second: i64) -> Self {
        let duration = time::Duration::new(epoch_second, 0);
        Self(time::OffsetDateTime::UNIX_EPOCH + duration)
    }

    /// Creates a new `Instant` object from the specified number of epoch seconds
    /// and an additional nanosecond adjustment.
    ///
    /// This method computes the total number of nanoseconds since the epoch
    /// (`UNIX_EPOCH`) by combining `epoch_second` and `nano_adjustment`. It ensures
    /// that any overflow in the addition is gracefully handled, panicking if an
    /// overflow occurs. The result is then converted into seconds and nanoseconds
    /// to construct the `Instant`.
    ///
    /// # Parameters
    ///
    /// * `epoch_second` - The number of seconds since the Unix epoch (January 1, 1970 00:00:00 UTC).
    /// * `nano_adjustment` - An additional adjustment in nanoseconds, which can be positive
    ///   or negative.
    ///
    /// # Panics
    ///
    /// This function will panic if the computation of total nanoseconds overflows
    /// or underflows the range of valid `i64` values.
    ///
    /// # Returns
    ///
    /// Returns an instance of `Self` (`Instant`), representing the specified point
    /// in time relative to the epoch.
    ///
    /// # Examples
    ///
    /// ```
    /// let instant = Instant::from_epoch_second_nano(1_600_000_000, 500_000_000); // November 13, 2020
    /// ```
    pub fn of_epoch_second_nano(epoch_second: i64, nano_adjustment: i32) -> Self {
        let duration = time::Duration::new(epoch_second, nano_adjustment);
        Self(time::OffsetDateTime::UNIX_EPOCH + duration)
    }

    /// Creates a new `Instant` instance from the provided epoch milliseconds.
    ///
    /// # Parameters
    /// - `epoch_milli`: An `i64` value representing the number of milliseconds since the Unix epoch
    ///   (January 1, 1970, 00:00:00 UTC).
    ///
    /// # Returns
    /// Returns an `Instant` instance that represents the equivalent point in time based on the input.
    ///
    /// # Details
    /// - The method computes the seconds and remaining milliseconds from the given `epoch_milli`.
    /// - It combines these values to construct a `Duration` and then adds it to the `UNIX_EPOCH`.
    ///
    /// # Example
    /// ```rust
    /// use joda_rs::Instant;
    ///
    /// let epoch_milli = 1_687_307_200_000; // Example epoch milliseconds.
    /// let instant = Instant::from_epoch_milli(epoch_milli);
    /// ```
    ///
    /// Note: This implementation assumes that the input `epoch_milli` is non-negative and within the
    /// representable range of the Unix timestamp.
    pub fn of_epoch_millisecond(epoch_millisecond: i64) -> Self {
        let secs = epoch_millisecond.div_euclid(1000);
        let millis = epoch_millisecond.rem_euclid(1000);
        Self(
            time::OffsetDateTime::UNIX_EPOCH
                + time::Duration::new(secs, (millis * 1_000_000) as i32),
        )
    }

    /// Returns the number of seconds from the Unix Epoch.
    ///
    /// Epoch time is the number of seconds that have elapsed since
    /// January 1, 1970 00:00:00 UTC (the Unix epoch).
    ///
    /// Leap seconds are not taken into account.
    ///
    /// ### Returns
    /// An `i64` representing the number of seconds elapsed since the Unix Epoch.
    ///
    /// ### Example
    /// ```rust
    /// let epoch_seconds = OffsetDateTime.now_utc().to_epoch_seconds();
    /// println!("Epoch seconds: {}", epoch_seconds);
    /// ```
    pub fn epoch_seconds(self) -> i64 {
        self.0.unix_timestamp()
    }

    /// Returns the number of milliseconds elapsed since the Unix Epoch.
    ///
    /// Epoch time is the number of milliseconds that have elapsed since
    /// January 1, 1970 00:00:00 UTC (the Unix epoch).
    ///
    /// Leap seconds are not taken into account.
    ///
    /// ### Returns
    /// An `i128` representing the epoch time in milliseconds.
    ///
    /// ### Example
    /// ```rust
    /// let timestamp = OffsetDateTime.now_utc();
    /// let milliseconds = timestamp.epoch_milliseconds();
    /// println!("Epoch time in milliseconds: {}", milliseconds);
    /// ```
    pub fn epoch_milliseconds(self) -> i128 {
        self.epoch_seconds() as i128 * 1_000
            + (self.epoch_nanoseconds() % 1_000_000_000) as i128 / 1_000
    }

    /// Returns the number of nanoseconds elapsed since the Unix Epoch.
    ///
    /// Epoch time is the number of nanoseconds that have elapsed since
    /// January 1, 1970 00:00:00 UTC (the Unix epoch).
    ///
    /// Leap seconds are not taken into account.
    ///
    /// ### Returns
    /// An `i128` representing the epoch time in nanoseconds.
    ///
    /// ### Examples
    /// ```rust
    /// let nanoseconds = OffsetDateTime.now_utc().epoch_nanoseconds();
    /// println!("Epoch time in nanoseconds: {}", nanoseconds);
    /// ```
    pub fn epoch_nanoseconds(self) -> i128 {
        let seconds = self.0.unix_timestamp() as i128;
        let nanos = self.0.nanosecond() as i128;
        seconds * 1_000_000_000 + nanos
    }

    pub fn plus_seconds(self, seconds: i64) -> Self {
        Self(self.0.add(time::Duration::seconds(seconds)))
    }

    pub fn minus_seconds(self, seconds: i64) -> Self {
        Self(self.0.sub(time::Duration::seconds(seconds)))
    }

    pub fn plus_milliseconds(self, milliseconds: i64) -> Self {
        Self(self.0.add(time::Duration::milliseconds(milliseconds)))
    }

    pub fn minus_milliseconds(self, milliseconds: i64) -> Self {
        Self(self.0.sub(time::Duration::milliseconds(milliseconds)))
    }

    pub fn plus_nanoseconds(self, nanoseconds: i64) -> Self {
        Self(self.0.add(time::Duration::nanoseconds(nanoseconds)))
    }

    pub fn minus_nanoseconds(self, nanoseconds: i64) -> Self {
        Self(self.0.sub(time::Duration::nanoseconds(nanoseconds)))
    }

    pub fn at_offset(self, offset: ZoneOffset) -> OffsetDateTime {
        let utc_datetime = self.0.to_offset(time::UtcOffset::UTC);
        let offset_seconds = offset.total_seconds();
        let target_offset =
            time::UtcOffset::from_whole_seconds(offset_seconds).expect("Invalid timezone offset");
        let target_datetime = utc_datetime.to_offset(target_offset);
        OffsetDateTime::from(target_datetime)
    }

    pub fn at_zone(self, zone: ZoneId) -> ZonedDateTime {
        let utc_datetime = self.0.to_offset(time::UtcOffset::UTC);
        let offset_seconds = zone.to_offset();
        let target_offset =
            time::UtcOffset::from_whole_seconds(offset_seconds).expect("Invalid timezone offset");
        let target_datetime = utc_datetime.to_offset(target_offset);
        ZonedDateTime::from(target_datetime)
    }

    /// Determines whether the current instance is before another instance.
    ///
    /// ### Arguments
    /// - `other`: A reference to the instance to compare against.
    ///
    /// ### Returns
    /// - `true` if the current instance occurs before the `other` instance.
    /// - `false` otherwise.
    ///
    /// ### Example
    /// ```rust
    /// let date1 = LocalDate::now();
    /// let date2 = date1.plus_hours(1);
    ///
    /// assert!(date1.is_before(date2));
    /// assert!(!date2.is_before(date1));
    /// ```
    pub fn is_before(self, other: Self) -> bool {
        self < other
    }

    /// Determines whether the current instance is after another instance.
    ///
    /// ### Arguments
    /// - `other`: A reference to the instance to compare against.
    ///
    /// ### Returns
    /// - `true` if the current instance occurs after the `other` instance.
    /// - `false` otherwise.
    ///
    /// ### Example
    /// ```rust
    /// let date1 = LocalDate::now();
    /// let date2 = date1.plus_hours(1);
    ///
    /// assert!(!date1.is_after(date2));
    /// assert!(date2.is_after(date1));
    /// ```
    pub fn is_after(self, other: Self) -> bool {
        self > other
    }

    /// Determines whether the current instance is on or before another instance.
    ///
    /// ### Arguments
    /// - `other`: A reference to the instance to compare against.
    ///
    /// ### Returns
    /// - `true` if the current instance occurs on or before the `other` instance.
    /// - `false` otherwise.
    ///
    /// ### Example
    /// ```rust
    /// let date1 = LocalDate::now();
    /// let date2 = date1.plus_hours(1);
    /// let date3 = date1.minus_hours(1);
    ///
    /// assert!(date1.is_on_or_before(date1));
    /// assert!(date1.is_on_or_before(date2));
    /// assert!(!date1.is_on_or_before(date3));
    /// ```
    pub fn is_on_or_before(self, other: Self) -> bool {
        self <= other
    }

    /// Determines whether the current other is on or after another instance.
    ///
    /// ### Arguments
    /// - `other`: A reference to the instance to compare against.
    ///
    /// ### Returns
    /// - `true` if the current instance occurs on or after the `other` instance.
    /// - `false` otherwise.
    ///
    /// ### Example
    /// ```rust
    /// let date1 = LocalDate::now();
    /// let date2 = date1.minus_hours(1);
    /// let date3 = date1.plus_hours(1);
    ///
    /// assert!(date1.is_on_or_after(date1));
    /// assert!(date1.is_on_or_after(date2));
    /// assert!(!date1.is_on_or_after(date3));
    /// ```
    pub fn is_on_or_after(self, other: Self) -> bool {
        self >= other
    }

    // pub(crate) fn inner(self) -> time::OffsetDateTime {
    //     self.0
    // }
}

impl TemporalInstant for Instant {
    fn epoch_seconds(self) -> i64 {
        Self::epoch_seconds(self)
    }

    fn epoch_milliseconds(self) -> i128 {
        Self::epoch_milliseconds(self)
    }

    fn epoch_nanoseconds(self) -> i128 {
        Self::epoch_nanoseconds(self)
    }
}
