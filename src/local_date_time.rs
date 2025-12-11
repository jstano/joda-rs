use crate::{Clock, DayOfWeek, Duration, Instant, LocalDate, LocalTime, Month, OffsetDateTime, TemporalInstant, Year, ZoneId, ZoneOffset, ZonedDateTime};
use std::fmt;
use time::UtcOffset;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(transparent))]
pub struct LocalDateTime(
    #[cfg_attr(feature = "serde", serde(with = "time::serde::rfc3339"))]
    time::PrimitiveDateTime
);

impl LocalDateTime {
    pub fn now() -> Self {
        let odt = time::OffsetDateTime::now_utc();
        Self(time::PrimitiveDateTime::new(odt.date(), odt.time()))
    }

    pub fn now_with_clock(clock: &Clock) -> Self {
        clock.instant().at_zone(clock.zone()).to_local_date_time()
    }

    pub fn now_with_zone(zone: ZoneId) -> Self {
        Instant::now().at_zone(zone).to_local_date_time()
    }

    pub fn new(year: i32, month: i32, day: i32, hour: i32, minute: i32, second: i32) -> Self {
        let date = LocalDate::of(year, month, day);
        let time = LocalTime::of(hour, minute, second);
        Self::of_date_time(date, time)
    }

    pub fn of(year: i32, month: i32, day: i32, hour: i32, minute: i32, second: i32) -> Self {
        Self::new(year, month, day, hour, minute, second)
    }

    pub fn of_date_time(date: LocalDate, time: LocalTime) -> Self {
        Self(time::PrimitiveDateTime::new(date.inner(), time.inner()))
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
    /// let datetime1 = LocalDateTime::now();
    /// let datetime2 = datetime1.plus_hours(1);
    ///
    /// assert!(datetime1.is_before(datetime2));
    /// assert!(!datetime2.is_before(datetime1));
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
    /// let datetime1 = LocalDateTime::now();
    /// let datetime2 = datetime1.plus_hours(1);
    ///
    /// assert!(!datetime1.is_after(datetime2));
    /// assert!(datetime2.is_after(datetime1));
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
    /// let datetime1 = LocalDateTime::now();
    /// let datetime2 = datetime1.plus_hours(1);
    /// let datetime3 = datetime1.minus_hours(1);
    ///
    /// assert!(datetime1.is_on_or_before(datetime1));
    /// assert!(datetime1.is_on_or_before(datetime2));
    /// assert!(!datetime1.is_on_or_before(datetime3));
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
    /// let datetime1 = LocalDateTime::now();
    /// let datetime2 = datetime1.minus_hours(1);
    /// let datetime3 = datetime1.plus_hours(1);
    ///
    /// assert!(datetime1.is_on_or_after(datetime1));
    /// assert!(datetime1.is_on_or_after(datetime2));
    /// assert!(!datetime1.is_on_or_after(datetime3));
    /// ```
    pub fn is_on_or_after(self, other: Self) -> bool {
        self >= other
    }

    pub fn to_instant(self, zone: ZoneId) -> Instant {
        Instant::of_epoch_second(self.epoch_seconds()).at_zone(zone).to_instant()
    }

    pub fn to_instant_utc(self) -> Instant {
        Instant::of_epoch_second(self.epoch_seconds()).at_zone(ZoneId::UTC).to_instant()
    }

    pub fn at_zone(self, zone: ZoneId) -> ZonedDateTime {
        ZonedDateTime::of(self, zone)
    }

    pub fn at_offset(self, offset: ZoneOffset) -> OffsetDateTime {
        OffsetDateTime::of(self, offset)
    }

    /// Converts the current instance into a `LocalDate`.
    ///
    /// ### Returns
    /// - A `LocalDate` that represents the local date portion of the current instance.
    ///
    /// ### Example
    /// ```rust
    /// let local_date = LocalDateTime::now().to_local_date();
    /// println!("{}", local_date);
    /// ```
    pub fn to_local_date(self) -> LocalDate {
        LocalDate::from(self.0.date())
    }

    /// Converts the current instance of a time object to its equivalent local time representation.
    ///
    /// ### Returns
    /// * `LocalTime` - A representation of the time adjusted to the local timezone.
    ///
    /// ### Example
    /// ```rust
    /// let local_time = LocalDateTime::now().to_local_time();
    /// println!("Local time: {:?}", local_time);
    /// ```
    pub fn to_local_time(self) -> LocalTime {
        LocalTime::from(self.0.time())
    }

    /// Converts the current instance into a `LocalDateTime`.
    ///
    /// ### Returns
    /// - A `LocalDateTime` that represents the date portion of the current instance.
    ///
    /// ### Example
    /// ```rust
    /// let local_date_time = LocalDateTime::now().to_local_date_time();
    /// println!("{}", local_date_time);
    /// ```
    pub fn to_local_date_time(self) -> LocalDateTime {
        self.clone()
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
    /// let epoch_seconds = ZonedDateTime.now_utc().epoch_seconds();
    /// println!("Epoch seconds: {}", epoch_seconds);
    /// ```
    pub fn epoch_seconds(self) -> i64 {
        self.0.assume_offset(UtcOffset::UTC).unix_timestamp()
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
    /// let timestamp = ZonedDateTime.now_utc();
    /// let milliseconds = timestamp.epoch_milliseconds();
    /// println!("Epoch time in milliseconds: {}", milliseconds);
    /// ```
    fn epoch_milliseconds(self) -> i128 {
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
    /// let nanoseconds = ZonedDateTime.now_utc().epoch_nanoseconds();
    /// println!("Epoch time in nanoseconds: {}", nanoseconds);
    /// ```
    pub fn epoch_nanoseconds(self) -> i128 {
        self.0.assume_offset(UtcOffset::UTC).unix_timestamp_nanos()
    }

    /// Returns the year component of the date.
    ///
    /// ### Description
    /// This method extracts and returns the year as an `i32` value.
    ///
    /// ### Returns
    /// An `i32` representing the year of the date.
    ///
    /// ### Examples
    /// ```rust
    /// let year = LocalDateTime::now().year();
    /// println!("Year: {}", year);
    /// ```
    pub fn year(self) -> i32 {
        self.0.year()
    }

    /// Returns the `Month` corresponding to the month component of the date.
    ///
    /// ### Returns
    /// A `Month` corresponding to the month component of the date.
    ///
    /// ### Example
    /// ```rust
    /// let month = LocalDateTime::now().month();
    /// println!("Month: {}", month);
    /// ```
    pub fn month(self) -> Month {
        self.0.month().into()
    }

    /// Returns the numerical value associated with the month of the date.
    ///
    /// ### Returns
    /// An `i32` representing the numerical value of the month.
    ///
    /// ### Example
    /// ```rust
    /// let month_value = LocalDateTime::now().month_value();
    /// println!("Month value: {}", month_value);
    /// ```
    pub fn month_value(self) -> i32 {
        u8::from(self.0.month()) as i32
    }

    /// Returns the day of the year of the date.
    ///
    /// This method computes the ordinal day of the year (1-based) for the
    /// date encapsulated within the struct. For example, January 1st will
    /// return `1`, and December 31st will return `365` (or `366` in a leap year).
    ///
    /// ### Returns
    /// An `i32` representing the ordinal day of the year.
    ///
    /// ### Example
    /// ```rust
    /// let day_of_year = LocalDateTime::now().day_of_year();
    /// println!("Day of year: {}", day_of_year);
    /// ```
    pub fn day_of_year(self) -> i32 {
        self.0.ordinal() as i32
    }

    /// Returns the day of the month of the date.
    ///
    /// ### Returns
    /// An `i32` representing the day of the month (1-31).
    ///
    /// ### Example
    /// ```rust
    /// let day_of_month = LocalDateTime::now().day_of_month();
    /// println!("Day of month: {}", day_of_month);
    /// ```
    pub fn day_of_month(self) -> i32 {
        self.0.day() as i32
    }

    /// Returns the `DayOfWeek` of the date.
    ///
    /// ### Returns
    /// A `DayOfWeek` representing the enumerated day of the week value corresponding to the date.
    ///
    /// ### Example
    /// ```rust
    /// let day_of_week = LocalDateTime::now().day_of_week();
    /// println!("Day of week: {}", day_of_week);
    /// ```
    pub fn day_of_week(self) -> DayOfWeek {
        self.0.weekday().into()
    }

    /// Returns the number of days in the month of the date.
    ///
    /// It evaluates whether the year is a leap year and
    /// determines the correct number of days in the month.
    ///
    /// ### Returns
    /// An `i32` representing the number of days in the month.
    ///
    /// ### Example
    /// ```rust
    /// let date = LocalDateTime::now();
    /// println!("Days in month: {}", date.length_of_month());
    /// ```
    pub fn length_of_month(self) -> i32 {
        let month = self.month();
        let year = Year::of(self.year());
        month.length(year.is_leap()) as i32
    }

    /// Determines if the current year represented by the date is a leap year.
    ///
    /// ### Returns
    /// * `true` - if the year is a leap year.
    /// * `false` - if the year is not a leap year.
    ///
    /// A year is a leap year if it meets the following conditions:
    /// - It is evenly divisible by 4.
    /// - It is not evenly divisible by 100 unless it is also divisible by 400.
    ///
    /// ### Example
    /// ```rust
    /// assert!(LocalDateTime::of(2024, 3, 26, 0, 0, 0).is_leap_year()); // 2024 is a leap year.
    /// assert!(!LocalDateTime::of(2023, 3, 26, 0, 0, 0).is_leap_year()); // 2023 is not a leap year.
    /// ```
    pub fn is_leap_year(self) -> bool {
        Year::of(self.year()).is_leap()
    }

    /// Returns the number of days in the year of the date.
    ///
    /// ### Returns
    /// An `i32` representing the number of days in the year.
    ///
    /// For common years, this typically returns 365. For leap years, it returns 366.
    ///
    /// ### Examples
    /// ```rust
    /// let date = LocalDateTime::now();
    /// let year = date.year();
    /// let days_in_year = date.length_of_year();
    /// println!("The year {} has {} days.", year, days_in_year);
    /// ```
    pub fn length_of_year(self) -> i32 {
        Year::of(self.year()).length()
    }

    /// Returns the hour component of the time.
    ///
    /// ### Returns
    /// An `i32` representing the hour of the time.
    ///
    /// ### Example
    /// ```rust
    /// let hour = LocalDateTime::now().hour();
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
    /// let minute = LocalDateTime::now().minute();
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
    /// let second = LocalDateTime::now().second();
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
    /// let time = LocalDateTime::now()
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
    /// let nanoseconds = LocalDateTime::now().nanosecond();
    /// println!("Nanoseconds value: {}", nanoseconds);
    /// ```
    pub fn nanosecond(self) -> i32 {
        self.0.nanosecond() as i32
    }

    pub fn plus_years(self, years: i64) -> Self {
        let date: LocalDate = LocalDate::from(self.0.date());
        let time: LocalTime = LocalTime::from(self.0.time());
        let new_date = date.plus_years(years);
        LocalDateTime::of_date_time(new_date, time)
    }

    pub fn plus_months(self, months: i32) -> Self {
        let date: LocalDate = LocalDate::from(self.0.date());
        let time: LocalTime = LocalTime::from(self.0.time());
        let new_date = date.plus_months(months);
        LocalDateTime::of_date_time(new_date, time)
    }

    pub fn plus_weeks(self, weeks: i64) -> Self {
        Self(self.0.saturating_add(Duration::of_weeks(weeks).inner()))
    }

    pub fn plus_days(self, days: i64) -> Self {
        Self(self.0.saturating_add(Duration::of_days(days).inner()))
    }

    pub fn plus_hours(self, hours: i64) -> Self {
        Self(self.0.saturating_add(Duration::of_hours(hours).inner()))
    }

    pub fn plus_minutes(self, minutes: i64) -> Self {
        Self(self.0.saturating_add(Duration::of_minutes(minutes).inner()))
    }

    pub fn plus_seconds(self, seconds: i64) -> Self {
        Self(self.0.saturating_add(Duration::of_seconds(seconds).inner()))
    }

    pub fn plus_milliseconds(self, milliseconds: i64) -> Self {
        Self(
            self.0
                .saturating_add(Duration::of_milliseconds(milliseconds).inner()),
        )
    }

    pub fn plus_nanoseconds(self, nanoseconds: i64) -> Self {
        Self(
            self.0
                .saturating_add(Duration::of_nanoseconds(nanoseconds).inner()),
        )
    }

    pub fn minus_years(self, years: i64) -> Self {
        let date: LocalDate = LocalDate::from(self.0.date());
        let time: LocalTime = LocalTime::from(self.0.time());
        let new_date = date.minus_years(years);
        LocalDateTime::of_date_time(new_date, time)
    }

    pub fn minus_months(self, months: i32) -> Self {
        let date: LocalDate = LocalDate::from(self.0.date());
        let time: LocalTime = LocalTime::from(self.0.time());
        let new_date = date.minus_months(months);
        LocalDateTime::of_date_time(new_date, time)
    }

    pub fn minus_weeks(self, weeks: i64) -> Self {
        Self(self.0.saturating_sub(Duration::of_weeks(weeks).inner()))
    }

    pub fn minus_days(self, days: i64) -> Self {
        Self(self.0.saturating_sub(Duration::of_days(days).inner()))
    }

    pub fn minus_hours(self, hours: i64) -> Self {
        Self(self.0.saturating_sub(Duration::of_hours(hours).inner()))
    }

    pub fn minus_minutes(self, minutes: i64) -> Self {
        Self(self.0.saturating_sub(Duration::of_minutes(minutes).inner()))
    }

    pub fn minus_seconds(self, seconds: i64) -> Self {
        Self(self.0.saturating_sub(Duration::of_seconds(seconds).inner()))
    }

    pub fn minus_milliseconds(self, milliseconds: i64) -> Self {
        Self(
            self.0
                .saturating_sub(Duration::of_milliseconds(milliseconds).inner()),
        )
    }

    pub fn minus_nanoseconds(self, nanoseconds: i64) -> Self {
        Self(
            self.0
                .saturating_sub(Duration::of_nanoseconds(nanoseconds).inner()),
        )
    }

    pub fn with_year(self, year: i32) -> Self {
        Self(self.0.replace_year(year).expect("invalid year"))
    }

    pub fn with_month(self, month: i32) -> Self {
        Self(
            self.0
                .replace_month(Month::of(month).into())
                .expect("invalid month"),
        )
    }

    pub fn with_day_of_year(self, day_of_year: u16) -> Self {
        Self(
            self.0
                .replace_ordinal(day_of_year)
                .expect("invalid day of year"),
        )
    }

    pub fn with_day_of_month(self, day: u8) -> Self {
        Self(self.0.replace_day(day).expect("invalid day"))
    }

    pub fn with_hour(self, hour: u8) -> Self {
        Self(self.0.replace_hour(hour).expect("invalid hour"))
    }

    pub fn with_minute(self, minute: u8) -> Self {
        Self(self.0.replace_minute(minute).expect("invalid minute"))
    }

    pub fn with_second(self, second: u8) -> Self {
        Self(self.0.replace_second(second).expect("invalid second"))
    }

    pub fn with_millisecond(self, millisecond: u16) -> Self {
        Self(
            self.0
                .replace_millisecond(millisecond)
                .expect("invalid millisecond"),
        )
    }

    pub fn with_nanosecond(self, nanosecond: u32) -> Self {
        Self(
            self.0
                .replace_nanosecond(nanosecond)
                .expect("invalid nanosecond"),
        )
    }

    pub(crate) fn from(pdt: time::PrimitiveDateTime) -> Self {
        Self(pdt)
    }

    pub(crate) fn inner(self) -> time::PrimitiveDateTime {
        self.0
    }
}

impl core::ops::Sub for LocalDateTime {
    type Output = Duration;
    fn sub(self, rhs: Self) -> Self::Output {
        let duration: time::Duration = self.0 - rhs.0;
        Duration::from(duration)
    }
}

impl TemporalInstant for LocalDateTime {
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

impl fmt::Display for LocalDateTime {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
