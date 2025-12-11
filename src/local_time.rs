use crate::{Clock, Duration, Instant, LocalDate, LocalDateTime, ZoneId};
use core::ops::Sub;
use std::fmt;
use std::ops::Add;
use time::UtcOffset;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(transparent))]
pub struct LocalTime(
    #[cfg_attr(feature = "serde", serde(with = "time::serde::rfc3339"))]
    time::Time
);

impl LocalTime {
    pub fn now() -> Self {
        Self(time::OffsetDateTime::now_utc().to_offset(UtcOffset::current_local_offset().unwrap()).time())
    }

    pub fn now_with_clock(clock: &Clock) -> Self {
        clock.instant().at_zone(clock.zone()).to_local_time()
    }

    pub fn now_with_zone(zone: ZoneId) -> Self {
        Instant::now().at_zone(zone).to_local_time()
    }

    pub fn new(hour: i32, minute: i32, second: i32) -> Self {
        Self(time::Time::from_hms(hour as u8, minute as u8, second as u8).expect("invalid time"))
    }

    pub fn of(hour: i32, minute: i32, second: i32) -> Self {
        Self::new(hour, minute, second)
    }

    pub fn of_hour_minute(hour: i32, minute: i32) -> Self {
        Self::new(hour, minute, 0)
    }

    pub fn of_hms_nano(hour: i32, minute: i32, second: i32, nanosecond: i32) -> Self {
        let nano = u32::try_from(nanosecond).expect("x must be non-negative");
        Self(
            time::Time::from_hms_nano(hour as u8, minute as u8, second as u8, nano)
                .expect("invalid time"),
        )
    }

    pub fn of_second_of_day(second: i32) -> Self {
        let total_nanos = i128::from(second) * 1_000_000_000_i128;
        Self::of_total_nanos_of_day(total_nanos)
    }

    pub fn of_nanosecond_of_day(nanosecond: i64) -> Self {
        Self::of_total_nanos_of_day(i128::from(nanosecond))
    }

    pub fn of_total_nanos_of_day(total: i128) -> Self {
        // wrap around 24h like java.time
        const DAY_NANOS: i128 = 86_400_i128 * 1_000_000_000_i128;
        let mut n = total % DAY_NANOS;
        if n < 0 {
            n += DAY_NANOS;
        }
        let hour = (n / 3_600_000_000_000) as u8; // 3600*1e9
        n %= 3_600_000_000_000;
        let minute = (n / 60_000_000_000) as u8;
        n %= 60_000_000_000;
        let second = (n / 1_000_000_000) as u8;
        let nano = (n % 1_000_000_000) as u32;
        LocalTime(time::Time::from_hms_nano(hour, minute, second, nano).unwrap())
    }

    pub fn parse(s: &str) -> Self {
        use time::format_description::well_known::Iso8601;
        Self(time::Time::parse(s, &Iso8601::DEFAULT).expect("invalid time string"))
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
    /// let time1 = LocalTime::now();
    /// let time2 = time1.plus_hours(1);
    ///
    /// assert!(time1.is_before(time2));
    /// assert!(!time2.is_before(time1));
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
    /// let time1 = LocalTime::now();
    /// let time2 = time1.plus_hours(1);
    ///
    /// assert!(!time1.is_after(time2));
    /// assert!(time2.is_after(time1));
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
    /// let time1 = LocalTime::now();
    /// let time2 = time1.plus_hours(1);
    /// let time3 = time1.minus_hours(1);
    ///
    /// assert!(time1.is_on_or_before(time1));
    /// assert!(time1.is_on_or_before(time2));
    /// assert!(!time1.is_on_or_before(time3));
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
    /// let time1 = LocalTime::now();
    /// let time2 = time1.minus_hours(1);
    /// let time3 = time1.plus_hours(1);
    ///
    /// assert!(time1.is_on_or_after(time1));
    /// assert!(time1.is_on_or_after(time2));
    /// assert!(!time1.is_on_or_after(time3));
    /// ```
    pub fn is_on_or_after(self, other: Self) -> bool {
        self >= other
    }

    /// Returns the hour component of the time.
    ///
    /// ### Returns
    /// An `i32` representing the hour of the time.
    ///
    /// ### Example
    /// ```rust
    /// let hour = LocalTime::now().hour();
    /// println!("Hour value: {}", hour);
    /// ```
    pub fn hour(self) -> i32 {
        self.0.hour() as i32
    }

    /// Returns the minute component of the time.
    ///
    /// ### Returns
    /// An `i32` representing the minute of the time.
    ///
    /// ### Example
    /// ```rust
    /// let minute = LocalTime::now().minute();
    /// println!("Minute value: {}", minute);
    /// ```
    pub fn minute(self) -> i32 {
        self.0.minute() as i32
    }

    /// Returns the second component of the time.
    ///
    /// ### Returns
    /// An `i32` representing the second of the time.
    ///
    /// ### Example
    /// ```rust
    /// let second = LocalTime::now().second();
    /// println!("Second value: {}", second);
    /// ```
    pub fn second(self) -> i32 {
        self.0.second() as i32
    }

    /// Returns the millisecond component of the time.
    ///
    /// # Returns
    /// An `i32` representing the millisecond portion of the time, ranging from 0 to 999.
    ///
    /// # Example
    /// ```rust
    /// let time = LocalTime::now()
    /// let milliseconds = time.millisecond();
    /// println!("Milliseconds: {}", milliseconds);
    /// ```
    pub fn millisecond(self) -> i32 {
        self.0.millisecond() as i32
    }

    /// Returns the nanosecond component of the time.
    ///
    /// ### Returns
    /// An `i32` representing the nanosecond portion of the time.
    ///
    /// ### Example
    /// ```rust
    /// let nanoseconds = LocalTime::now().nanosecond();
    /// println!("Nanoseconds value: {}", nanoseconds);
    /// ```
    pub fn nanosecond(self) -> i32 {
        self.0.nanosecond() as i32
    }

    pub fn plus_hours(self, hours: i64) -> Self {
        Self(self.0.add(Duration::of_hours(hours).inner()))
    }

    pub fn plus_minutes(self, minutes: i64) -> Self {
        Self(self.0.add(Duration::of_minutes(minutes).inner()))
    }

    pub fn plus_seconds(self, seconds: i64) -> Self {
        Self(self.0.add(Duration::of_seconds(seconds).inner()))
    }

    pub fn plus_milliseconds(self, milliseconds: i64) -> Self {
        Self(self.0.add(Duration::of_milliseconds(milliseconds).inner()))
    }

    pub fn plus_nanoseconds(self, nanoseconds: i64) -> Self {
        Self(self.0.add(Duration::of_nanoseconds(nanoseconds).inner()))
    }

    pub fn minus_hours(self, hours: i64) -> Self {
        Self(self.0.sub(Duration::of_hours(hours).inner()))
    }

    pub fn minus_minutes(self, minutes: i64) -> Self {
        Self(self.0.sub(Duration::of_minutes(minutes).inner()))
    }

    pub fn minus_seconds(self, seconds: i64) -> Self {
        Self(self.0.sub(Duration::of_seconds(seconds).inner()))
    }

    pub fn minus_milliseconds(self, milliseconds: i64) -> Self {
        Self(self.0.sub(Duration::of_milliseconds(milliseconds).inner()))
    }

    pub fn minus_nanoseconds(self, nanoseconds: i64) -> Self {
        Self(self.0.sub(Duration::of_nanoseconds(nanoseconds).inner()))
    }

    pub fn with_hour(self, hour: u8) -> Self {
        let time = self.0.replace_hour(hour).expect("invalid hour");
        Self(time::Time::from(time))
    }

    pub fn with_minute(self, minute: u8) -> Self {
        let time = self.0.replace_minute(minute).expect("invalid minute");
        Self(time::Time::from(time))
    }
    pub fn with_second(self, second: u8) -> Self {
        let time = self.0.replace_second(second).expect("invalid second");
        Self(time::Time::from(time))
    }

    pub fn with_millisecond(self, millisecond: u16) -> Self {
        let time = self
            .0
            .replace_millisecond(millisecond)
            .expect("invalid millisecond");
        Self(time::Time::from(time))
    }

    pub fn with_nanosecond(self, nanosecond: u32) -> Self {
        let time = self
            .0
            .replace_nanosecond(nanosecond)
            .expect("invalid nanosecond");
        Self(time::Time::from(time))
    }

    pub fn at_date(self, date: LocalDate) -> LocalDateTime {
        LocalDateTime::of_date_time(date, self)
    }

    pub(crate) fn from(time: time::Time) -> Self {
        LocalTime(time)
    }

    pub(crate) fn inner(self) -> time::Time {
        self.0
    }
}

impl Sub for LocalTime {
    type Output = Duration;
    fn sub(self, rhs: Self) -> Self::Output {
        let duration: time::Duration = self.0 - rhs.0;
        Duration::from(duration)
    }
}

impl fmt::Display for LocalTime {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
