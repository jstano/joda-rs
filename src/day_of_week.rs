/// Represents the days of the week as an enumerated type.
///
/// This enum associates each day of the week with a corresponding integer value
/// (starting from 1 for `Monday` and ending with 7 for `Sunday`).
/// It derives several traits to enable common operations such as ordering, hashing,
/// equality checks, and debugging.
///
/// # Variants
///
/// - `Monday`: Represents the first day of the week, assigned a value of 1.
/// - `Tuesday`: Represents the second day of the week, assigned a value of 2.
/// - `Wednesday`: Represents the third day of the week, assigned a value of 3.
/// - `Thursday`: Represents the fourth day of the week, assigned a value of 4.
/// - `Friday`: Represents the fifth day of the week, assigned a value of 5.
/// - `Saturday`: Represents the sixth day of the week, assigned a value of 6.
/// - `Sunday`: Represents the seventh day of the week, assigned a value of 7.
///
/// # Derivable Traits
///
/// - `Debug`: Automatically provides a human-readable string representation
///   of the enum for debugging purposes.
/// - `Clone`: Enables creating a duplicate of the enum instance.
/// - `Copy`: Allows the enum to be copied bitwise instead of moved, as it
///   is small and fixed in size.
/// - `PartialEq` and `Eq`: Enable comparison for equality and inequality.
/// - `Hash`: Allows the enum to be used in hashed collections like `HashSet`
///   or `HashMap`.
/// - `PartialOrd` and `Ord`: Provide comparison operations for ordering
///   (`<`, `<=`, `>`, `>=`).
///
/// # Example
///
/// ```
/// use joda_rs::DayOfWeek;
///
/// let today = DayOfWeek::Wednesday;
/// println!("Day number: {}", today.value()); // Outputs: Day number: 3
/// ```
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum DayOfWeek {
    Monday = 1,
    Tuesday = 2,
    Wednesday = 3,
    Thursday = 4,
    Friday = 5,
    Saturday = 6,
    Sunday = 7,
}

impl DayOfWeek {
    /// Converts an i32 value into the corresponding `DayOfWeek` enum variant.
    ///
    /// # Parameters
    /// - `value`: An integer representing the day of the week. It must be in the range 1 through 7:
    ///   - `1` maps to `DayOfWeek::Monday`
    ///   - `2` maps to `DayOfWeek::Tuesday`
    ///   - `3` maps to `DayOfWeek::Wednesday`
    ///   - `4` maps to `DayOfWeek::Thursday`
    ///   - `5` maps to `DayOfWeek::Friday`
    ///   - `6` maps to `DayOfWeek::Saturday`
    ///   - `7` maps to `DayOfWeek::Sunday`
    ///
    /// # Returns
    /// Returns the corresponding `DayOfWeek` enum variant for a valid input value.
    ///
    /// # Panics
    /// This function will panic if the input value is not within the range `1` to `7`.
    ///
    /// # Examples
    /// ```rust
    /// let monday = DayOfWeek::of(1);
    /// assert_eq!(monday, DayOfWeek::Monday);
    ///
    /// let friday = DayOfWeek::of(5);
    /// assert_eq!(friday, DayOfWeek::Friday);
    ///
    /// // This will panic:
    /// // let invalid_day = DayOfWeek::of(8);
    /// ```
    pub fn of(value: i32) -> Self {
        match value {
            1 => DayOfWeek::Monday,
            2 => DayOfWeek::Tuesday,
            3 => DayOfWeek::Wednesday,
            4 => DayOfWeek::Thursday,
            5 => DayOfWeek::Friday,
            6 => DayOfWeek::Saturday,
            7 => DayOfWeek::Sunday,
            _ => panic!("invalid day-of-week value (1-7)"),
        }
    }

    /// Attempts to create a `DayOfWeek` enum instance from an integer value.
    ///
    /// # Arguments
    /// * `value` - An integer representing the day of the week (1 through 7).
    ///
    /// # Returns
    /// * `Ok(DayOfWeek)` if the `value` corresponds to a valid day of the week:
    ///   * `1` -> `DayOfWeek::Monday`
    ///   * `2` -> `DayOfWeek::Tuesday`
    ///   * `3` -> `DayOfWeek::Wednesday`
    ///   * `4` -> `DayOfWeek::Thursday`
    ///   * `5` -> `DayOfWeek::Friday`
    ///   * `6` -> `DayOfWeek::Saturday`
    ///   * `7` -> `DayOfWeek::Sunday`
    /// * `Err(&'static str)` if the `value` is not within the valid range (1-7),
    ///   in which case the error message `"invalid day-of-week value (1-7)"` is returned.
    ///
    /// # Example
    /// ```
    /// let day = DayOfWeek::try_of(3);
    /// assert_eq!(day, Ok(DayOfWeek::Wednesday));
    ///
    /// let invalid_day = DayOfWeek::try_of(8);
    /// assert_eq!(invalid_day, Err("invalid day-of-week value (1-7)"));
    /// ```
    pub fn try_of(value: i32) -> Result<Self, &'static str> {
        match value {
            1 => Ok(DayOfWeek::Monday),
            2 => Ok(DayOfWeek::Tuesday),
            3 => Ok(DayOfWeek::Wednesday),
            4 => Ok(DayOfWeek::Thursday),
            5 => Ok(DayOfWeek::Friday),
            6 => Ok(DayOfWeek::Saturday),
            7 => Ok(DayOfWeek::Sunday),
            _ => Err("invalid day-of-week value (1-7)"),
        }
    }

    /// Converts the DayOfWeek instance to it's `i32` value.
    ///
    /// # Returns
    /// - `i32`: The integer representation of `DayOfWeek` as an `i32`.
    ///
    /// # Example
    /// ```
    /// use joda_rs::DayOfWeek;
    ///
    /// let today = DayOfWeek::Wednesday;
    /// println!("Day number: {}", today.value()); // Outputs: Day number: 3
    /// ```
    pub fn value(self) -> i32 {
        self as i32
    }

    /// Determines if the day of the week is a weekend.
    ///
    /// # Returns
    /// * `true` - If the current day is either Saturday or Sunday.
    /// * `false` - If the current day is not a weekend (Monday through Friday).
    ///
    /// # Examples
    /// ```
    /// use joda_rs::DayOfWeek;
    ///
    /// let saturday = DayOfWeek::Saturday;
    /// let monday = DayOfWeek::Monday;
    ///
    /// assert!(saturday.is_weekend());
    /// assert!(!monday.is_weekend());
    /// ```
    pub fn is_weekend(self) -> bool {
        self == DayOfWeek::Saturday || self == DayOfWeek::Sunday
    }

    /// Determines if the day of the week is a weekday.
    ///
    /// # Returns
    /// * `true` - If the day is a weekday (Monday through Friday).
    /// * `false` - If the day is not a weekday (i.e., it is a weekend, such as Saturday or Sunday).
    ///
    /// # Example
    /// ```
    /// use joda_rs::DayOfWeek;
    ///
    /// let saturday = DayOfWeek::Saturday;
    /// let monday = DayOfWeek::Monday;
    ///
    /// assert!(!saturday.is_weekday());
    /// assert!(monday.is_weekday());
    /// ```
    pub fn is_weekday(self) -> bool {
        !self.is_weekend()
    }

    /// Adds a specified number of days to the current `DayOfWeek` and returns the resulting `DayOfWeek`.
    ///
    /// This method performs the addition in a cyclic manner, ensuring the result always falls within a valid weekday range (Monday to Sunday).
    ///
    /// # Parameters
    /// - `self`: The starting `DayOfWeek` instance.
    /// - `days`: The number of days to add. Can be positive (to move forward) or negative (to move backward).
    ///
    /// # Returns
    /// A new `DayOfWeek` instance that represents the day after adding the specified number of days to the current day.
    ///
    /// # Algorithm
    /// 1. Normalize the current `DayOfWeek` (`self`) value to a zero-based index (0 for Monday to 6 for Sunday).
    /// 2. Add the given number of `days` (mod 7), ensuring the result stays within a valid range by wrapping around if necessary (handles negative values as well).
    /// 3. Map the zero-based result back to the one-based `DayOfWeek` representation and return it.
    ///
    /// # Example
    /// ```
    /// use joda_rs::DayOfWeek;
    ///
    /// let tuesday = DayOfWeek::Tuesday;
    /// let sunday = tuesday.plus(5); // Adds 5 days to Tuesday
    /// assert_eq!(sunday.value(), 7); // Sunday = 7
    ///
    /// let friday = sunday.plus(-2); // Subtracts 2 days from Sunday
    /// assert_eq!(friday.value(), 5); // Friday = 5
    /// ```
    pub fn plus(self, days: i64) -> Self {
        // Normalize to 0..6, add, wrap, then map back to 1..7
        let zero_based = (self.value() - 1) as i64;
        let wrapped = (zero_based + (days % 7) + 7) % 7; // handle negative
        DayOfWeek::of((wrapped as i32) + 1)
    }

    /// Subtracts a specified number of days to the current `DayOfWeek` and returns the resulting `DayOfWeek`.
    ///
    /// This method performs the subtraction in a cyclic manner, ensuring the result always falls within a valid weekday range (Monday to Sunday).
    ///
    /// # Parameters
    /// - `self`: The starting `DayOfWeek` instance.
    /// - `days`: The number of days to subtract. Can be positive (to move backward) or negative (to move forward).
    ///
    /// # Returns
    /// A new `DayOfWeek` instance that represents the day after subtracting the specified number of days from the current day.
    ///
    /// # Algorithm
    /// 1. Normalize the current `DayOfWeek` (`self`) value to a zero-based index (0 for Monday to 6 for Sunday).
    /// 2. Subtract the given number of `days` (mod 7), ensuring the result stays within a valid range by wrapping around if necessary (handles negative values as well).
    /// 3. Map the zero-based result back to the one-based `DayOfWeek` representation and return it.
    ///
    /// # Example
    /// ```
    /// use joda_rs::DayOfWeek;
    ///
    /// let tuesday = DayOfWeek::Tuesday;
    /// let thursday = tuesday.minus(5); // Subtracts 5 days from Tuesday
    /// assert_eq!(sunday.value(), 4); // Thursday = 4
    ///
    /// let friday = sunday.plus(-2); // Adds 2 days from Sunday
    /// assert_eq!(friday.value(), 5); // Friday = 5
    /// ```
    pub fn minus(self, days: i64) -> Self {
        self.plus(-(days))
    }
}

impl From<time::Weekday> for DayOfWeek {
    fn from(value: time::Weekday) -> Self {
        match value {
            time::Weekday::Monday => DayOfWeek::Monday,
            time::Weekday::Tuesday => DayOfWeek::Tuesday,
            time::Weekday::Wednesday => DayOfWeek::Wednesday,
            time::Weekday::Thursday => DayOfWeek::Thursday,
            time::Weekday::Friday => DayOfWeek::Friday,
            time::Weekday::Saturday => DayOfWeek::Saturday,
            time::Weekday::Sunday => DayOfWeek::Sunday,
        }
    }
}

impl From<DayOfWeek> for time::Weekday {
    fn from(value: DayOfWeek) -> Self {
        match value {
            DayOfWeek::Monday => time::Weekday::Monday,
            DayOfWeek::Tuesday => time::Weekday::Tuesday,
            DayOfWeek::Wednesday => time::Weekday::Wednesday,
            DayOfWeek::Thursday => time::Weekday::Thursday,
            DayOfWeek::Friday => time::Weekday::Friday,
            DayOfWeek::Saturday => time::Weekday::Saturday,
            DayOfWeek::Sunday => time::Weekday::Sunday,
        }
    }
}

impl core::fmt::Display for DayOfWeek {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let s = match self {
            DayOfWeek::Monday => "MONDAY",
            DayOfWeek::Tuesday => "TUESDAY",
            DayOfWeek::Wednesday => "WEDNESDAY",
            DayOfWeek::Thursday => "THURSDAY",
            DayOfWeek::Friday => "FRIDAY",
            DayOfWeek::Saturday => "SATURDAY",
            DayOfWeek::Sunday => "SUNDAY",
        };
        f.write_str(s)
    }
}
