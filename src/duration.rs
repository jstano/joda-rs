use crate::temporal::TemporalInstant;
use rust_decimal::Decimal;
use std::fmt;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Duration(time::Duration);

impl Duration {
    pub fn of_weeks(weeks: i64) -> Self {
        Duration(time::Duration::weeks(weeks))
    }

    pub fn of_days(days: i64) -> Self {
        Duration(time::Duration::days(days))
    }

    pub fn of_hours(hours: i64) -> Self {
        Duration(time::Duration::hours(hours))
    }

    pub fn of_minutes(minutes: i64) -> Self {
        Duration(time::Duration::minutes(minutes))
    }

    pub fn of_seconds(seconds: i64) -> Self {
        Duration(time::Duration::seconds(seconds))
    }

    pub fn of_milliseconds(milliseconds: i64) -> Self {
        Duration(time::Duration::milliseconds(milliseconds))
    }

    pub fn of_nanoseconds(nanoseconds: i64) -> Self {
        Duration(time::Duration::nanoseconds(nanoseconds))
    }

    pub fn to_weeks(self) -> i64 {
        self.0.whole_weeks()
    }

    pub fn to_days(self) -> i64 {
        self.0.whole_days()
    }

    pub fn to_hours(self) -> i64 {
        self.0.whole_hours()
    }

    pub fn to_minutes(self) -> i64 {
        self.0.whole_minutes()
    }

    pub fn to_seconds(self) -> i64 {
        self.0.whole_seconds()
    }

    pub fn to_millis(self) -> i64 {
        let ms = self.0.whole_milliseconds();
        let ms = ms.clamp(i128::from(i64::MIN), i128::from(i64::MAX));
        ms as i64
    }

    pub fn to_nanos(self) -> i128 {
        self.0.whole_nanoseconds()
    }

    /// Returns duration as fractional hours, rounded to 4 decimals.
    pub fn fractional_hours(self) -> f64 {
        Self::round_to_decimals(self.0.whole_nanoseconds() as f64 / 3_600_000_000_000.0, 4)
    }

    pub fn fractional_hours_decimal(self) -> Decimal {
        (Decimal::from(self.0.whole_nanoseconds()) / Decimal::from(3_600_000_000_000i128)).round_dp(4)
    }

    /// Returns duration as fractional seconds, rounded to 4 decimals.
    pub fn fractional_seconds(self) -> f64 {
        Self::round_to_decimals(self.0.whole_nanoseconds() as f64 / 1_000_000_000.0, 4)
    }

    pub fn fractional_seconds_decimal(self) -> Decimal {
        (Decimal::from(self.0.whole_nanoseconds()) / Decimal::from(1_000_000_000)).round_dp(4)
    }

    pub fn is_negative(self) -> bool {
        self.0.is_negative()
    }

    pub fn is_zero(self) -> bool {
        self.0.is_zero()
    }

    pub fn is_positive(self) -> bool {
        self.0.is_positive()
    }

    /// Calculates the duration between two temporal instants.
    ///
    /// This function determines the absolute difference in seconds between two temporal
    /// instants provided as `start` and `end`. It returns the resulting duration as a
    /// `Duration` object.
    ///
    /// # Type Parameters
    /// - `T`: A type that implements the `TemporalInstant` trait, providing methods to
    ///   retrieve the temporal instant in epoch seconds.
    ///
    /// # Parameters
    /// - `start`: A reference to the starting temporal instant of type `T`.
    /// - `end`: A reference to the ending temporal instant of type `T`.
    ///
    /// # Returns
    /// - A `Duration` instance representing the absolute duration in seconds between
    ///   the two temporal instants.
    ///
    /// # Examples
    /// ```
    /// use joda_rs::Duration;
    /// use joda_rs::OffsetDateTime;
    ///
    /// let start = OffsetDateTime::now()
    /// let end = end.plus_days(1)
    /// let duration = Duration::between(&start, &end);
    /// assert_eq!(duration.in_seconds(), 86400); // 1 day in seconds
    /// ```
    ///
    /// # Notes
    /// - This function does not account for leap seconds or time zone differences;
    ///   it uses the epoch seconds directly for computation.
    /// - The result is always positive, as the absolute difference is calculated.
    ///
    /// # Errors
    /// - No errors are expected from this method as long as the input types implement
    ///   the `TemporalInstant` trait and provide valid epoch seconds.
    pub fn between<T: TemporalInstant>(start_inclusive: T, end_exclusive: T) -> Self {
        let diff_ns = end_exclusive.epoch_nanoseconds() - start_inclusive.epoch_nanoseconds();
        // Convert i128 nanoseconds → seconds + leftover nanos
        let seconds = (diff_ns / 1_000_000_000) as i64;
        let nanos = (diff_ns % 1_000_000_000) as i32;
        Self(time::Duration::new(seconds, nanos))
    }

    pub fn plus(self, other: Duration) -> Self {
        Duration(self.0 + other.0)
    }

    pub fn minus(self, other: Duration) -> Self {
        Duration(self.0 - other.0)
    }

    pub fn plus_days(self, days: i64) -> Self {
        Duration(self.0.saturating_add(time::Duration::days(days)))
    }

    pub fn minus_days(self, days: i64) -> Self {
        Duration(self.0.saturating_sub(time::Duration::days(days)))
    }

    pub fn plus_hours(self, hours: i64) -> Self {
        Duration(self.0.saturating_add(time::Duration::hours(hours)))
    }
    pub fn minus_hours(self, hours: i64) -> Self {
        Duration(self.0.saturating_sub(time::Duration::hours(hours)))
    }

    pub fn plus_minutes(self, minutes: i64) -> Self {
        Duration(self.0.saturating_add(time::Duration::minutes(minutes)))
    }
    pub fn minus_minutes(self, minutes: i64) -> Self {
        Duration(self.0.saturating_sub(time::Duration::minutes(minutes)))
    }

    pub fn plus_seconds(self, seconds: i64) -> Self {
        Duration(self.0.saturating_add(time::Duration::seconds(seconds)))
    }

    pub fn minus_seconds(self, seconds: i64) -> Self {
        Duration(self.0.saturating_sub(time::Duration::seconds(seconds)))
    }

    pub fn plus_milliseconds(self, milliseconds: i64) -> Self {
        Duration(self.0.saturating_add(time::Duration::milliseconds(milliseconds)))
    }

    pub fn minus_milliseconds(self, milliseconds: i64) -> Self {
        Duration(self.0.saturating_sub(time::Duration::milliseconds(milliseconds)))
    }

    pub fn plus_nanoseconds(self, nanoseconds: i64) -> Self {
        Duration(self.0.saturating_add(time::Duration::nanoseconds(nanoseconds)))
    }

    pub fn minus_nanoseconds(self, nanoseconds: i64) -> Self {
        Duration(self.0.saturating_sub(time::Duration::nanoseconds(nanoseconds)))
    }

    pub fn abs(self) -> Self {
        Duration(self.0.abs())
    }

    pub(crate) fn from(duration: time::Duration) -> Self {
        Self(duration)
    }

    pub(crate) fn inner(self) -> time::Duration {
        self.0
    }

    fn round_to_decimals(value: f64, decimals: u32) -> f64 {
        let factor = 10f64.powi(decimals as i32);
        (value * factor).round() / factor
    }
}

impl fmt::Display for Duration {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
