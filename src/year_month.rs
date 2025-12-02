use crate::{LocalDate, Month, Year};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct YearMonth {
    year: i32,
    month: Month,
}

impl YearMonth {
    pub fn of(year: i32, month: i32) -> Self {
        let m = Month::of(month);
        YearMonth { year, month: m }
    }

    pub fn of_year_month(year: i32, month: Month) -> Self {
        YearMonth { year, month }
    }

    pub fn now() -> Self {
        let d = time::OffsetDateTime::now_utc().date();
        let year = d.year();
        let month: Month = d.month().into();
        YearMonth { year, month }
    }

    pub fn year(self) -> i32 { self.year }
    pub fn month(self) -> Month { self.month }
    pub fn month_value(self) -> i32 { self.month.value() }

    pub fn is_leap_year(self) -> bool { Year::of(self.year).is_leap() }

    pub fn length_of_month(self) -> i32 {
        let leap = self.is_leap_year();
        self.month.length(leap)
    }

    pub fn plus_months(self, months: i64) -> Self {
        // Convert to total months from year 0, add, and normalize
        let base = (self.year as i64).saturating_mul(12) + (self.month_value() as i64 - 1);
        let total = base.saturating_add(months);
        let new_year = total.div_euclid(12) as i32;
        let new_month0 = total.rem_euclid(12) as i32; // 0..11
        let new_month = Month::of(new_month0 + 1);
        YearMonth { year: new_year, month: new_month }
    }
    pub fn minus_months(self, months: i64) -> Self { self.plus_months(-months) }

    pub fn plus_years(self, years: i64) -> Self {
        let ny = self.year as i64 + years;
        if ny > i32::MAX as i64 || ny < i32::MIN as i64 { panic!("year overflow") }
        YearMonth { year: ny as i32, month: self.month }
    }
    pub fn minus_years(self, years: i64) -> Self { self.plus_years(-years) }

    pub fn with_month(self, month: i32) -> Self { YearMonth { year: self.year, month: Month::of(month) } }
    pub fn with_year(self, year: i32) -> Self { YearMonth { year, month: self.month } }

    pub fn at_day(self, day: i32) -> LocalDate {
        // Validate day based on month length and leap
        let max_day = self.length_of_month();
        if day == 0 || day > max_day { panic!("invalid day for YearMonth") }
        let tm: time::Month = self.month.into();
        let d = time::Date::from_calendar_date(self.year, tm, day as u8).expect("invalid date");
        LocalDate::from(d)
    }

    // New helpers like java.time.YearMonth
    pub fn first_day_of_month(self) -> LocalDate {
        self.at_day(1)
    }

    pub fn last_day_of_month(self) -> LocalDate {
        let last = self.length_of_month();
        self.at_day(last)
    }

    pub fn is_before(self, other: YearMonth) -> bool { (self.year, self.month) < (other.year, other.month) }
    pub fn is_after(self, other: YearMonth) -> bool { (self.year, self.month) > (other.year, other.month) }
    pub fn is_on_or_before(self, other: YearMonth) -> bool { !self.is_after(other) }
    pub fn is_on_or_after(self, other: YearMonth) -> bool { !self.is_before(other) }
}

impl core::fmt::Display for YearMonth {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{:04}-{:02}", self.year, self.month_value())
    }
}
