use crate::{LocalDate, Month, Year};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct MonthDay {
    month: Month,
    day: u8,
}

impl MonthDay {
    pub fn of(month: i32, day: u8) -> Self {
        let m = Month::of(month);
        Self::of_month_day(m, day)
    }

    pub fn of_month_day(month: Month, day: u8) -> Self {
        // Validate day within the maximum days a month can have in any year.
        // For MonthDay, February 29 is allowed (independent of year), but not Feb 30.
        if day == 0 { panic!("day must be 1..=31") }
        let max_any_year: u8 = match month {
            Month::January | Month::March | Month::May | Month::July |
            Month::August | Month::October | Month::December => 31,
            Month::April | Month::June | Month::September | Month::November => 30,
            Month::February => 29,
        };
        if day > max_any_year { panic!("invalid day for month") }
        MonthDay { month, day }
    }

    // Queries
    pub fn month(self) -> Month { self.month }
    pub fn month_value(self) -> i32 { self.month.value() }
    pub fn day_of_month(self) -> u8 { self.day }

    // Comparisons
    pub fn is_before(self, other: MonthDay) -> bool {
        (self.month_value(), self.day) < (other.month_value(), other.day)
    }
    pub fn is_after(self, other: MonthDay) -> bool {
        (self.month_value(), self.day) > (other.month_value(), other.day)
    }
    pub fn is_on_or_before(self, other: MonthDay) -> bool { !self.is_after(other) }
    pub fn is_on_or_after(self, other: MonthDay) -> bool { !self.is_before(other) }

    // MonthDay.atYear(int) -> LocalDate, clamping day if necessary (e.g., Feb 29 -> Feb 28 on non-leap year)
    pub fn at_year(self, year: i32) -> LocalDate {
        let leap = Year::of(year).is_leap();
        let max_for_year = self.month.length(leap);
        let mut day = self.day as i32;
        if day > max_for_year { day = max_for_year; }
        let tm: time::Month = self.month.into();
        let d = time::Date::from_calendar_date(year, tm, day as u8).expect("invalid date");
        LocalDate::from(d)
    }
}
