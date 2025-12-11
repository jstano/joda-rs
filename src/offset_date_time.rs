use crate::{Clock, DayOfWeek, Duration, Instant, LocalDate, LocalDateTime, LocalTime, Month, TemporalInstant, Year, ZoneOffset};
use std::fmt;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(transparent))]
pub struct OffsetDateTime(time::OffsetDateTime);

impl OffsetDateTime {
    /// Returns the current UTC datetime wrapped in a custom `OffsetDateTime` type.
    ///
    /// # Examples
    /// ```rust
    /// let current_time = OffsetDateTime::now_utc();
    /// println!("Current time: {:?}", current_time);
    /// ```
    pub fn now_utc() -> Self {
        Instant::now().at_offset(ZoneOffset::UTC)
    }

    /// Returns the current type at the specified `ZoneOffset`.
    ///
    /// # Examples
    /// ```rust
    /// let current_time = OffsetDateTime::now_offset(ZoneOffset::of_hours(8));
    /// println!("Current time: {:?}", current_time);
    /// ```
    pub fn now_with_offset(offset: ZoneOffset) -> Self {
        Self(time::OffsetDateTime::now_utc().to_offset(offset.into()))
    }

    /// Returns the current time as an instance of `Self` using the provided `Clock`.
    ///
    /// This function takes a `Clock` reference and retrieves the current instant
    /// from the clock. The instant is then converted to a date-time value at the
    /// UTC offset (ZoneOffset of 0 hours).
    ///
    /// # Parameters
    /// * `clock` - A `Clock` instance used to fetch the current time.
    ///
    /// # Returns
    /// A `Self` instance representing the current time at UTC.
    ///
    /// # Example
    /// ```rust
    /// let clock = Clock::new();
    /// let current_time = joda_rs::now_with_clock(clock);
    /// println!("Current time: {:?}", current_time);
    /// ```
    pub fn now_with_clock(clock: &Clock) -> Self {
        clock.instant().at_offset(ZoneOffset::of_hours(0))
    }

    /// Constructs a new `OffsetDateTime` instance from a `LocalDateTime` in the UTC zone.
    ///
    /// # Parameters
    /// - `ldt`: A `LocalDateTime` representing the local date and time without any associated time zone offset.
    ///
    /// # Returns
    /// A new `OffsetDateTime` instance that combines the provided `LocalDateTime` in the UTC zone.
    ///
    /// # Example
    /// ```rust
    /// let ldt = LocalDateTime::of(2025, 10, 6, 8, 30, 45);
    /// let zone_offset = ZoneOffset::new_utc(ldt);
    /// let offset_date_time = OffsetDateTime::of(local_date_time, zone_offset);
    /// ```
    pub fn new_utc(ldt: LocalDateTime) -> Self {
        let pdt: time::PrimitiveDateTime = ldt.inner();
        let off: time::UtcOffset = ZoneOffset::UTC.inner();
        Self(pdt.assume_offset(off))
    }

    /// Constructs a new `OffsetDateTime` instance from a `LocalDateTime` and a `ZoneOffset`.
    ///
    /// # Parameters
    /// - `ldt`: A `LocalDateTime` representing the local date and time without any associated time zone offset.
    /// - `offset`: A `ZoneOffset` representing the fixed offset from UTC that should be applied to the given `LocalDateTime`.
    ///
    /// # Returns
    /// A new `OffsetDateTime` instance that combines the provided `LocalDateTime` with the specified `ZoneOffset`.
    ///
    /// # Example
    /// ```rust
    /// let ldt = LocalDateTime::of(2025, 10, 6, 8, 30, 45);
    /// let zone_offset = ZoneOffset::of(ldt, ZoneOffset::UTC);
    ///
    /// let offset_date_time = OffsetDateTime::of(local_date_time, zone_offset);
    /// ```
    ///
    /// # Notes
    /// - The `assume_offset` method is used internally to assign the provided offset to the `PrimitiveDateTime`.
    /// - It is the caller's responsibility to ensure that the combination of `LocalDateTime` and `ZoneOffset` represents a valid `OffsetDateTime`.
    pub fn new(ldt: LocalDateTime, offset: ZoneOffset) -> Self {
        let pdt: time::PrimitiveDateTime = ldt.inner();
        let off: time::UtcOffset = offset.inner();
        Self(pdt.assume_offset(off))
    }

    /// Constructs a new `OffsetDateTime` instance from a `LocalDateTime` and a `ZoneOffset`.
    ///
    /// # Parameters
    /// - `ldt`: A `LocalDateTime` representing the local date and time without any associated time zone offset.
    /// - `offset`: A `ZoneOffset` representing the fixed offset from UTC that should be applied to the given `LocalDateTime`.
    ///
    /// # Returns
    /// A new `OffsetDateTime` instance that combines the provided `LocalDateTime` with the specified `ZoneOffset`.
    ///
    /// # Example
    /// ```
    /// use joda_rs::{LocalDateTime, ZoneOffset, OffsetDateTime};
    ///
    /// let ldt = LocalDateTime::of(2025, 10, 6, 8, 30, 45);
    /// let zone_offset = ZoneOffset::of(ldt, ZoneOffset::UTC);
    ///
    /// let offset_date_time = OffsetDateTime::of(local_date_time, zone_offset);
    /// ```
    ///
    /// # Notes
    /// - The `assume_offset` method is used internally to assign the provided offset to the `PrimitiveDateTime`.
    /// - It is the caller's responsibility to ensure that the combination of `LocalDateTime` and `ZoneOffset` represents a valid `OffsetDateTime`.
    pub fn of(ldt: LocalDateTime, offset: ZoneOffset) -> Self {
        let pdt: time::PrimitiveDateTime = ldt.inner();
        let off: time::UtcOffset = offset.inner();
        Self(pdt.assume_offset(off))
    }

    pub fn to_instant(self) -> Instant {
        Instant::of_epoch_second(self.0.unix_timestamp())
    }

    /// Returns the offset in whole seconds from the UTC for the current instance.
    ///
    /// This method retrieves the offset of the current instance as an `i32` value,
    /// representing the total number of seconds between the local time and
    /// Coordinated Universal Time (UTC).
    ///
    /// # Return
    ///
    /// * An `i32` representing the whole seconds offset.
    ///
    /// # Example
    ///
    /// ```
    /// let offset_seconds = OffsetDateTime::now().offset();
    /// println!("Offset in seconds: {}", offset_seconds);
    /// ```
    pub fn offset(self) -> i32 {
        self.0.offset().whole_seconds()
    }

    /// Returns a new `OffsetDateTime` instance with the same point in time but using the specified `ZoneOffset`.
    ///
    /// # Parameters
    /// - `offset`: The desired `ZoneOffset` to be applied to the returned `OffsetDateTime`.
    ///
    /// # Returns
    /// An `OffsetDateTime` that represents the same instant in time as the current instance, but with the specified offset.
    ///
    /// # Example
    /// ```
    /// let original_date_time = OffsetDateTime::now_utc();
    /// let new_offset = ZoneOffset::utc_offset(3600); // Offset representing +01:00
    /// let adjusted_date_time = original_date_time.with_offset_same_instant(new_offset);
    ///
    /// assert_eq!(original_date_time.epoch_seconds(), adjusted_date_time.epoch_seconds());
    /// assert_ne!(original_date_time.offset(), adjusted_date_time.offset());
    /// ```
    ///
    /// # Notes
    /// - This method does not alter the instant in time but simply changes the associated offset.
    /// - Use this method when you want to represent the same physical moment in time with a different `ZoneOffset`.
    pub fn with_offset_same_instant(self, offset: ZoneOffset) -> OffsetDateTime {
        OffsetDateTime::from(self.0.to_offset(offset.inner()))
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
    /// let datetime1 = OffsetDateTime::now_utc();
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
    /// let datetime1 = OffsetDateTime::now_utc();
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
    /// let datetime1 = OffsetDateTime::now_utc();
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
    /// let datetime1 = OffsetDateTime::now_utc();
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

    /// Converts the current instance into a `LocalDate`.
    ///
    /// ### Returns
    /// - A `LocalDate` that represents the date portion of the current instance.
    ///
    /// ### Example
    /// ```rust
    /// let date_time = OffsetDateTime::now_utc();
    /// let local_date = date_time.to_local_date();
    /// println!("{}", local_date);
    /// ```
    pub fn to_local_date(self) -> LocalDate {
        LocalDate::from(self.0.date())
    }

    /// Converts the current instance into a `LocalTime`.
    ///
    /// ### Returns
    /// - A `LocalTime` that represents the time portion of the current instance.
    ///
    /// ### Example
    /// ```rust
    /// let date_time = OffsetDateTime::now_utc();
    /// let local_time = date_time.to_local_time();
    /// println!("{}", local_time);
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
    /// let date_time = OffsetDateTime::now_utc();
    /// let local_date_time = date_time.to_local_date_time();
    /// println!("{}", local_date_time);
    /// ```
    pub fn to_local_date_time(self) -> LocalDateTime {
        LocalDateTime::from(time::PrimitiveDateTime::new(self.0.date(), self.0.time()))
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
    /// let epoch_seconds = OffsetDateTime.now_utc().epoch_seconds();
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
    /// let nanoseconds = OffsetDateTime.now_utc().epoch_nanoseconds();
    /// println!("Epoch time in nanoseconds: {}", nanoseconds);
    /// ```
    pub fn epoch_nanoseconds(self) -> i128 {
        self.0.unix_timestamp_nanos()
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
    /// let year = OffsetDateTime::now_utc().year();
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
    /// let month = OffsetDateTime::now_utc().month();
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
    /// let month_value = OffsetDateTime::now_utc().month_value();
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
    /// let day_of_year = OffsetDateTime::now_utc().day_of_year();
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
    /// let day_of_month = OffsetDateTime::now_utc().day_of_month();
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
    /// let day_of_week = OffsetDateTime::now_utc().day_of_week();
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
    /// let date = ZonedDateTime::now_utc();
    /// println!("Days in month: {}", date.length_of_month());
    /// ```
    pub fn length_of_month(self) -> i32 {
        self.0.month().length(self.0.year()) as i32
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
    /// assert!(OffsetDateTime::of(2024, 3, 26, 0, 0, 0).is_leap_year()); // 2024 is a leap year.
    /// assert!(!OffsetDateTime::of(2023, 3, 26, 0, 0, 0).is_leap_year()); // 2023 is not a leap year.
    /// ```
    pub fn is_leap_year(self) -> bool {
        Year::of(self.0.year()).is_leap()
    }

    /// Returns the number of days in the year of the date.
    ///
    /// ### Returns
    /// An `i32` representing the number of days in the year.
    /// For common years, this typically returns 365. For leap years, it returns 366.
    ///
    /// ### Examples
    /// ```rust
    /// let date = OffsetDateTime::now_utc();
    /// let year = date.year();
    /// let days_in_year = date.length_of_year();
    /// println!("The year {} has {} days.", year, days_in_year);
    /// ```
    pub fn length_of_year(self) -> i32 {
        Year::of(self.0.year()).length()
    }

    /// Returns the hour component of the time.
    ///
    /// ### Returns
    /// An `i32` representing the hour of the time.
    ///
    /// ### Example
    /// ```rust
    /// let hour = OffsetDateTime::now_utc().hour();
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
    /// let minute = OffsetDateTime::now_utc().minute();
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
    /// let second = OffsetDateTime::now_utc().second();
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
    /// let time = OffsetDateTime::now_utc()
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
    /// let nanoseconds = OffsetDateTime::now_utc().nanosecond();
    /// println!("Nanoseconds value: {}", nanoseconds);
    /// ```
    pub fn nanosecond(self) -> i32 {
        self.0.nanosecond() as i32
    }

    pub fn plus_years(self, years: i64) -> Self {
        let date = self.0.date();
        let time = self.0.time();

        // adjust year, preserving month/day as much as possible
        let new_year = date.year().checked_add(years as i32)
            .expect("year overflow in plus_years");

        // clamp invalid dates (e.g., Feb 29 on non-leap year)
        let new_date = time::Date::from_calendar_date(
            new_year,
            date.month(),
            date.day().min(date.month().length(new_year))
        ).unwrap();

        let pdt = time::PrimitiveDateTime::new(new_date, time);
        Self(pdt.assume_offset(self.0.offset()))
    }

    pub fn plus_months(self, months: i64) -> Self {
        let date = self.0.date();
        let time = self.0.time();
        let offset = self.0.offset();

        // Convert the current date to total months since year 0
        let total_months = date.year() as i64 * 12 + (date.month() as i64 - 1) + months;
        let new_year = (total_months / 12) as i32;
        let new_month_index = (total_months % 12 + 12) % 12 + 1; // Month index in 1..=12
        let new_month = time::Month::try_from(new_month_index as u8).expect("invalid month index");

        // Clamp the day to the maximum day of the new month
        let new_day = std::cmp::min(date.day(), new_month.length(new_year));

        let new_date = time::Date::from_calendar_date(new_year, new_month, new_day)
            .expect("valid date guaranteed by clamping");

        let pdt = time::PrimitiveDateTime::new(new_date, time);
        Self(pdt.assume_offset(offset))
    }

    pub fn plus_weeks(self, weeks: i64) -> Self {
        Self(self.0.checked_add(Duration::of_weeks(weeks).inner()).expect("Date overflow in plus_weeks"))
    }

    pub fn plus_days(self, days: i64) -> Self {
        Self(self.0.checked_add(Duration::of_days(days).inner()).expect("Date overflow in plus_days"))
    }

    pub fn plus_hours(self, hours: i64) -> Self {
        Self(self.0.checked_add(Duration::of_hours(hours).inner()).expect("Date overflow in plus_hours"))
    }

    pub fn plus_minutes(self, minutes: i64) -> Self {
        Self(self.0.checked_add(Duration::of_minutes(minutes).inner()).expect("Date overflow in plus_minutes"))
    }

    pub fn plus_seconds(self, seconds: i64) -> Self {
        Self(self.0.checked_add(Duration::of_seconds(seconds).inner()).expect("Date overflow in plus_seconds"))
    }

    pub fn plus_milliseconds(self, milliseconds: i64) -> Self {
        Self(self.0.checked_add(Duration::of_milliseconds(milliseconds).inner()).expect("Date overflow in plus_milliseconds"))
    }

    pub fn plus_nanoseconds(self, nanoseconds: i64) -> Self {
        Self(self.0.checked_add(Duration::of_nanoseconds(nanoseconds).inner()).expect("Date overflow in plus_nanoseconds"))
    }

    pub fn minus_years(self, years: i64) -> Self {
        self.plus_years(-years)
    }

    pub fn minus_months(self, months: i64) -> Self {
        self.plus_months(-months)
    }

    pub fn minus_weeks(self, weeks: i64) -> Self {
        Self(self.0.checked_sub(Duration::of_weeks(weeks).inner()).expect("Date overflow in minus_weeks"))
    }

    pub fn minus_days(self, days: i64) -> Self {
        Self(self.0.checked_sub(Duration::of_days(days).inner()).expect("Date overflow in minus_days"))
    }

    pub fn minus_hours(self, hours: i64) -> Self {
        Self(self.0.checked_sub(Duration::of_hours(hours).inner()).expect("Date overflow in minus_hours"))
    }

    pub fn minus_minutes(self, minutes: i64) -> Self {
        Self(self.0.checked_sub(Duration::of_minutes(minutes).inner()).expect("Date overflow in minus_minutes"))
    }

    pub fn minus_seconds(self, seconds: i64) -> Self {
        Self(self.0.checked_sub(Duration::of_seconds(seconds).inner()).expect("Date overflow in minus_seconds"))
    }

    pub fn minus_milliseconds(self, milliseconds: i64) -> Self {
        Self(self.0.checked_sub(Duration::of_milliseconds(milliseconds).inner()).expect("Date overflow in minus_milliseconds"))
    }

    pub fn minus_nanoseconds(self, nanoseconds: i64) -> Self {
        Self(self.0.checked_sub(Duration::of_nanoseconds(nanoseconds).inner()).expect("Date overflow in minus_nanoseconds"))
    }

    pub fn with_year(self, year: i32) -> Self {
        Self(self.0.replace_year(year).expect("invalid year"))
    }

    pub fn with_month(self, month: i32) -> Self {
        Self(self.0.replace_month(Month::of(month).into()).expect("invalid month"))
    }

    pub fn with_day_of_year(self, day_of_year: u16) -> Self {
        Self(self.0.replace_ordinal(day_of_year).expect("invalid day of year"))
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
        Self(self.0.replace_millisecond(millisecond).expect("invalid millisecond"))
    }

    pub fn with_nanosecond(self, nanosecond: u32) -> Self {
        Self(self.0.replace_nanosecond(nanosecond).expect("invalid nanosecond"))
    }

    pub(crate) fn from(inner: time::OffsetDateTime) -> Self {
        Self(inner)
    }

    pub(crate) fn inner(self) -> time::OffsetDateTime {
        self.0
    }
}

impl TemporalInstant for OffsetDateTime {
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

impl fmt::Display for OffsetDateTime {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
