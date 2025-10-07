use crate::{DayOfWeek, LocalDate, LocalDateTime, LocalTime, Month, Year};

pub trait Temporal: Ord + Copy {
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
    /// let datetime1 = OffsetDateTime::now();
    /// let datetime2 = datetime1.plus_hours(1);
    ///
    /// assert!(datetime1.is_before(datetime2));
    /// assert!(!datetime2.is_before(datetime1));
    /// ```
    fn is_before(self, other: Self) -> bool {
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
    /// let datetime1 = OffsetDateTime::now();
    /// let datetime2 = datetime1.plus_hours(1);
    ///
    /// assert!(!datetime1.is_after(datetime2));
    /// assert!(datetime2.is_after(datetime1));
    /// ```
    fn is_after(self, other: Self) -> bool {
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
    /// let datetime1 = OffsetDateTime::now();
    /// let datetime2 = datetime1.plus_hours(1);
    /// let datetime3 = datetime1.minus_hours(1);
    ///
    /// assert!(datetime1.is_on_or_before(datetime1));
    /// assert!(datetime1.is_on_or_before(datetime2));
    /// assert!(!datetime1.is_on_or_before(datetime3));
    /// ```
    fn is_on_or_before(self, other: Self) -> bool {
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
    /// let datetime1 = OffsetDateTime::now();
    /// let datetime2 = datetime1.minus_hours(1);
    /// let datetime3 = datetime1.plus_hours(1);
    ///
    /// assert!(datetime1.is_on_or_after(datetime1));
    /// assert!(datetime1.is_on_or_after(datetime2));
    /// assert!(!datetime1.is_on_or_after(datetime3));
    /// ```
    fn is_on_or_after(self, other: Self) -> bool {
        self >= other
    }
}

pub trait TemporalInstant {
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
    fn epoch_seconds(self) -> i64;

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
    fn epoch_milliseconds(self) -> i128;

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
    fn epoch_nanoseconds(self) -> i128;
}

pub trait TemporalDate: Temporal {
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
    /// let year = OffsetDateTime::now().year();
    /// println!("Year: {}", year);
    /// ```
    fn year(self) -> i32;

    /// Returns the `Month` corresponding to the month component of the date.
    ///
    /// ### Returns
    /// A `Month` corresponding to the month component of the date.
    ///
    /// ### Example
    /// ```rust
    /// let month = OffsetDateTime::now().month();
    /// println!("Month: {}", month);
    /// ```
    fn month(self) -> Month;

    /// Returns the numerical value associated with the month of the date.
    ///
    /// ### Returns
    /// An `i32` representing the numerical value of the month.
    ///
    /// ### Example
    /// ```rust
    /// let month_value = OffsetDateTime::now().month_value();
    /// println!("Month value: {}", month_value);
    /// ```
    fn month_value(self) -> i32 {
        self.month().value()
    }

    /// Returns the day of the year of the date.
    ///
    /// This method computes the ordinal day of the year (1-based) for the
    /// date encapsulated within the struct. For example, January 1st will
    /// return `1`, and December 31st will return `365` (or `366` in a leap year).
    ///
    /// ### Returns
    /// A `u16` representing the ordinal day of the year.
    ///
    /// ### Example
    /// ```rust
    /// let day_of_year = OffsetDateTime::now().day_of_year();
    /// println!("Day of year: {}", day_of_year);
    /// ```
    fn day_of_year(self) -> u16;

    /// Returns the day of the month of the date.
    ///
    /// ### Returns
    /// A `u8` representing the day of the month (1-31).
    ///
    /// ### Example
    /// ```rust
    /// let day_of_month = OffsetDateTime::now().day_of_month();
    /// println!("Day of month: {}", day_of_month);
    /// ```
    fn day_of_month(self) -> u8;

    /// Returns the `DayOfWeek` of the date.
    ///
    /// ### Returns
    /// A `DayOfWeek` representing the enumerated day of the week value corresponding to the date.
    ///
    /// ### Example
    /// ```rust
    /// let day_of_week = OffsetDateTime::now().day_of_week();
    /// println!("Day of week: {}", day_of_week);
    /// ```
    fn day_of_week(self) -> DayOfWeek;

    /// Returns the number of days in the month of the date.
    ///
    /// It evaluates whether the year is a leap year and
    /// determines the correct number of days in the month.
    ///
    /// ### Returns
    /// A `u8` representing the number of days in the month.
    ///
    /// ### Example
    /// ```rust
    /// let date = LocalDate::new(2023, 2);
    /// println!("Days in month: {}", date.length_of_month());
    /// ```
    fn length_of_month(self) -> i32 {
        let month = self.month();
        let year = Year::of(self.year());
        month.length(year.is_leap())
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
    fn is_leap_year(self) -> bool {
        Year::of(self.year()).is_leap()
    }

    /// Returns the number of days in the year of the date.
    ///
    /// ### Returns
    /// An `i32` representing the number of days in the year.
    /// For common years, this typically returns 365. For leap years, it returns 366.
    ///
    /// ### Examples
    /// ```rust
    /// let date = LocalDate::of(2024, 3, 26);
    /// let year = date.year();
    /// let days_in_year = date.length_of_year();
    /// println!("The year {} has {} days.", year, days_in_year);
    /// ```
    fn length_of_year(self) -> i32 {
        Year::of(self.year()).length()
    }

    /// Adds the specified number of years to the current instance, returning a new instance.
    ///
    /// ### Arguments
    /// * `years` - An `i64` value indicating the number of years to add. This value
    ///   can be positive or negative, allowing for both addition and subtraction.
    ///
    /// ### Returns
    /// * A new instance that represents the value resulting from
    ///   adding the specified years to the original instance.
    ///
    /// ### Panics
    /// This function will panic if the addition causes an overflow or underflow.
    ///
    /// ### Examples
    /// ```rust
    /// let datetime = OffsetDateTime::now_utc();
    /// let updated_datetime = datetime.plus_years(5);
    /// println!("Original: {}, Updated: {}", datetime, updated_datetime);
    /// ```
    fn plus_years(self, years: i64) -> Self;

    /// Subtracts the specified number of years from the current instance, returning a new instance.
    ///
    /// ### Arguments
    /// * `years` - An `i64` value indicating the number of years to subtract. This value
    ///   can be positive or negative, allowing for both addition and subtraction.
    ///
    /// ### Returns
    /// * A new instance that represents the value resulting from
    ///   subtracting the specified years from the original instance.
    ///
    /// ### Panics
    /// This function will panic if the subtraction causes an overflow or underflow.
    ///
    /// ### Examples
    /// ```rust
    /// let datetime = OffsetDateTime::now_utc();
    /// let updated_datetime = datetime.minus_years(5);
    /// println!("Original: {}, Updated: {}", datetime, updated_datetime);
    /// ```
    fn minus_years(self, years: i64) -> Self {
        self.plus_years(-years)
    }

    /// Adds the specified number of months to the current instance, returning a new instance.
    ///
    /// ### Arguments
    /// * `months` - An `i64` value indicating the number of months to add. This value
    ///   can be positive or negative, allowing for both addition and subtraction.
    ///
    /// ### Returns
    /// * A new instance that represents the value resulting from
    ///   adding the specified months to the original instance.
    ///
    /// ### Panics
    /// This function will panic if the addition causes an overflow or underflow.
    ///
    /// ### Examples
    /// ```rust
    /// let datetime = OffsetDateTime::now_utc();
    /// let updated_datetime = datetime.plus_months(5);
    /// println!("Original: {}, Updated: {}", datetime, updated_datetime);
    /// ```
    fn plus_months(self, months: i64) -> Self;

    /// Subtracts the specified number of months from the current instance, returning a new instance.
    ///
    /// ### Arguments
    /// * `months` - An `i64` value indicating the number of months to subtract. This value
    ///   can be positive or negative, allowing for both addition and subtraction.
    ///
    /// ### Returns
    /// * A new instance that represents the value resulting from
    ///   subtracting the specified months from the original instance.
    ///
    /// ### Panics
    /// This function will panic if the subtraction causes an overflow or underflow.
    ///
    /// ### Examples
    /// ```rust
    /// let datetime = OffsetDateTime::now_utc();
    /// let updated_datetime = datetime.minus_months(5);
    /// println!("Original: {}, Updated: {}", datetime, updated_datetime);
    /// ```
    fn minus_months(self, months: i64) -> Self  {
        self.plus_months(-months)
    }

    /// Adds the specified number of weeks to the current instance, returning a new instance.
    ///
    /// ### Arguments
    /// * `weeks` - An `i64` value indicating the number of weeks to add. This value
    ///   can be positive or negative, allowing for both addition and subtraction.
    ///
    /// ### Returns
    /// * A new instance that represents the value resulting from
    ///   adding the specified weeks to the original instance.
    ///
    /// ### Panics
    /// This function will panic if the addition causes an overflow or underflow.
    ///
    /// ### Examples
    /// ```rust
    /// let datetime = OffsetDateTime::now_utc();
    /// let updated_datetime = datetime.plus_weeks(5);
    /// println!("Original: {}, Updated: {}", datetime, updated_datetime);
    /// ```
    fn plus_weeks(self, weeks: i64) -> Self;

    /// Subtracts the specified number of weeks from the current instance, returning a new instance.
    ///
    /// ### Arguments
    /// * `weeks` - An `i64` value indicating the number of weeks to subtract. This value
    ///   can be positive or negative, allowing for both addition and subtraction.
    ///
    /// ### Returns
    /// * A new instance that represents the value resulting from
    ///   subtracting the specified weeks from the original instance.
    ///
    /// ### Panics
    /// This function will panic if the subtraction causes an overflow or underflow.
    ///
    /// ### Examples
    /// ```rust
    /// let datetime = OffsetDateTime::now_utc();
    /// let updated_datetime = datetime.minus_weeks(5);
    /// println!("Original: {}, Updated: {}", datetime, updated_datetime);
    /// ```
    fn minus_weeks(self, weeks: i64) -> Self;

    /// Adds the specified number of days to the current instance, returning a new instance.
    ///
    /// ### Arguments
    /// * `days` - An `i64` value indicating the number of days to add. This value
    ///   can be positive or negative, allowing for both addition and subtraction.
    ///
    /// ### Returns
    /// * A new instance that represents the value resulting from
    ///   adding the specified days to the original instance.
    ///
    /// ### Panics
    /// This function will panic if the addition causes an overflow or underflow.
    ///
    /// ### Examples
    /// ```rust
    /// let datetime = OffsetDateTime::now_utc();
    /// let updated_datetime = datetime.plus_days(5);
    /// println!("Original: {}, Updated: {}", datetime, updated_datetime);
    /// ```
    fn plus_days(self, days: i64) -> Self;

    /// Subtracts the specified number of days from the current instance, returning a new instance.
    ///
    /// ### Arguments
    /// * `days` - An `i64` value indicating the number of days to subtract. This value
    ///   can be positive or negative, allowing for both addition and subtraction.
    ///
    /// ### Returns
    /// * A new instance that represents the value resulting from
    ///   subtracting the specified days from the original instance.
    ///
    /// ### Panics
    /// This function will panic if the subtraction causes an overflow or underflow.
    ///
    /// ### Examples
    /// ```rust
    /// let datetime = OffsetDateTime::now_utc();
    /// let updated_datetime = datetime.minus_days(5);
    /// println!("Original: {}, Updated: {}", datetime, updated_datetime);
    /// ```
    fn minus_days(self, days: i64) -> Self;

    /// Returns a new instance with the year updated to the specified value.
    ///
    /// # Parameters
    /// - `year`: An `i32` representing the year to set in the date or time instance.
    ///
    /// # Returns
    /// A new instance of `Self` (the same type as the calling object) with the year updated to the provided value.
    ///
    /// # Example
    /// ```rust
    /// let date = LocalDate::new(2023, 7, 15);
    /// let updated_date = date.with_year(2025);
    /// assert_eq!(updated_date.year(), 2025);
    /// ```
    fn with_year(self, year: i32) -> Self;

    /// Returns a new instance with the month updated to the specified value.
    ///
    /// # Parameters
    /// - `month`: An `i32` representing the month to set, where valid values typically range
    ///   from 1 (January) to 12 (December).
    ///
    /// # Returns
    /// A new instance of `Self` (the same type as the calling object) with the month updated to the provided value.
    ///
    /// # Panics
    /// - The method may panic if the `month` value is out of the valid range (e.g., less than 1 or greater than 12),
    ///   depending on how the method is implemented.
    ///
    /// # Example
    /// ```rust
    /// let original_date = LocalDate::new(2023, 1, 15);
    /// let updated_date = original_date.with_month(7); // Changes the month to July
    /// assert_eq!(updated_date.month(), 7);
    /// ```
    fn with_month(self, month: i32) -> Self;

    /// Returns a new instance with the day of the year updated to the specified value.
    ///
    /// # Parameters
    /// - `day`: The day of the month to set (1-31).
    ///          The validity of the `day` value will depend on the month and year of the date.
    ///
    /// # Returns
    /// A new instance of `Self` (the same type as the calling object) with the day of month updated to the provided value.
    ///
    /// # Panics
    /// This method may panic if `day` is out of valid range (e.g., 0 or greater than 31),
    /// or if the day does not exist in the context of the date's month and year.
    ///
    /// # Examples
    /// ```rust
    /// let date = LocalDate::of(2023, 10, 15);
    /// let updated_date = date.with_day_of_month(25); // Updates date to 25th October 2023
    /// ```
    fn with_day_of_month(self, day: u8) -> Self;

    /// Returns a new instance with the day of the year updated to the specified value.
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
    fn with_day_of_year(self, day_of_year: u16) -> Self;

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
    fn first_day_of_month(self) -> Self {
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
    fn last_day_of_month(self) -> Self {
        let length = self.length_of_month();
        self.with_day_of_month(length as u8)
    }

    /// Calculates the first day of the next month based on the current date.
    ///
    /// This method shifts the date forward by one month and then sets the day to
    /// the first day of that month. It is useful for determining the starting date
    /// of the next calendar month.
    ///
    /// ### Returns
    /// A new instance of `Self` representing the first day of the next month.
    ///
    /// ### Example
    /// ```rust
    /// let date = LocalDate::of(2023, 10, 15);
    /// let first_day = date.first_day_of_next_month();
    /// assert_eq!(first_day, LocalDate::of(2023, 11, 1));
    /// ```
    fn first_day_of_next_month(self) -> Self {
        self.plus_months(1).first_day_of_month()
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
    fn first_day_of_year(self) -> Self {
        self.with_day_of_year(1)
    }

    /// Calculates the first day of the next year based on the current date.
    ///
    /// This method returns a new instance of the current date shifted to the same calendar day
    /// of the next year. It increments the year by one and adjusts the date to January 1st
    /// of the resulting year.
    ///
    /// # Returns
    /// * `Self` - A new instance representing the first day (January 1st) of the next calendar year.
    ///
    /// # Example
    /// ```
    /// let current_date = SomeDate::new(2023, 5, 15); // Example of a date instance.
    /// let next_year_first_day = current_date.first_day_of_next_year();
    /// assert_eq!(next_year_first_day, SomeDate::new(2024, 1, 1));
    /// ```
    fn first_day_of_next_year(self) -> Self {
        self.plus_years(1).with_day_of_year(1)
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
    fn last_day_of_year(self) -> Self {
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
    fn first_in_month(self, dow: DayOfWeek) -> Self {
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
    fn last_in_month(self, dow: DayOfWeek) -> Self {
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
    fn next(self, dow: DayOfWeek) -> Self {
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
    fn next_or_same(self, dow: DayOfWeek) -> Self {
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
    fn previous(self, dow: DayOfWeek) -> Self {
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
    fn previous_or_same(self, dow: DayOfWeek) -> Self {
        let cur = self.day_of_week();
        let delta = (cur.value() - dow.value() + 7) % 7;
        self.minus_days(delta as i64)
    }
}

pub trait TemporalTime: Temporal {
    /// Returns the hour component of the time.
    ///
    /// ### Returns
    /// A `u8` representing the hour of the time.
    ///
    /// ### Example
    /// ```rust
    /// let hour = OffsetDateTime::now().hour();
    /// println!("Hour value: {}", hour);
    /// ```
    fn hour(self) -> u8;

    /// Returns the second component of the time.
    ///
    /// ### Returns
    /// A `u8` representing the second of the time.
    ///
    /// ### Example
    /// ```rust
    /// let minute = OffsetDateTime::now().minute();
    /// println!("Minute value: {}", minute);
    /// ```
    fn minute(self) -> u8;

    /// Returns the second component of the time.
    ///
    /// ### Returns
    /// A `u8` representing the second of the time.
    ///
    /// ### Example
    /// ```rust
    /// let second = OffsetDateTime::now().second();
    /// println!("Second value: {}", second);
    /// ```
    fn second(self) -> u8;

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
    fn millisecond(self) -> u16;

    /// Returns the nanosecond component of the time.
    ///
    /// ### Returns
    /// An `i32` representing the nanosecond portion of the time.
    ///
    /// ### Example
    /// ```rust
    /// let nanoseconds = OffsetDateTime::now().nanosecond();
    /// println!("Nanoseconds value: {}", nanoseconds);
    /// ```
    fn nanosecond(self) -> i32;

    /// Calculates the total number of nanoseconds elapsed since the beginning of the day
    /// for the given time instance.
    ///
    /// # Returns
    ///
    /// A `u128` representing the total duration in nanoseconds from midnight to the current time.
    ///
    /// This is computed by converting the hours, minutes, and seconds to nanoseconds,
    /// and adding the existing nanoseconds.
    ///
    /// # Example
    ///
    /// ```rust
    /// let time = some_time_object; // Assume `time` is an object with `hour`, `minute`, `second`, `nanosecond` methods.
    /// let total_nanoseconds = time.total_nanoseconds_of_day();
    /// println!("Total nanoseconds since start of the day: {}", total_nanoseconds);
    /// ```
    fn total_nanoseconds_of_day(self) -> u128 {
        let hours = self.hour() as u128;
        let minutes = self.minute() as u128;
        let seconds = self.second() as u128;
        let nanoseconds = self.nanosecond() as u128;
        (((hours * 60 + minutes) * 60 + seconds) * 1_000_000_000) + nanoseconds
    }

    /// Adds the specified number of hours to the current `OffsetDateTime`, returning a new `OffsetDateTime` instance.
    ///
    /// ### Arguments
    ///
    /// * `hours` - An `i64` value indicating the number of hours to add. This value
    ///   can be positive or negative, allowing for both addition and subtraction.
    ///
    /// ### Returns
    ///
    /// * A new `OffsetDateTime` instance that represents the time resulting from
    ///   adding the specified hours to the original instance.
    ///
    /// ### Examples
    ///
    /// ```
    /// use joda_rs::OffsetDateTime;
    ///
    /// let datetime = OffsetDateTime::now_utc();
    /// let updated_datetime = datetime.plus_hours(5);
    /// println!("Original: {}, Updated: {}", datetime, updated_datetime);
    /// ```
    ///
    /// ### Panics
    /// This function will panic if the addition causes an overflow or underflow in the internal timestamp representation.
    fn plus_hours(self, hours: i64) -> Self;

    /// Subtracts the specified number of hours from the current `OffsetDateTime`, returning a new `OffsetDateTime` instance.
    ///
    /// ### Arguments
    ///
    /// * `hours` - An `i64` value indicating the number of hours to subtract. This value
    ///   can be positive or negative, allowing for both addition and subtraction.
    ///
    /// ### Returns
    ///
    /// * A new `OffsetDateTime` instance that represents the time resulting from
    ///   subtracting the specified hours from the original instance.
    ///
    /// ### Examples
    ///
    /// ```
    /// use joda_rs::OffsetDateTime;
    ///
    /// let datetime = OffsetDateTime::now_utc();
    /// let updated_datetime = datetime.minus_hours(5);
    /// println!("Original: {}, Updated: {}", datetime, updated_datetime);
    /// ```
    ///
    /// ### Panics
    /// This function will panic if the subtraction causes an overflow or underflow in the internal timestamp representation.
    fn minus_hours(self, hours: i64) -> Self;

    /// Adds the specified number of minutes to the current `OffsetDateTime`, returning a new `OffsetDateTime` instance.
    ///
    /// ### Arguments
    ///
    /// * `minutes` - An `i64` value indicating the number of minutes to add. This value
    ///   can be positive or negative, allowing for both addition and subtraction.
    ///
    /// ### Returns
    ///
    /// * A new `OffsetDateTime` instance that represents the time resulting from
    ///   adding the specified minutes to the original instance.
    ///
    /// ### Examples
    ///
    /// ```
    /// use joda_rs::OffsetDateTime;
    ///
    /// let datetime = OffsetDateTime::now_utc();
    /// let updated_datetime = datetime.plus_minutes(5);
    /// println!("Original: {}, Updated: {}", datetime, updated_datetime);
    /// ```
    ///
    /// ### Panics
    /// This function will panic if the addition causes an overflow or underflow in the internal timestamp representation.
    fn plus_minutes(self, minutes: i64) -> Self;

    /// Subtracts the specified number of minutes from the current `OffsetDateTime`, returning a new `OffsetDateTime` instance.
    ///
    /// ### Arguments
    ///
    /// * `minutes` - An `i64` value indicating the number of minutes to subtract. This value
    ///   can be positive or negative, allowing for both addition and subtraction.
    ///
    /// ### Returns
    ///
    /// * A new `OffsetDateTime` instance that represents the time resulting from
    ///   subtracting the specified minutes from the original instance.
    ///
    /// ### Examples
    ///
    /// ```
    /// use joda_rs::OffsetDateTime;
    ///
    /// let datetime = OffsetDateTime::now_utc();
    /// let updated_datetime = datetime.minus_minutes(5);
    /// println!("Original: {}, Updated: {}", datetime, updated_datetime);
    /// ```
    ///
    /// ### Panics
    /// This function will panic if the subtraction causes an overflow or underflow in the internal timestamp representation.
    fn minus_minutes(self, minutes: i64) -> Self;

    /// Adds the specified number of seconds to the current `OffsetDateTime`, returning a new `OffsetDateTime` instance.
    ///
    /// ### Arguments
    ///
    /// * `seconds` - An `i64` value indicating the number of seconds to add. This value
    ///   can be positive or negative, allowing for both addition and subtraction.
    ///
    /// ### Returns
    ///
    /// * A new `OffsetDateTime` instance that represents the time resulting from
    ///   adding the specified seconds to the original instance.
    ///
    /// ### Examples
    ///
    /// ```
    /// use joda_rs::OffsetDateTime;
    ///
    /// let datetime = OffsetDateTime::now_utc();
    /// let updated_datetime = datetime.plus_seconds(5);
    /// println!("Original: {}, Updated: {}", datetime, updated_datetime);
    /// ```
    ///
    /// ### Panics
    /// This function will panic if the addition causes an overflow or underflow in the internal timestamp representation.
    fn plus_seconds(self, seconds: i64) -> Self;

    /// Subtracts the specified number of seconds from the current `OffsetDateTime`, returning a new `OffsetDateTime` instance.
    ///
    /// ### Arguments
    ///
    /// * `seconds` - An `i64` value indicating the number of seconds to subtract. This value
    ///   can be positive or negative, allowing for both addition and subtraction.
    ///
    /// ### Returns
    ///
    /// * A new `OffsetDateTime` instance that represents the time resulting from
    ///   subtracting the specified seconds from the original instance.
    ///
    /// ### Examples
    ///
    /// ```
    /// use joda_rs::OffsetDateTime;
    ///
    /// let datetime = OffsetDateTime::now_utc();
    /// let updated_datetime = datetime.minus_seconds(5);
    /// println!("Original: {}, Updated: {}", datetime, updated_datetime);
    /// ```
    ///
    /// ### Panics
    /// This function will panic if the subtraction causes an overflow or underflow in the internal timestamp representation.
    fn minus_seconds(self, seconds: i64) -> Self;

    /// Adds the specified number of nanoseconds to the current `OffsetDateTime`, returning a new `OffsetDateTime` instance.
    ///
    /// ### Arguments
    ///
    /// * `nanos` - An `i64` value indicating the number of nanoseconds to add. This value
    ///   can be positive or negative, allowing for both addition and subtraction.
    ///
    /// ### Returns
    ///
    /// * A new `OffsetDateTime` instance that represents the time resulting from
    ///   adding the specified nanoseconds to the original instance.
    ///
    /// ### Examples
    ///
    /// ```
    /// use joda_rs::OffsetDateTime;
    ///
    /// let datetime = OffsetDateTime::now_utc();
    /// let updated_datetime = datetime.plus_nanos(5);
    /// println!("Original: {}, Updated: {}", datetime, updated_datetime);
    /// ```
    ///
    /// ### Panics
    /// This function will panic if the addition causes an overflow or underflow in the internal timestamp representation.
    fn plus_millis(self, milliseconds: i64) -> Self;

    /// Subtracts the specified number of nanoseconds from the current `OffsetDateTime`, returning a new `OffsetDateTime` instance.
    ///
    /// ### Arguments
    ///
    /// * `nanos` - An `i64` value indicating the number of nanoseconds to subtract. This value
    ///   can be positive or negative, allowing for both addition and subtraction.
    ///
    /// ### Returns
    ///
    /// * A new `OffsetDateTime` instance that represents the time resulting from
    ///   subtracting the specified nanoseconds from the original instance.
    ///
    /// ### Examples
    ///
    /// ```
    /// use joda_rs::OffsetDateTime;
    ///
    /// let datetime = OffsetDateTime::now_utc();
    /// let updated_datetime = datetime.minus_millis(5);
    /// println!("Original: {}, Updated: {}", datetime, updated_datetime);
    /// ```
    ///
    /// ### Panics
    /// This function will panic if the subtraction causes an overflow or underflow in the internal timestamp representation.
    fn minus_millis(self, milliseconds: i64) -> Self;

    /// Adds the specified number of nanoseconds to the current `OffsetDateTime`, returning a new `OffsetDateTime` instance.
    ///
    /// ### Arguments
    ///
    /// * `nanos` - An `i64` value indicating the number of nanoseconds to add. This value
    ///   can be positive or negative, allowing for both addition and subtraction.
    ///
    /// ### Returns
    ///
    /// * A new `OffsetDateTime` instance that represents the time resulting from
    ///   adding the specified nanoseconds to the original instance.
    ///
    /// ### Examples
    ///
    /// ```
    /// use joda_rs::OffsetDateTime;
    ///
    /// let datetime = OffsetDateTime::now_utc();
    /// let updated_datetime = datetime.plus_nanos(5);
    /// println!("Original: {}, Updated: {}", datetime, updated_datetime);
    /// ```
    ///
    /// ### Panics
    /// This function will panic if the addition causes an overflow or underflow in the internal timestamp representation.
    fn plus_nanos(self, nanoseconds: i64) -> Self;

    /// Subtracts the specified number of nanoseconds from the current `OffsetDateTime`, returning a new `OffsetDateTime` instance.
    ///
    /// ### Arguments
    ///
    /// * `nanos` - An `i64` value indicating the number of nanoseconds to subtract. This value
    ///   can be positive or negative, allowing for both addition and subtraction.
    ///
    /// ### Returns
    ///
    /// * A new `OffsetDateTime` instance that represents the time resulting from
    ///   subtracting the specified nanoseconds from the original instance.
    ///
    /// ### Examples
    ///
    /// ```
    /// use joda_rs::OffsetDateTime;
    ///
    /// let datetime = OffsetDateTime::now_utc();
    /// let updated_datetime = datetime.minus_nanos(5);
    /// println!("Original: {}, Updated: {}", datetime, updated_datetime);
    /// ```
    ///
    /// ### Panics
    /// This function will panic if the subtraction causes an overflow or underflow in the internal timestamp representation.
    fn minus_nanos(self, nanoseconds: i64) -> Self;

    /// Returns a new instance of the object with the hour set to the specified value.
    ///
    /// # Parameters
    /// - `hour` (u8): The hour to set, typically in a 24-hour format (0-23). The value
    ///   should be valid within this range; otherwise, behavior may be undefined or result
    ///   in a runtime error depending on the implementation.
    ///
    /// # Returns
    /// A new instance of `Self` with the hour component updated. Other components
    /// (e.g., minutes, seconds) remain unchanged.
    ///
    /// # Examples
    /// ```rust
    /// let time = SomeTimeStruct::new(10, 30, 45); // Example initialization
    /// let updated_time = time.with_hour(15);
    /// assert_eq!(updated_time.hour(), 15);
    /// ```
    ///
    /// Note that this function does not mutate the original instance but rather returns
    /// a new modified instance.
    fn with_hour(self, hour: u8) -> Self;

    /// Modifies the time instance by setting the minute component to the specified value.
    ///
    /// # Parameters
    /// - `minute`: An unsigned 8-bit integer (`u8`) representing the minute value
    ///   to set. Valid values range from 0 to 59.
    ///
    /// # Returns
    /// Returns a new instance of the type implementing this method with the updated
    /// minute value. The rest of the time components (e.g., hour, second) remain unchanged.
    ///
    /// # Panics
    /// This function may panic if the provided minute value is outside the valid range (0–59),
    /// depending on the implementation.
    ///
    /// # Examples
    /// ```rust
    /// let time = some_time.with_minute(45);
    /// assert_eq!(time.minute(), 45);
    /// ```
    fn with_minute(self, minute: u8) -> Self;

    /// Sets the `second` field of the implementing type to the provided value.
    ///
    /// This method takes a `u8` value representing the second (0-59) and returns
    /// an updated instance of the implementing type with the `second` set to the specified value.
    ///
    /// # Parameters
    ///
    /// * `second`: A `u8` value representing the seconds (must be in the range 0-59).
    ///
    /// # Returns
    ///
    /// An updated instance of the implementing type with the specified `second` value.
    ///
    /// # Panics
    ///
    /// This method may panic if the provided `second` value is out of the valid range
    /// (e.g., if a value greater than 59 is provided). Ensure the input is within the valid range.
    ///
    /// # Examples
    ///
    /// ```
    /// let time = Time::new(10, 20, 30); // Assuming a Time struct with hours, minutes, seconds
    /// let updated_time = time.with_second(45);
    /// assert_eq!(updated_time.second(), 45);
    /// ```
    fn with_second(self, second: u8) -> Self;

    /// Sets the millisecond component of the current time or date.
    ///
    /// This method takes a `u16` value representing the desired millisecond value
    /// (0-999) and updates the millisecond component of the instance. It returns
    /// a new instance with the updated millisecond, leaving the original instance
    /// unmodified.
    ///
    /// # Parameters
    /// - `millisecond`: A `u16` value representing the millisecond to set.
    ///   The value must be in the range of `0` to `999`, where `0` represents
    ///   the start of the second and `999` is the last millisecond of the second.
    ///
    /// # Returns
    /// - `Self`: A new instance of the type with the updated millisecond value.
    ///
    /// # Panics
    /// This method may panic if the provided millisecond value is out of the valid range (0-999).
    /// Ensure the input is properly validated before calling this method.
    ///
    /// # Examples
    /// ```rust
    /// let original_time = MyTime::new(12, 30, 45, 500); // Assuming `MyTime` has hours, minutes, seconds, ms.
    /// let updated_time = original_time.with_millisecond(250);
    ///
    /// assert_eq!(updated_time.millisecond(), 250);
    /// assert_eq!(original_time.millisecond(), 500); // The original instance remains unchanged.
    /// ```
    fn with_millisecond(self, millisecond: u16) -> Self;

    /// Returns a new instance of the type with the nanosecond field updated to the given value.
    ///
    /// # Parameters
    /// - `nanosecond`: A `u32` value representing the nanoseconds to set. Must be between `0` and `999,999,999` inclusive.
    ///
    /// # Returns
    /// A new instance of the type with the updated nanosecond value. If the provided `nanosecond`
    /// value is out of range, the method may panic or return an invalid/unexpected result based on
    /// the implementation.
    ///
    /// # Examples
    /// ```
    /// let time = some_time.with_nanosecond(123_456_789);
    /// assert_eq!(time.nanosecond(), 123_456_789);
    /// ```
    fn with_nanosecond(self, nanosecond: u32) -> Self;
}

pub trait TemporalDateTime: TemporalDate + TemporalTime {
    /// Converts the current instance into a `LocalDate`.
    ///
    /// ### Returns
    /// - A `LocalDate` that represents the local date portion of the current instance.
    ///
    /// ### Example
    /// ```rust
    /// let date_time = OffsetDateTime::now_utc();
    /// let local_date = date_time.to_local_date();
    /// println!("{}", local_date);
    /// ```
    fn to_local_date(self) -> LocalDate;

    /// Converts the current instance of a time object to its equivalent local time representation.
    ///
    /// ### Returns
    /// * `LocalTime` - A representation of the time adjusted to the local timezone.
    ///
    /// ### Example
    /// ```rust
    /// let utc_time = OffsetDateTime::now_utc();
    /// let local_time = utc_time.to_local_time();
    /// println!("Local time: {:?}", local_time);
    /// ```
    fn to_local_time(self) -> LocalTime;

    /// Converts the given instance to a `LocalDateTime`.
    ///
    /// ### Returns
    /// A `LocalDateTime` object representing the converted date and time in the
    /// system's local timezone.
    ///
    /// ### Examples
    /// ```rust
    /// let utc_date_time = OffsetDateTime::now_utc();
    /// let local_date_time = utc_date_time.to_local_date_time();
    /// println!("{}", local_date_time);
    /// ```
    fn to_local_date_time(self) -> LocalDateTime;
}
