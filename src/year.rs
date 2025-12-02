use crate::{Clock, LocalDate, Month, YearMonth, ZoneId};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// A wrapper struct representing a year as a 32-bit signed integer.
///
/// This struct provides a lightweight, type-safe abstraction for representing
/// years, ensuring that the value is always treated explicitly as a `Year`.
///
/// # Derivable Traits
/// - `Debug`: Enables formatting the `Year` using the `{:?}` formatter.
/// - `Clone`: Allows creating a copy of the `Year` struct.
/// - `Copy`: Allows bitwise copying of the `Year` since it is a simple value type.
/// - `PartialEq`: Enables equality comparisons between `Year` instances.
/// - `Eq`: Ensures that `Year` supports full equality comparison.
/// - `Hash`: Allows instances of `Year` to be hashed, enabling their use
///   in hashed collections like `HashMap` or `HashSet`.
/// - `PartialOrd`: Allows lexicographical ordering of `Year` values, supporting
///   partial ordering. For `Year`, this is effectively total ordering due to `i32`.
/// - `Ord`: Enables total ordering of `Year` values.
///
/// # Fields
/// - `0: i32` - The wrapped `i32` value representing the year.
///
/// # Examples
///
/// ```rust
/// use joda_rs::Year;
///
/// let year_2023 = Year::of(2023);
/// let year_1990 = Year::of(1990);
///
/// assert!(year_2023 > year_1990);
/// assert_eq!(year_2023.value(), 2023);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Year(pub i32);

impl Year {
    /// Creates a new instance of `Year` with the given integer value.
    ///
    /// # Arguments
    ///
    /// * `value` - An i32 that represents the numerical value of the year.
    ///
    /// # Returns
    ///
    /// Returns a new `Year` instance initialized with the provided `value`.
    ///
    /// # Example
    ///
    /// ```
    /// use joda_rs::Year;
    ///
    /// let year = Year::of(2023);
    /// println!("{:?}", year); // Output: Year(2023)
    /// ```
    pub fn of(value: i32) -> Self { Year(value) }

    /// Returns the year `i32` value.
    ///
    /// # Example
    ///
    /// ```
    /// use joda_rs::Year;
    ///
    /// let year = Year::of(2023);
    /// assert_eq!(year.value(), 2023);
    /// ```
    pub fn value(self) -> i32 { self.0 }

    /// Returns the current year as an instance of `Self` using the system's UTC clock.
    ///
    /// # Returns
    /// An instance of `Self` representing the current year.
    ///
    /// # Examples
    /// ```
    /// use joda_rs::Year;
    ///
    /// println!("Current year: {:?}", Year::now().value());
    /// ```
    pub fn now() -> Self {
        Self::now_with_clock(Clock::system_utc())
    }

    /// Retrieves the current datetime in the specified time zone.
    ///
    /// This function uses the system clock to obtain the current local time
    /// adjusted to the provided time zone identifier (`zone_id`). It creates an
    /// instance of the `Clock` struct configured with the given time zone and
    /// retrieves the current datetime based on that clock.
    ///
    /// # Parameters
    /// - `zone_id`: A `ZoneId` representing the desired time zone for the year.
    ///
    /// # Returns
    /// A new instance of `Self` representing the current year in the specified time zone.
    ///
    /// # Example
    /// ```rust
    /// use joda_rs::ZoneId;
    /// use joda_rs::Year;
    ///
    /// let zone_id = ZoneId::new("America/New_York");
    /// let current_year = Year::now_with_zone(zone_id);
    /// println!("Current year in New York: {:?}", current_year);
    /// ```
    ///
    /// # Panics
    /// This function may panic if the system clock is unavailable or if the
    /// specified `zone_id` is invalid.
    ///
    /// # See Also
    /// - [`Self::now_with_clock`](#method.now_with_clock): A more general method
    /// that accepts a `Clock` instance directly for greater control over time
    /// computation.
    pub fn now_with_zone(zone_id: ZoneId) -> Self {
        Self::now_with_clock(Clock::system(zone_id))
    }

    /// Creates a `Year` instance based on the current time provided by a given `Clock`.
    ///
    /// # Parameters
    /// - `clock`: A `Clock` instance that provides the current time in a specific time zone.
    ///
    /// # Returns
    /// A new `Year` instance containing the year extracted from the current local date
    /// in the provided clock's time zone.
    ///
    /// # Example
    /// ```
    /// use jod_rs::{Year, Clock};
    ///
    /// let clock = Clock::system();
    /// let current_year = Year::now_with_clock(clock);
    /// println!("Current year: {:?}", current_year);
    /// ```
    ///
    /// # Notes
    /// This function relies on the `Clock` to determine the current time and time zone.
    /// Ensure the `Clock` instance is correctly configured for your use case.
    pub fn now_with_clock(clock: Clock) -> Self {
        Year(clock.instant().at_zone(clock.zone()).to_local_date().year())
    }

    /// Determines whether the given year is a leap year.
    ///
    /// A leap year is defined as follows:
    /// - It is divisible by 4.
    /// - However, if it is divisible by 100, it is not a leap year unless it is also divisible by 400.
    ///
    /// # Returns
    ///
    /// `true` if the year is a leap year, `false` otherwise.
    ///
    /// # Examples
    ///
    /// ```
    /// assert_eq!(Year(2000).is_leap(), true); // 2000 is a leap year
    /// assert_eq!(Year(1900).is_leap(), false); // 1900 is not a leap year
    /// assert_eq!(Year(2024).is_leap(), true); // 2024 is a leap year
    /// assert_eq!(Year(2023).is_leap(), false); // 2023 is not a leap year
    /// ```
    pub fn is_leap(self) -> bool {
        let y = self.0;
        (y % 4 == 0) && ((y % 100 != 0) || (y % 400 == 0))
    }

    /// Returns the number of days in the current year.
    ///
    /// This method calculates the number of days based on whether the year is a leap year or not.
    ///
    /// # Returns
    /// - `366` if the year is a leap year.
    /// - `365` if the year is not a leap year.
    ///
    /// # Note
    /// This method internally calls the `is_leap` function to determine if the year is a leap year.
    ///
    /// # Example
    /// ```
    /// let year = Year::of(2020);
    /// let days = year.length();
    /// assert_eq!(days, 366); // For a leap year
    /// ```
    pub fn length(self) -> i32 { if self.is_leap() { 366 } else { 365 } }

    /// Adds a specified number of years to the current `Year` instance.
    ///
    /// # Parameters
    /// - `years`: An `i64` value representing the number of years to add to the current year.
    ///
    /// # Returns
    /// A new `Year` instance with the updated year value which is the result of adding `years` to the current year.
    ///
    /// # Panics
    /// The function will panic if the resulting year value exceeds the bounds of an `i32`
    /// (i.e., `i32::MAX` or `i32::MIN`), ensuring that the year value remains within the valid range.
    ///
    /// # Examples
    /// ```
    /// let year = Year::of(2023);
    /// let updated_year = year.plus(5); // Adds 5 years to 2023
    /// assert_eq!(updated_year, Year::of(2028));
    ///
    /// // Panics if the resulting year exceeds `i32` limits
    /// let year = Year::of(2_000_000_000);
    /// // year.plus(1); // This will panic
    /// ```
    pub fn plus(self, years: i32) -> Self {
        let v = self.0 as i64 + years as i64;
        if v > i32::MAX as i64 || v < i32::MIN as i64 { panic!("year overflow") }
        Year(v as i32)
    }

    /// Subtracts the specified number of years from the current instance.
    ///
    /// # Arguments
    ///
    /// * `years` - A signed 32-bit integer representing the number of years to subtract.
    ///   Providing a negative value is equivalent to adding years.
    ///
    /// # Returns
    ///
    /// Returns a modified instance of the same type, offset by the specified number of years.
    ///
    /// # Behavior
    ///
    /// This function internally calls the `plus` method with a negative number of years
    /// to perform subtraction.
    ///
    /// # Examples
    ///
    /// ```
    /// let year1 = Year::of(2028).minus(5); // Subtracts 5 years
    /// let year2 = Year::of(2025).minus(-3); // Adds 3 years (negative subtraction)
    /// ```
    pub fn minus(self, years: i32) -> Self { self.plus(-years) }

    /// Determines if the current `Year` instance is before another given `Year`.
    ///
    /// # Parameters
    /// - `other`: The `Year` instance to compare against.
    ///
    /// # Returns
    /// - `true` if the current year is earlier than the `other` year.
    /// - `false` otherwise.
    ///
    /// # Example
    /// ```
    /// let year1 = Year::of(2020);
    /// let year2 = Year::of(2025);
    ///
    /// assert!(year1.is_before(year2));
    /// assert!(!year2.is_before(year1));
    /// ```
    pub fn is_before(self, other: Year) -> bool { self.0 < other.0 }

    ///
    /// Determines if the current `Year` is after the specified `other` `Year`.
    ///
    /// # Arguments
    ///
    /// * `&self` - A reference to the current `Year`.
    /// * `other` - A `Year` to compare against.
    ///
    /// # Returns
    ///
    /// * `true` if the current `Year` is after `other`.
    /// * `false` otherwise.
    ///
    /// # Examples
    ///
    /// ```
    /// let year1 = Year::of(2023);
    /// let year2 = Year::of(2022);
    ///
    /// assert!(year1.is_after(year2));
    /// assert!(!year2.is_after(year1));
    /// ```
    pub fn is_after(self, other: Year) -> bool { self.0 > other.0 }

    /// Determines if the current `Year` instance is on or before the given `Year`.
    ///
    /// # Parameters
    /// - `other`: A `Year` instance to compare against.
    ///
    /// # Returns
    /// - `true` if the `Year` represented by `self` is less than or equal to the `Year` represented by `other`.
    /// - `false` otherwise.
    ///
    /// # Example
    /// ```
    /// let year1 = Year::of(2020);
    /// let year2 = Year::of(2021);
    ///
    /// assert!(year1.is_on_or_before(year2)); // 2020 is on or before 2021
    /// assert!(!year2.is_on_or_before(year1)); // 2021 is not before 2020
    /// ```
    pub fn is_on_or_before(self, other: Year) -> bool { self.0 <= other.0 }

    /// Checks if the current `Year` is on or after the specified `other` `Year`.
    ///
    /// # Arguments
    ///
    /// * `other` - A `Year` to compare against the current `Year`.
    ///
    /// # Returns
    ///
    /// * `true` if the current `Year` is greater than or equal to the `other` `Year`.
    /// * `false` otherwise.
    ///
    /// # Example
    ///
    /// ```
    /// let year1 = Year::of(2023);
    /// let year2 = Year::of(2022);
    ///
    /// assert!(year1.is_on_or_after(year2)); // 2023 is on or after 2022
    /// assert!(!year2.is_on_or_after(year1)); // 2022 is not on or after 2023
    /// ```
    pub fn is_on_or_after(self, other: Year) -> bool { self.0 >= other.0 }

    /// Constructs a `YearMonth` instance by combining the current year (encapsulated in the `self` object)
    /// with the specified month.
    ///
    /// # Arguments
    ///
    /// * `month` - A value of type `Month` representing the desired month to associate with the year.
    ///
    /// # Returns
    ///
    /// * A `YearMonth` instance representing the combination of the current year and the specified month.
    ///
    /// # Example
    ///
    /// ```rust
    /// use joda_rs::{Year, Month, YearMonth};
    ///
    /// let year = Year::new(2023);
    /// let month = Month::January;
    /// let year_month = year.at_month(month);
    ///
    /// assert_eq!(year_month, YearMonth::of_year_month(2023, Month::January));
    /// ```
    pub fn at_month(self, month: Month) -> YearMonth {
        YearMonth::of_year_month(self.0, month)
    }

    /// Creates a `LocalDate` instance corresponding to the provided `month` and `day`
    /// in the year encapsulated by the current `self` instance.
    ///
    /// # Arguments
    ///
    /// * `month` - The month of the year represented as a `Month` enum value.
    /// * `day` - The day of the month as an unsigned 8-bit integer (`u8`).
    ///
    /// # Returns
    ///
    /// * A `LocalDate` instance representing the specified date.
    ///
    /// # Panics
    ///
    /// This function will panic if the provided combination of `month` and `day`
    /// does not form a valid date in the year encapsulated by `self`. For example,
    /// it will panic if `day` is greater than the number of days in the specified
    /// `month`, or if `month` falls outside the valid range.
    ///
    /// # Examples
    ///
    /// ```
    /// use joda_rs::{Year, Month, LocalDate};
    ///
    /// let year = Year::new(2023);
    /// let month = Month::March;
    /// let day = 15;
    ///
    /// let date = year.at_month_day(month, day);
    /// assert_eq!(date.to_string(), "2023-03-15");
    /// ```
    pub fn at_month_day(self, month: Month, day: u8) -> LocalDate {
        let m: time::Month = month.into();
        let d = time::Date::from_calendar_date(self.0, m, day).expect("invalid date");
        LocalDate::from(d)
    }

    /// Returns a `LocalDate` corresponding to the given day of the year (`day_of_year`) within the current year.
    ///
    /// # Parameters
    /// - `day_of_year` (u16): The day of the year, starting from 1. Must be in the range `1..=365`
    ///   for common years or `1..=366` for leap years. Passing `day_of_year` as `0` will
    ///   trigger a panic, as it is considered an invalid input.
    ///
    /// # Returns
    /// - A `LocalDate` representing the date derived from the given `day_of_year` within the
    ///   year encapsulated by the instance. The calculation is based on January 1st as the
    ///   starting point.
    ///
    /// # Panics
    /// - The function panics if `day_of_year` is `0`, as there is no valid day for such input.
    ///
    /// # Example
    /// ```rust
    /// let year = Year::of(2023);
    /// let date = year.at_day(32);    // Corresponds to February 1st, 2023.
    /// assert_eq!(date, LocalDate::of(2023, 2, 1));
    /// ```
    ///
    /// # Implementation Notes
    /// - Internally, the function calculates the date by creating the January 1st date of the
    ///   year and then adding the appropriate number of days to derive the final result.
    /// - `time::Date::from_calendar_date` is employed to initialize January 1st of the year.
    /// - The addition of days is performed safely using `saturating_add` to prevent overflow/range issues.
    ///
    /// # Remarks
    /// - Ensure the `day_of_year` corresponds to the correct number of days for the respective year type
    ///   (common or leap year) to avoid unexpected results.
    pub fn at_day(self, day_of_year: u16) -> LocalDate {
        if day_of_year == 0 { panic!("day-of-year must be 1..=365/366") }
        let jan1 = time::Date::from_calendar_date(self.0, time::Month::January, 1).unwrap();
        let doy = day_of_year as i64 - 1;
        LocalDate::from(jan1.saturating_add(time::Duration::days(doy)))
    }
}

impl core::fmt::Display for Year {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result { write!(f, "{}", self.0) }
}
