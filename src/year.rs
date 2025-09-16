#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Year(pub i32);

impl Year {
    pub fn of(value: i32) -> Self { Year(value) }
    pub fn get_value(self) -> i32 { self.0 }

    pub fn now() -> Self {
        // Use UTC clock; java.time uses system default zone, but we avoid tz here.
        let y = time::OffsetDateTime::now_utc().date().year();
        Year(y)
    }

    pub fn is_leap(self) -> bool {
        let y = self.0;
        (y % 4 == 0) && ((y % 100 != 0) || (y % 400 == 0))
    }

    pub fn length(self) -> i32 { if self.is_leap() { 366 } else { 365 } }

    pub fn plus(self, years: i64) -> Self {
        let v = self.0 as i64 + years;
        if v > i32::MAX as i64 || v < i32::MIN as i64 { panic!("year overflow") }
        Year(v as i32)
    }
    pub fn minus(self, years: i64) -> Self { self.plus(-years) }

    // java.time.Year.atMonth(int) returns YearMonth; we don't have YearMonth.
    // Provide conveniences to build LocalDate like Year.at_month_day(Month, int) and Year.at_day(int).
    pub fn at_month_day(self, month: crate::Month, day: u8) -> crate::LocalDate {
        let m: time::Month = month.into();
        let d = time::Date::from_calendar_date(self.0, m, day).expect("invalid date");
        crate::LocalDate(d)
    }

    // java.time.Year.atDay(int) — day-of-year (1-365/366)
    pub fn at_day(self, day_of_year: u16) -> crate::LocalDate {
        if day_of_year == 0 { panic!("day-of-year must be 1..=365/366") }
        let jan1 = time::Date::from_calendar_date(self.0, time::Month::January, 1).unwrap();
        let doy = day_of_year as i64 - 1;
        crate::LocalDate(jan1.saturating_add(time::Duration::days(doy)))
    }
}

impl core::fmt::Display for Year {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result { write!(f, "{}", self.0) }
}
