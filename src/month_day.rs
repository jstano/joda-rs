#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct MonthDay {
    month: crate::Month,
    day: u8,
}

impl MonthDay {
    // Constructors (not explicitly requested but useful and consistent with other types)
    pub fn of(month: i32, day: u8) -> Self {
        let m = crate::Month::of(month);
        Self::of_month_day(m, day)
    }

    pub fn of_month_day(month: crate::Month, day: u8) -> Self {
        // Validate day within the maximum days a month can have in any year.
        // For MonthDay, February 29 is allowed (independent of year), but not Feb 30.
        if day == 0 { panic!("day must be 1..=31") }
        let max_any_year: u8 = match month {
            crate::Month::January | crate::Month::March | crate::Month::May | crate::Month::July |
            crate::Month::August | crate::Month::October | crate::Month::December => 31,
            crate::Month::April | crate::Month::June | crate::Month::September | crate::Month::November => 30,
            crate::Month::February => 29,
        };
        if day > max_any_year { panic!("invalid day for month") }
        MonthDay { month, day }
    }

    // Queries
    pub fn month(&self) -> crate::Month { self.month }
    pub fn month_value(&self) -> i32 { self.month.value() }
    pub fn day_of_month(&self) -> u8 { self.day }

    // Comparisons
    pub fn is_before(&self, other: MonthDay) -> bool {
        (self.month_value(), self.day) < (other.month_value(), other.day)
    }
    pub fn is_after(&self, other: MonthDay) -> bool {
        (self.month_value(), self.day) > (other.month_value(), other.day)
    }
    pub fn is_on_or_before(&self, other: MonthDay) -> bool { !self.is_after(other) }
    pub fn is_on_or_after(&self, other: MonthDay) -> bool { !self.is_before(other) }

    // MonthDay.atYear(int) -> LocalDate, clamping day if necessary (e.g., Feb 29 -> Feb 28 on non-leap year)
    pub fn at_year(self, year: i32) -> crate::LocalDate {
        let leap = crate::Year::of(year).is_leap();
        let max_for_year = self.month.length(leap);
        let mut day = self.day as u8;
        if day > max_for_year { day = max_for_year; }
        let tm: time::Month = self.month.into();
        let d = time::Date::from_calendar_date(year, tm, day).expect("invalid date");
        crate::LocalDate(d)
    }
}
