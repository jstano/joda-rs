#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Period {
    years: i32,
    months: i32,
    days: i32,
}

impl Period {
    pub const fn new(years: i32, months: i32, days: i32) -> Self { Self { years, months, days } }
    pub const fn of(years: i32, months: i32, days: i32) -> Self { Self { years, months, days } }
    pub const fn of_years(years: i32) -> Self { Self { years, months: 0, days: 0 } }
    pub const fn of_months(months: i32) -> Self { Self { years: 0, months, days: 0 } }
    pub const fn of_weeks(weeks: i32) -> Self { Self { years: 0, months: 0, days: weeks * 7 } }
    pub const fn of_days(days: i32) -> Self { Self { years: 0, months: 0, days } }

    pub const fn years(self) -> i32 { self.years }
    pub const fn months(self) -> i32 { self.months }
    pub const fn days(self) -> i32 { self.days }
    pub const fn total_months(self) -> i64 { (self.years as i64) * 12 + (self.months as i64) }

    pub const fn is_zero(self) -> bool { self.years == 0 && self.months == 0 && self.days == 0 }
    pub const fn is_negative(self) -> bool { self.years < 0 || self.months < 0 || self.days < 0 }
    pub const fn is_positive(self) -> bool { self.years > 0 || self.months > 0 || self.days > 0 }

    pub const fn plus(self, other: Period) -> Self { Self { years: self.years + other.years, months: self.months + other.months, days: self.days + other.days } }
    pub const fn minus(self, other: Period) -> Self { Self { years: self.years - other.years, months: self.months - other.months, days: self.days - other.days } }
    pub const fn negated(self) -> Self { Self { years: -self.years, months: -self.months, days: -self.days } }

    pub const fn plus_years(self, years: i32) -> Self { Self { years: self.years + years, ..self } }
    pub const fn plus_months(self, months: i32) -> Self { Self { months: self.months + months, ..self } }
    pub const fn plus_days(self, days: i32) -> Self { Self { days: self.days + days, ..self } }
    pub const fn minus_years(self, years: i32) -> Self { Self { years: self.years - years, ..self } }
    pub const fn minus_months(self, months: i32) -> Self { Self { months: self.months - months, ..self } }
    pub const fn minus_days(self, days: i32) -> Self { Self { days: self.days - days, ..self } }

    pub const fn with_years(self, years: i32) -> Self { Self { years, ..self } }
    pub const fn with_months(self, months: i32) -> Self { Self { months, ..self } }
    pub const fn with_days(self, days: i32) -> Self { Self { days, ..self } }
}
