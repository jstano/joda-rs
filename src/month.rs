#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Represents the months of the year as an enum with each variant corresponding to a specific month.
///
/// # Variants
/// - `January`: Represents the month of January, assigned the value `1`.
/// - `February`: Represents the month of February, assigned the value `2`.
/// - `March`: Represents the month of March, assigned the value `3`.
/// - `April`: Represents the month of April, assigned the value `4`.
/// - `May`: Represents the month of May, assigned the value `5`.
/// - `June`: Represents the month of June, assigned the value `6`.
/// - `July`: Represents the month of July, assigned the value `7`.
/// - `August`: Represents the month of August, assigned the value `8`.
/// - `September`: Represents the month of September, assigned the value `9`.
/// - `October`: Represents the month of October, assigned the value `10`.
/// - `November`: Represents the month of November, assigned the value `11`.
/// - `December`: Represents the month of December, assigned the value `12`.
///
/// # Traits
/// This enum derives the following traits:
/// - `Debug`: Allows the enumeration to be formatted using the `Debug` formatter.
/// - `Clone`: Allows the `Month` enum to be cloned.
/// - `Copy`: Enables the `Month` enum to implement copy semantics so that instances are duplicated implicitly
///   rather than moved.
/// - `PartialEq` and `Eq`: Enable comparison of `Month` instances for equality and inequality.
/// - `Hash`: Allows `Month` to be hashed, making it usable as keys in hash maps.
/// - `PartialOrd` and `Ord`: Enables comparison of `Month` instances for ordering purposes.
///
/// # Usage
/// Commonly used in scenarios where representing a specific month of the year is required:
///
/// ```
/// use joda_rs::Month;
///
/// let january = Month::January;
/// let december = Month::December;
///
/// assert_eq!(january.value(), 1);
/// assert!(january < december);
/// ```
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum Month {
    January = 1,
    February = 2,
    March = 3,
    April = 4,
    May = 5,
    June = 6,
    July = 7,
    August = 8,
    September = 9,
    October = 10,
    November = 11,
    December = 12,
}

impl Month {
    /// Creates an instance of `Month` from a given integer value.
    ///
    /// # Arguments
    ///
    /// * `value` - An integer representing the month. Valid values are:
    ///   * `1` for January
    ///   * `2` for February
    ///   * `3` for March
    ///   * `4` for April
    ///   * `5` for May
    ///   * `6` for June
    ///   * `7` for July
    ///   * `8` for August
    ///   * `9` for September
    ///   * `10` for October
    ///   * `11` for November
    ///   * `12` for December
    ///
    /// # Returns
    ///
    /// A corresponding `Month` variant based on the input integer.
    ///
    /// # Panics
    ///
    /// This function will panic if the input value is not in the range `1..=12`.
    ///
    /// # Examples
    ///
    /// ```
    /// use joda_rs::Month;
    ///
    /// let month = Month::of(1);
    /// assert_eq!(month, Month::January);
    ///
    /// let month = Month::of(12);
    /// assert_eq!(month, Month::December);
    /// ```
    ///
    /// ```should_panic
    /// // This will panic because 13 is not a valid month value.
    /// let invalid_month = Month::of(13);
    /// ```
    pub fn of(value: i32) -> Self {
        match value {
            1 => Month::January,
            2 => Month::February,
            3 => Month::March,
            4 => Month::April,
            5 => Month::May,
            6 => Month::June,
            7 => Month::July,
            8 => Month::August,
            9 => Month::September,
            10 => Month::October,
            11 => Month::November,
            12 => Month::December,
            _ => panic!("invalid month value (1-12)"),
        }
    }

    /// Converts the instance of the implementing enum or type into its corresponding `i32` value.
    ///
    /// # Returns
    ///
    /// * `i32` - The integer representation of the Month.
    ///
    /// # Example
    ///
    /// ```rust
    /// use joda_rs::Month;
    ///
    /// let month = Month::January;
    /// assert_eq!(month.value(), 1);
    ///
    /// let month = Month::December;
    /// assert_eq!(month.value(), 12);
    /// ```
    pub fn value(self) -> i32 { self as i32 }

    /// Returns the number of days in the month.
    ///
    /// This function calculates the number of days in the month represented by `self`.
    /// It takes into account whether the year is a leap year when determining the length
    /// of February.
    ///
    /// # Parameters
    ///
    /// - `leap_year`: A boolean indicating whether the year is a leap year.
    ///   If `true`, February will have 29 days; otherwise, it will have 28 days.
    ///
    /// # Returns
    ///
    /// - A `usize` value representing the number of days in the month.
    ///
    /// # Example
    ///
    /// ```
    /// use joda_rs::Month;
    ///
    /// let january = Month::January;
    /// assert_eq!(january.length(false), 31);
    ///
    /// let february_normal = Month::February;
    /// assert_eq!(february_normal.length(false), 28);
    ///
    /// let february_leap = Month::February;
    /// assert_eq!(february_leap.length(true), 29);
    /// ```
    ///
    /// # Note
    ///
    /// This function assumes `self` represents a valid month from the `Month` enum.
    pub fn length(self, leap_year: bool) -> i32 {
        match self {
            Month::January => 31,
            Month::February => if leap_year { 29 } else { 28 },
            Month::March => 31,
            Month::April => 30,
            Month::May => 31,
            Month::June => 30,
            Month::July => 31,
            Month::August => 31,
            Month::September => 30,
            Month::October => 31,
            Month::November => 30,
            Month::December => 31,
        }
    }

    /// Returns the minimum length of the Month.
    ///
    /// # Returns
    ///
    /// * `usize` - The minimum length of the Month.
    ///
    /// # Example
    ///
    /// ```
    /// use joda_rs::Month;
    ///
    /// let january = Month::January;
    /// assert_eq!(january.min_length(false), 31);
    ///
    /// let february = Month::February;
    /// assert_eq!(february.min_length(false), 28);
    /// ```
    pub fn min_length(self) -> i32 { self.length(false) }

    /// Returns the maximum length of the Month.
    ///
    /// # Returns
    ///
    /// * `usize` - The maximum length of the Month.
    ///
    /// # Example
    ///
    /// ```
    /// use joda_rs::Month;
    ///
    /// let january = Month::January;
    /// assert_eq!(january.max_length(false), 31);
    ///
    /// let february = Month::February;
    /// assert_eq!(february.max_length(false), 29);
    /// ```
    pub fn max_length(self) -> i32 { self.length(true) }

    /// Adds the specified number of months to the current `Month` value and returns the resulting `Month`.
    ///
    /// This method performs month arithmetic in a cyclic manner, ensuring the result always falls
    /// within the valid range of months (1 through 12). The calculation wraps around if the
    /// `months` parameter causes the result to go beyond December or before January.
    ///
    /// # Parameters
    ///
    /// * `months` - The number of months to add. Can be positive (to move forward in time)
    ///   or negative (to move backward in time).
    ///
    /// # Returns
    ///
    /// A `Month` instance representing the resulting month after adding the specified number
    /// of months to the month.
    ///
    /// # Examples
    ///
    /// ```
    /// use joda_rs::Month;
    ///
    /// let result = Month::May.plus(8); // Adding 8 months results in January (5 + 8 = 13, wraps back to 1)
    /// assert_eq!(result, Month::January);
    ///
    /// let result = Month::May.plus(-7); // Subtracting 7 months results in October (May - 7 months wraps backward)
    /// assert_eq!(result, Month::October);
    /// ```
    ///
    /// # Notes
    ///
    /// * Internally, the calculation converts the month to zero-based indexing
    ///   (0 for January, 11 for December) to facilitate modular arithmetic.
    /// * The result is calculated using modulo 12 arithmetic to ensure cyclic behavior.
    /// * The returned month is converted back to one-based indexing (1 for January, 12 for December).
    ///
    pub fn plus(self, months: i64) -> Self {
        let zero = (self.value() - 1) as i64;
        let wrapped = (zero + (months % 12) + 12) % 12;
        Month::of((wrapped as i32) + 1)
    }

    /// Subtracts the specified number of months from the current `Month` value and returns the resulting `Month`.
    ///
    /// This method performs month arithmetic in a cyclic manner, ensuring the result always falls
    /// within the valid range of months (1 through 12). The calculation wraps around if the
    /// `months` parameter causes the result to go beyond December or before January.
    ///
    /// # Parameters
    ///
    /// * `months` - The number of months to subtract. Can be positive (to move backward in time)
    ///   or negative (to move forward in time).
    ///
    /// # Returns
    ///
    /// A `Month` instance representing the resulting month after subtracting the specified number
    /// of months from the month.
    ///
    /// # Examples
    ///
    /// ```
    /// use joda_rs::Month;
    ///
    /// let result = Month::May.minus(8); // Subtracting 8 months results in September (May - 7 months wraps backward)
    /// assert_eq!(result, Month::September);
    ///
    /// let result = Month::May.minus(-7); // Adding 7 months results in December (5 + 8 = 13, wraps back to 1)
    /// assert_eq!(result, Month::December);
    /// ```
    ///
    /// # Notes
    ///
    /// * Internally, the calculation converts the month to zero-based indexing
    ///   (0 for January, 11 for December) to facilitate modular arithmetic.
    /// * The result is calculated using modulo 12 arithmetic to ensure cyclic behavior.
    /// * The returned month is converted back to one-based indexing (1 for January, 12 for December).
    ///
    pub fn minus(self, months: i64) -> Self { self.plus(-(months)) }
}

impl core::fmt::Display for Month {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let s = match self {
            Month::January => "JANUARY",
            Month::February => "FEBRUARY",
            Month::March => "MARCH",
            Month::April => "APRIL",
            Month::May => "MAY",
            Month::June => "JUNE",
            Month::July => "JULY",
            Month::August => "AUGUST",
            Month::September => "SEPTEMBER",
            Month::October => "OCTOBER",
            Month::November => "NOVEMBER",
            Month::December => "DECEMBER",
        };
        f.write_str(s)
    }
}

impl From<time::Month> for Month {
    fn from(value: time::Month) -> Self {
        match value {
            time::Month::January => Month::January,
            time::Month::February => Month::February,
            time::Month::March => Month::March,
            time::Month::April => Month::April,
            time::Month::May => Month::May,
            time::Month::June => Month::June,
            time::Month::July => Month::July,
            time::Month::August => Month::August,
            time::Month::September => Month::September,
            time::Month::October => Month::October,
            time::Month::November => Month::November,
            time::Month::December => Month::December,
        }
    }
}

impl From<Month> for time::Month {
    fn from(value: Month) -> Self {
        match value {
            Month::January => time::Month::January,
            Month::February => time::Month::February,
            Month::March => time::Month::March,
            Month::April => time::Month::April,
            Month::May => time::Month::May,
            Month::June => time::Month::June,
            Month::July => time::Month::July,
            Month::August => time::Month::August,
            Month::September => time::Month::September,
            Month::October => time::Month::October,
            Month::November => time::Month::November,
            Month::December => time::Month::December,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn conversion_to_from_time_month() {
        let time_may: time::Month = Month::May.into();
        let month_may: Month = Month::from(time_may);
        assert_eq!(month_may, Month::May);
    }

    #[test]
    fn from_time_month_maps_all_months() {
        use crate::Month;
        let cases = [
            (time::Month::January, Month::January),
            (time::Month::February, Month::February),
            (time::Month::March, Month::March),
            (time::Month::April, Month::April),
            (time::Month::May, Month::May),
            (time::Month::June, Month::June),
            (time::Month::July, Month::July),
            (time::Month::August, Month::August),
            (time::Month::September, Month::September),
            (time::Month::October, Month::October),
            (time::Month::November, Month::November),
            (time::Month::December, Month::December),
        ];
        for (t, expected) in cases {
            let got: Month = t.into();
            assert_eq!(got, expected, "mapping for {:?}", t);
        }
    }

    #[test]
    fn into_time_month_maps_all_months() {
        use crate::Month;
        let cases = [
            (Month::January, time::Month::January),
            (Month::February, time::Month::February),
            (Month::March, time::Month::March),
            (Month::April, time::Month::April),
            (Month::May, time::Month::May),
            (Month::June, time::Month::June),
            (Month::July, time::Month::July),
            (Month::August, time::Month::August),
            (Month::September, time::Month::September),
            (Month::October, time::Month::October),
            (Month::November, time::Month::November),
            (Month::December, time::Month::December),
        ];
        for (m, expected) in cases {
            let got: time::Month = m.into();
            assert_eq!(got, expected, "mapping for {:?}", m);
        }
    }
}
