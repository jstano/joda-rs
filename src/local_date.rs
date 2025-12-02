use crate::{Clock, DayOfWeek, Duration, Instant, LocalDateTime, LocalTime, Month, Year, ZoneId};
use std::fmt;
use time::UtcOffset;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct LocalDate(time::Date);

impl LocalDate {
    pub fn now() -> Self {
        Self(time::OffsetDateTime::now_utc().to_offset(UtcOffset::current_local_offset().unwrap()).date())
    }

    pub fn now_with_clock(clock: &Clock) -> Self {
        clock.instant().at_zone(clock.zone()).to_local_date()
    }

    pub fn now_with_zone(zone: ZoneId) -> Self {
        Instant::now().at_zone(zone).to_local_date()
    }

    pub fn new(year: i32, month: i32, day: i32) -> Self {
        let m = time::Month::try_from(month as u8).expect("invalid month 1-12");
        let d = time::Date::from_calendar_date(year, m, day as u8).expect("invalid date");
        Self(d)
    }

    pub fn of(year: i32, month: i32, day: i32) -> Self {
        Self::new(year, month, day)
    }

    pub fn of_epoch_day(days_since_epoch: i64) -> Self {
        let odt = time::OffsetDateTime::UNIX_EPOCH + time::Duration::days(days_since_epoch);
        Self(odt.date())
    }

    pub fn of_year_day(year: i32, day_of_year: i32) -> Self {
        let date = time::Date::from_ordinal_date(year, day_of_year as u16)
            .expect("invalid day of year");
        Self(date)
    }

    pub fn last_day_of_month_year(year: i32, month: i32) -> Self {
        let m = Month::of(month);
        let leap = Year::of(year).is_leap();
        let last = m.length(leap);
        let tm: time::Month = m.into();
        let d = time::Date::from_calendar_date(year, tm, last as u8)
            .expect("invalid date for last_day_of_month_year");
        Self(d)
    }

    pub fn parse(s: &str) -> Self {
        use time::format_description::well_known::Iso8601;
        Self(time::Date::parse(s, &Iso8601::DEFAULT).expect("invalid date string"))
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

    /// Returns the year component of the date.
    ///
    /// ### Returns
    /// An `i32` representing the year of the date.
    ///
    /// ### Examples
    /// ```rust
    /// let year = LocalDate::now().year();
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
    /// let month = LocalDate::now().month();
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
    /// let month_value = LocalDate::now().month_value();
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
    /// let day_of_year = LocalDate::now().day_of_year();
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
    /// let day_of_month = LocalDate::now().day_of_month();
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
    /// let day_of_week = LocalDate::now().day_of_week();
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
    /// let date = LocalDate::now();
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
    /// assert!(LocalDate::of(2024, 3, 26).is_leap_year()); // 2024 is a leap year.
    /// assert!(!LocalDate::of(2023, 3, 26).is_leap_year()); // 2023 is not a leap year.
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
    /// let date = LocalDate::of(2024, 3, 26);
    /// let year = date.year();
    /// let days_in_year = date.length_of_year();
    /// println!("The year {} has {} days.", year, days_in_year);
    /// ```
    pub fn length_of_year(self) -> i32 {
        Year::of(self.year()).length()
    }

    /// Adds the specified number of years to the current `OffsetDateTime`, returning a new `OffsetDateTime` instance.
    ///
    /// ### Arguments
    ///
    /// * `years` - An `i64` value indicating the number of years to add. This value
    ///   can be positive or negative, allowing for both addition and subtraction.
    ///
    /// ### Returns
    ///
    /// * A new `OffsetDateTime` instance that represents the time resulting from
    ///   adding the specified years to the original instance.
    ///
    /// ### Examples
    ///
    /// ```
    /// use joda_rs::OffsetDateTime;
    ///
    /// let datetime = OffsetDateTime::now_utc();
    /// let updated_datetime = datetime.plus_years(5);
    /// println!("Original: {}, Updated: {}", datetime, updated_datetime);
    /// ```
    ///
    /// ### Panics
    /// This function will panic if the addition causes an overflow or underflow in the internal timestamp representation.
    pub fn plus_years(self, years: i64) -> Self {
        let target_year_i64 = self.0.year() as i64 + years;
        if target_year_i64 > i32::MAX as i64 || target_year_i64 < i32::MIN as i64 {
            panic!("year overflow")
        }
        let new_year = target_year_i64 as i32;
        let month = self.0.month();
        let mut day = self.0.day();
        // Clamp Feb 29 to Feb 28 on non-leap years
        if month == time::Month::February && day == 29 && !time::util::is_leap_year(new_year) {
            day = 28;
        }
        let new_date = time::Date::from_calendar_date(new_year, month, day).expect("invalid date");
        Self(new_date)
    }

    /// Adds the specified number of months to the current `OffsetDateTime`, returning a new `OffsetDateTime` instance.
    ///
    /// ### Arguments
    ///
    /// * `months` - An `i64` value indicating the number of months to add. This value
    ///   can be positive or negative, allowing for both addition and subtraction.
    ///
    /// ### Returns
    ///
    /// * A new `OffsetDateTime` instance that represents the time resulting from
    ///   adding the specified months to the original instance.
    ///
    /// ### Examples
    ///
    /// ```
    /// use joda_rs::OffsetDateTime;
    ///
    /// let datetime = OffsetDateTime::now_utc();
    /// let updated_datetime = datetime.plus_months(5);
    /// println!("Original: {}, Updated: {}", datetime, updated_datetime);
    /// ```
    ///
    /// ### Panics
    /// This function will panic if the addition causes an overflow or underflow in the internal timestamp representation.
    pub fn plus_months(self, months: i32) -> Self {
        let mut year = self.year();
        let mut month = self.month_value() + months;

        // handle overflow of months
        while month > 12 {
            month -= 12;
            year += 1;
        }
        while month < 1 {
            month += 12;
            year -= 1;
        }

        // clamp day to the last day of the new month
        let last_day = Month::of(month).length(Year::of(year).is_leap());
        let day = self.day_of_month().min(last_day);

        LocalDate::new(year, month, day)
    }

    /// Adds the specified number of weeks to the current `OffsetDateTime`, returning a new `OffsetDateTime` instance.
    ///
    /// ### Arguments
    ///
    ///
    /// * `weeks` - An `i64` value indicating the number of weeks to add. This value
    ///   can be positive or negative, allowing for both addition and subtraction.
    ///
    /// ### Returns
    ///
    /// * A new `OffsetDateTime` instance that represents the time resulting from
    ///   adding the specified weeks to the original instance.
    ///
    /// ### Examples
    ///
    /// ```
    /// use joda_rs::OffsetDateTime;
    ///
    /// let datetime = OffsetDateTime::now_utc();
    /// let updated_datetime = datetime.plus_weeks(5);
    /// println!("Original: {}, Updated: {}", datetime, updated_datetime);
    /// ```
    ///
    /// ### Panics
    /// This function will panic if the addition causes an overflow or underflow in the internal timestamp representation.
    pub fn plus_weeks(self, weeks: i64) -> Self {
        Self(self.0.checked_add(Duration::of_weeks(weeks).inner()).expect("Date overflow in plus_weeks"))
    }

    /// Adds the specified number of days to the current `OffsetDateTime`, returning a new `OffsetDateTime` instance.
    ///
    /// ### Arguments
    ///
    /// * `days` - An `i64` value indicating the number of days to add. This value
    ///   can be positive or negative, allowing for both addition and subtraction.
    ///
    /// ### Returns
    ///
    /// * A new `OffsetDateTime` instance that represents the time resulting from
    ///   adding the specified days to the original instance.
    ///
    /// ### Examples
    ///
    /// ```
    /// use joda_rs::OffsetDateTime;
    ///
    /// let datetime = OffsetDateTime::now_utc();
    /// let updated_datetime = datetime.plus_days(5);
    /// println!("Original: {}, Updated: {}", datetime, updated_datetime);
    /// ```
    ///
    /// ### Panics
    /// This function will panic if the addition causes an overflow or underflow in the internal timestamp representation.
    pub fn plus_days(self, days: i64) -> Self {
        Self(self.0.checked_add(Duration::of_days(days).inner()).expect("Date overflow in plus_days"))
    }

    /// Subtracts the specified number of years from the current `OffsetDateTime`, returning a new `OffsetDateTime` instance.
    ///
    /// ### Arguments
    ///
    /// * `years` - An `i64` value indicating the number of years to subtract. This value
    ///   can be positive or negative, allowing for both addition and subtraction.
    ///
    /// ### Returns
    ///
    /// * A new `OffsetDateTime` instance that represents the time resulting from
    ///   subtracting the specified years from the original instance.
    ///
    /// ### Examples
    ///
    /// ```
    /// use joda_rs::OffsetDateTime;
    ///
    /// let datetime = OffsetDateTime::now_utc();
    /// let updated_datetime = datetime.minus_years(5);
    /// println!("Original: {}, Updated: {}", datetime, updated_datetime);
    /// ```
    ///
    /// ### Panics
    /// This function will panic if the subtraction causes an overflow or underflow in the internal timestamp representation.
    pub fn minus_years(self, years: i64) -> Self {
        self.plus_years(-years)
    }

    /// Subtracts the specified number of months from the current `OffsetDateTime`, returning a new `OffsetDateTime` instance.
    ///
    /// ### Arguments
    ///
    /// * `months` - An `i64` value indicating the number of months to subtract. This value
    ///   can be positive or negative, allowing for both addition and subtraction.
    ///
    /// ### Returns
    ///
    /// * A new `OffsetDateTime` instance that represents the time resulting from
    ///   subtracting the specified months from the original instance.
    ///
    /// ### Examples
    ///
    /// ```
    /// use joda_rs::OffsetDateTime;
    ///
    /// let datetime = OffsetDateTime::now_utc();
    /// let updated_datetime = datetime.minus_months(5);
    /// println!("Original: {}, Updated: {}", datetime, updated_datetime);
    /// ```
    ///
    /// ### Panics
    /// This function will panic if the subtraction causes an overflow or underflow in the internal timestamp representation.
    pub fn minus_months(self, months: i32) -> Self  {
        let total_months = (self.year() * 12 + (self.month_value() as i32 - 1)) - months;
        let year = total_months / 12;
        let month = (total_months % 12 + 12) % 12 + 1; // ensure 1..=12
        let last_day = Month::of(month).length(Year::of(year).is_leap());
        let day = self.day_of_month().min(last_day);

        LocalDate::new(year, month, day)
    }

    /// Subtracts the specified number of weeks from the current `OffsetDateTime`, returning a new `OffsetDateTime` instance.
    ///
    /// ### Arguments
    ///
    /// * `weeks` - An `i64` value indicating the number of weeks to subtract. This value
    ///   can be positive or negative, allowing for both addition and subtraction.
    ///
    /// ### Returns
    ///
    /// * A new `OffsetDateTime` instance that represents the time resulting from
    ///   subtracting the specified weeks from the original instance.
    ///
    /// ### Examples
    ///
    /// ```
    /// use joda_rs::OffsetDateTime;
    ///
    /// let datetime = OffsetDateTime::now_utc();
    /// let updated_datetime = datetime.minus_weeks(5);
    /// println!("Original: {}, Updated: {}", datetime, updated_datetime);
    /// ```
    ///
    /// ### Panics
    /// This function will panic if the subtraction causes an overflow or underflow in the internal timestamp representation.
    pub fn minus_weeks(self, weeks: i64) -> Self {
        Self(self.0.checked_sub(Duration::of_weeks(weeks).inner()).expect("Date overflow in minus_weeks"))
    }

    /// Subtracts the specified number of days from the current `OffsetDateTime`, returning a new `OffsetDateTime` instance.
    ///
    /// ### Arguments
    ///
    /// * `days` - An `i64` value indicating the number of days to subtract. This value
    ///   can be positive or negative, allowing for both addition and subtraction.
    ///
    /// ### Returns
    ///
    /// * A new `OffsetDateTime` instance that represents the time resulting from
    ///   subtracting the specified days from the original instance.
    ///
    /// ### Examples
    ///
    /// ```
    /// use joda_rs::OffsetDateTime;
    ///
    /// let datetime = OffsetDateTime::now_utc();
    /// let updated_datetime = datetime.minus_days(5);
    /// println!("Original: {}, Updated: {}", datetime, updated_datetime);
    /// ```
    ///
    /// ### Panics
    /// This function will panic if the subtraction causes an overflow or underflow in the internal timestamp representation.
    pub fn minus_days(self, days: i64) -> Self {
        Self(self.0.checked_sub(Duration::of_days(days).inner()).expect("Date overflow in minus_days"))
    }

    /// Sets the year component of the date or time instance to the given value and returns a new instance with the updated year.
    ///
    /// # Parameters
    /// - `year`: An `i32` representing the year to set in the date or time instance.
    ///
    /// # Returns
    /// - Returns a new instance of `Self` (the same type as the calling object) with the year updated to the provided value.
    ///
    /// # Example
    /// ```rust
    /// let date = SomeDateType::new(2023, 7, 15); // Assume SomeDateType has a constructor
    /// let updated_date = date.with_year(2025);
    /// assert_eq!(updated_date.year(), 2025);
    /// ```
    ///
    /// # Notes
    /// - This method does not mutate the original instance. It creates and returns a new instance with the updated year.
    /// - If the type enforces valid ranges for years, passing an out-of-range value may result in a panic or an error, depending on the implementation.
    ///
    /// # Safety
    /// Users are responsible for ensuring the input year value is valid for the implementation.
    pub fn with_year(self, year: i32) -> Self {
        Self(self.0.replace_year(year).expect("Invalid year"))
    }

    /// Sets the month of the specified date or time representation to the given value.
    ///
    /// # Parameters
    /// - `month`: An `i32` representing the month to set, where valid values typically range
    ///   from 1 (January) to 12 (December).
    ///
    /// # Returns
    /// - Returns a new instance of the calling object (`Self`) with the updated month value.
    ///
    /// # Panics
    /// - The method may panic if the `month` value is out of the valid range (e.g., less than 1 or greater than 12),
    ///   depending on how the method is implemented.
    ///
    /// # Example
    /// ```
    /// # use some_module::YourType;
    /// let original_date = YourType::new(2023, 1, 15); // Example initialization
    /// let updated_date = original_date.with_month(7); // Changes the month to July
    /// assert_eq!(updated_date.month(), 7);
    /// ```
    pub fn with_month(self, month: i32) -> Self {
        Self(self.0.replace_month(time::Month::try_from(month as u8).expect("Invalid month")).expect("Invalid month"))
    }

    /// Returns a new instance of the object with the day of the year updated to the specified value.
    ///
    /// This method does not modify the original instance. Instead, it creates a copy
    /// of the instance with the `day_of_year` property set to the given value.
    ///
    /// # Parameters
    /// - `day_of_year` (`u16`): The day of the year to set, where `1` represents
    ///   January 1st and values must be within the valid range for the specific calendar context (e.g., 1 to 365 or 1 to 366 for leap years).
    ///
    /// # Returns
    /// - `Self`: A new instance with the updated day of the year.
    ///
    /// # Panics
    /// This method may panic if the provided `day_of_year` is out of the valid range
    /// for the object being manipulated (e.g., a non-existent day in the context of the year).
    ///
    /// # Examples
    /// ```
    /// use some_module::SomeStruct;
    ///
    /// let date = SomeStruct::new(/* initialize with some date values */);
    /// let updated_date = date.with_day_of_year(200);
    ///
    /// // `updated_date` now has the day of the year set to 200, while the original `date` is unchanged.
    /// ```
    pub fn with_day_of_year(self, day_of_year: u16) -> Self {
        let year = Year::of(self.0.year());

        if day_of_year == 0 || day_of_year > year.length() as u16 {
            panic!("day-of-year must be 1..=365/366")
        }
        let jan1 = time::Date::from_calendar_date(self.0.year(), time::Month::January, 1).unwrap();
        Self(jan1 + time::Duration::days((day_of_year as i64) - 1))
    }

    /// Sets the day of the month for the given date, returning a new instance with the updated value.
    ///
    /// # Parameters
    /// - `day`: The day of the month to set (1-31).
    ///          The validity of the `day` value will depend on the month and year of the date.
    ///
    /// # Returns
    /// - `Self`: A new instance with the specified day of the month set.
    ///           If the provided day is invalid for the current month and year,
    ///           the method may return an invalid or undefined state depending on implementation.
    ///
    /// # Examples
    /// ```
    /// // Assuming `SomeDate` implements this method.
    /// let date = SomeDate::new(2023, 10, 15); // 15th October 2023
    /// let updated_date = date.with_day_of_month(25); // Updates date to 25th October 2023
    /// ```
    ///
    /// # Panics
    /// This method may panic if `day` is out of valid range (e.g., 0 or greater than 31),
    /// or if the day does not exist in the context of the date's month and year.
    pub fn with_day_of_month(self, day: i32) -> Self {
        Self(self.0.replace_day(day as u8).expect("invalid day"))
    }

    pub fn at_time(self, time: LocalTime) -> LocalDateTime {
        LocalDateTime::of_date_time(self, time)
    }

    pub fn at_start_of_day(self) -> LocalDateTime {
        let midnight = LocalTime::of(0, 0, 0);
        LocalDateTime::of_date_time(self, midnight)
    }

    /// Returns a new instance of the type with the day set to the first day of the month.
    ///
    /// This function creates a copy of the current instance and modifies its day-of-month
    /// property to the first day (`1`). The rest of the fields (e.g., month, year) remain unchanged.
    ///
    /// ### Returns
    /// A new instance of the type with the day set to `1`.
    ///
    /// ### Examples
    /// ```rust
    /// let date = LocalDate::of(2023, 10, 15);
    /// let first_day = date.first_day_of_month();
    /// assert_eq!(first_day, LocalDate::of(2023, 10, 1));
    /// ```
    pub fn first_day_of_month(self) -> Self {
        self.with_day_of_month(1)
    }

    /// Returns a new instance with the date adjusted to the last day of the current month.
    ///
    /// This method calculates the length of the current month using `length_of_month`
    /// and adjusts the day of the current date to the calculated value using `with_day_of_month`.
    ///
    /// ### Returns
    ///
    /// * `Self` - A new instance representing the date updated to the last day of the month.
    ///
    /// ### Example
    /// ```rust
    /// let date = LocalDate::of(2023, 10, 15);
    /// let first_day = date.last_day_of_month();
    /// assert_eq!(first_day, LocalDate::of(2023, 10, 31));
    /// ```
    pub fn last_day_of_month(self) -> Self {
        let len = self.length_of_month();
        self.with_day_of_month(len)
    }

    /// Returns a new instance representing the first day of the year, based on the current instance.
    ///
    /// ### Returns
    /// A new instance of the same type (`Self`) with the date set to the first day of the year.
    ///
    /// ### Example
    /// ```rust
    /// let date = LocalDate::of(2023, 10, 15);
    /// let first_day = date.first_day_of_year();
    /// assert_eq!(first_day, LocalDate::of(2023, 1, 1));
    /// ```
    pub fn first_day_of_year(self) -> Self {
        self.with_day_of_year(1)
    }

    /// Returns a new instance representing the last day of the year for the current date or time object.
    ///
    /// This method calculates the total number of days in the current year using `length_of_year()`,
    /// and then creates a new instance where the day is set to the last day of the year (i.e., the total number of days).
    ///
    /// ### Returns
    /// A new instance with the day set to the last day of the year.
    ///
    /// ### Example
    /// ```rust
    /// let date = LocalDate::of(2023, 10, 15);
    /// let first_day = date.last_day_of_year();
    /// assert_eq!(first_day, LocalDate::of(2023, 12, 31));
    /// ```
    pub fn last_day_of_year(self) -> Self {
        let len = self.length_of_year() as u16;
        self.with_day_of_year(len)
    }

    /// Determines the first occurrence of a specified day of the week (`dow`) in the current month.
    ///
    /// ### Arguments
    /// * `dow` - The desired day of the week (`DayOfWeek`) to find within the month.
    ///
    /// ### Returns
    /// * A new date representing the first occurrence of the specified `dow` in the month containing `self`.
    ///
    /// ### Description
    /// This function calculates the first occurrence of a given day of the week (`dow`)
    /// within the same month as the current date (`self`). To achieve this:
    /// 1. It retrieves the first day of the month using the `first_day_of_month` method.
    /// 2. It determines the day of the week for the first day of the month.
    /// 3. It computes the difference (`delta`) between the specified `dow` and the day of the week of the first day,
    ///    ensuring the result is within the range of 0 to 6 days forward.
    /// 4. The function then advances the first day of the month by the calculated `delta` days
    ///    and returns the resulting date.
    ///
    /// ### Assumptions
    /// * It is assumed that the `DayOfWeek` type has a `value()` method that returns an integer representation
    ///   of the day of the week (e.g., Sunday = 0, Monday = 1, etc.).
    /// * The `first_day_of_month` method returns the date object representing the first day of the month.
    /// * The `plus_days` method advances the date by the specified number of days.
    ///
    /// ### Example
    /// ```rust
    /// let date = LocalDate::of(2023, 10, 15);
    /// let desired_dow = DayOfWeek::Monday;
    /// let first_monday = date.first_in_month(desired_dow);
    /// println!("The first Monday of the month is: {}", first_monday);
    /// ```
    ///
    /// ### Notes
    /// * If the first day of the month is the same as the specified `dow`, the function will return that same date.
    /// * This function is useful for scheduling events or finding recurring days of the week within a specific month.
    pub fn first_in_month(self, dow: DayOfWeek) -> Self {
        let first = self.first_day_of_month();
        let current = first.day_of_week();
        let delta = (dow.value() - current.value() + 7) % 7; // 0..6 days forward
        first.plus_days(delta as i64)
    }

    /// Returns a new date adjusted to the last occurrence of the specified day of the week
    /// within the same month as the current date.
    ///
    /// # Parameters:
    /// - `dow` - The day of the week (`DayOfWeek`) for which the last occurrence in the month
    ///   should be found. For example, `DayOfWeek::Monday`, `DayOfWeek::Friday`, etc.
    ///
    /// # Returns:
    /// - A new date (`Self`) representing the last occurrence of the specified day of the week
    ///   in the same month as the original date.
    ///
    /// # Example:
    /// ```
    /// let date = /* Assume some date in March 2023 */;
    /// let last_friday = date.last_in_month(DayOfWeek::Friday);
    /// // `last_friday` would now be the last Friday of March 2023.
    /// ```
    ///
    /// # Implementation Details:
    /// - Calculates the last day of the current month (`last`).
    /// - Determines the day of the week of this last day (`current`).
    /// - Computes the backward difference (`delta`) between the current day of the week and the target day of the week (`dow`).
    /// - Adjusts the `last` date by subtracting `delta` days to obtain the desired last occurrence.
    ///
    /// # Assumptions:
    /// - The `self` object represents a valid date.
    /// - `DayOfWeek` provides the methods `.value()` to determine the numeric value of a day (for example, Monday = 1, Sunday = 7).
    ///
    /// # Notes:
    /// - The result is still in the same month as the original date.
    /// - If the target day is the same as the day of the week of the last day of the month, no adjustment is made
    ///   and the same `last` day is returned.
    ///
    /// # Dependencies:
    /// - The function assumes the existence of:
    ///   - `last_day_of_month()` method that returns the last date of the month.
    ///   - `day_of_week()` method that returns the day of the week for the current date.
    ///   - `minus_days()` method that subtracts a given number of days from a date and returns the updated date.
    pub fn last_in_month(self, dow: DayOfWeek) -> Self {
        let last = self.last_day_of_month();
        let current = last.day_of_week();
        let delta = (current.value() - dow.value() + 7) % 7; // 0..6 days backward
        last.minus_days(delta as i64)
    }

    /// Returns a new instance of the current object advanced to the next specified day of the week.
    ///
    /// This method calculates the smallest positive number of days needed to advance from the current day
    /// to the next occurrence of the specified `DayOfWeek`. If the current day already matches the
    /// requested `DayOfWeek`, it advances by exactly one week (7 days).
    ///
    /// # Arguments
    ///
    /// * `dow` - A `DayOfWeek` enum representing the target day of the week.
    ///
    /// # Returns
    ///
    /// A new instance of the object with its date advanced to the next occurrence of the specified `DayOfWeek`.
    ///
    /// # Example
    ///
    /// ```rust
    /// let current_date = ...; // Assume this is an instance of a date-like structure
    /// let next_monday = current_date.next(DayOfWeek::Monday);
    /// ```
    ///
    /// In the above example, if the `current_date` falls on a Tuesday, calling `next` with `DayOfWeek::Monday`
    /// will return the date for the following Monday.
    ///
    /// # Notes
    ///
    /// This method ensures a consistent progression by always moving forward in time. Even if the
    /// current day matches the target `DayOfWeek`, it will move to its next occurrence (i.e., one week later).
    ///
    /// The implementation relies on the difference between the numeric values of the current day and the target day,
    /// adjusted to provide a modulus operation in a 7-day week cycle. The logic accounts for ensuring a non-zero delta
    /// by defaulting to a full 7-day increment when the current day already matches the target day.
    pub fn next(self, dow: DayOfWeek) -> Self {
        let cur = self.day_of_week();
        let mut delta = (dow.value() - cur.value() + 7) % 7;
        if delta == 0 {
            delta = 7;
        }
        self.plus_days(delta as i64)
    }

    /// Returns the current date if it falls on the specified `DayOfWeek` or the next date
    /// that corresponds to the given `DayOfWeek`.
    ///
    /// # Arguments
    /// * `dow` - A `DayOfWeek` instance specifying the target day of the week.
    ///
    /// # Returns
    /// * `Self` - The current date if it matches the given `DayOfWeek`, or the nearest next date
    ///   that corresponds to the specified `DayOfWeek`.
    ///
    /// # Behavior
    /// * Determines the difference (`delta`) between the current day of the week and the
    ///   specified `DayOfWeek`.
    /// * If the current date is already the target day (`delta` is 0), returns the current date.
    /// * Otherwise, adds the calculated difference (`delta`) in days to the current date and
    ///   returns the resulting date.
    ///
    /// # Example
    /// ```rust
    /// use some_date_library::{Date, DayOfWeek}; // Replace with appropriate library imports
    ///
    /// let today = Date::new(2023, 10, 10); // Assume it's Tuesday
    /// let next_sunday = today.next_or_same(DayOfWeek::Sunday);
    ///
    /// assert_eq!(next_sunday, Date::new(2023, 10, 15)); // Upcoming Sunday
    /// ```
    pub fn next_or_same(self, dow: DayOfWeek) -> Self {
        let cur = self.day_of_week();
        let delta = (dow.value() - cur.value() + 7) % 7;
        self.plus_days(delta as i64)
    }

    /// Returns a new instance of the current date/time that corresponds to the most recent
    /// occurrence of the specified day of the week (`dow`) prior to (but not including) the current instance.
    ///
    /// # Parameters
    /// - `dow`: A `DayOfWeek` value representing the target day of the week (e.g., Monday, Tuesday, etc.).
    ///
    /// # Returns
    /// A new instance of `Self` adjusted to the most recent occurrence of the specified `DayOfWeek`.
    ///
    /// # Behavior
    /// - If the current instance already corresponds to the specified `DayOfWeek`,
    ///   it will roll back to the previous occurrence of that day (7 days prior).
    /// - If the target day is earlier in the week than the current day, it calculates the
    ///   offset by subtracting the day values, factoring in a modulo operation with 7 for week wrapping,
    ///   and applies the difference using `minus_days`.
    ///
    /// # Example
    /// ```rust
    /// use my_crate::{Date, DayOfWeek};
    ///
    /// let today = Date::from_ymd(2023, 10, 10); // Assume this is a Tuesday.
    /// let previous_sunday = today.previous(DayOfWeek::Sunday);
    ///
    /// assert_eq!(previous_sunday, Date::from_ymd(2023, 10, 8));
    /// ```
    ///
    /// # Notes
    /// - This function assumes that `self` is an instance of a type that implements
    ///   both `day_of_week()` (to retrieve the current day) and `minus_days()`
    ///   (to perform subtraction of days).
    /// - The calculation ensures a non-negative delta by adding 7 before performing the modulo
    ///   operation, ensuring proper week wraparound behavior for all cases.
    ///
    /// # See Also
    /// - `day_of_week`: Method used to retrieve the current day of the week.
    /// - `minus_days`: Method used to subtract days from the current instance.
    pub fn previous(self, dow: DayOfWeek) -> Self {
        let cur = self.day_of_week();
        let mut delta = (cur.value() - dow.value() + 7) % 7;
        if delta == 0 {
            delta = 7;
        }
        self.minus_days(delta as i64)
    }

    ///
    /// Returns the date that is either the same or the most recent previous occurrence
    /// of the specified day of the week (`dow`).
    ///
    /// # Parameters
    /// - `dow`: The desired day of the week (`DayOfWeek`) for which to calculate the previous or same date.
    ///
    /// # Returns
    /// - `Self`: A new instance of the date adjusted to the previous or same occurrence of the specified day of the week.
    ///
    /// # Behavior
    /// - If the current instance matches the specified day of the week (`dow`), it will return the same date.
    /// - Otherwise, it will calculate the most recent date that falls on the specified day of the week.
    ///
    /// # Example
    /// ```
    /// // Assuming `self` is a date representing a Thursday
    /// let adjusted_date = self.previous_or_same(DayOfWeek::Monday);
    /// // `adjusted_date` will now represent the most recent Monday (or stay the same if already Monday)
    /// ```
    ///
    /// # Implementation Details
    /// - The function calculates the difference (`delta`) between the current day of the week
    ///   and the desired day of the week (`dow`), ensuring it wraps correctly in a 7-day range.
    /// - The calculated `delta` is subtracted from the current date using `minus_days`,
    ///   resulting in either the same or a previous date as required.
    ///
    pub fn previous_or_same(self, dow: DayOfWeek) -> Self {
        let cur = self.day_of_week();
        let delta = (cur.value() - dow.value() + 7) % 7;
        self.minus_days(delta as i64)
    }

    pub(crate) fn from(date: time::Date) -> Self {
        Self(date)
    }

    pub(crate) fn inner(self) -> time::Date {
        self.0
    }
}

impl core::ops::Sub for LocalDate {
    type Output = Duration;
    fn sub(self, rhs: Self) -> Self::Output {
        let duration: time::Duration = self.0 - rhs.0;
        Duration::from(duration)
    }
}

impl fmt::Display for LocalDate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
