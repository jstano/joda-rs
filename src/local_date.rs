#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct LocalDate(pub time::Date);

impl From<time::Date> for LocalDate {
    fn from(d: time::Date) -> Self { LocalDate(d) }
}
impl From<LocalDate> for time::Date {
    fn from(w: LocalDate) -> Self { w.0 }
}
impl LocalDate {
    pub fn inner(&self) -> &time::Date { &self.0 }

    pub fn now() -> Self {
        // Use UTC clock to avoid platform tz complexities (java uses system default zone)
        let d = time::OffsetDateTime::now_utc().date();
        LocalDate(d)
    }

    pub fn now_zone_id(_zone: crate::ZoneId) -> Self {
        // Placeholder: we currently don't have tz database support; treat all zones as UTC.
        let d = time::OffsetDateTime::now_utc().date();
        LocalDate(d)
    }

    pub fn of(year: i32, month: u8, day: u8) -> Self {
        let m = time::Month::try_from(month).expect("invalid month 1-12");
        let d = time::Date::from_calendar_date(year, m, day).expect("invalid date");
        LocalDate(d)
    }

    pub fn parse(s: &str) -> Self {
        use time::format_description::well_known::Iso8601;
        let d = time::Date::parse(s, &Iso8601::DEFAULT).expect("invalid date string");
        LocalDate(d)
    }

    pub fn at_time(self, time_: crate::LocalTime) -> crate::LocalDateTime {
        crate::LocalDateTime::of_date_time(self, time_)
    }

    pub fn at_start_of_day(self) -> crate::LocalDateTime {
        let midnight = crate::LocalTime::of(0, 0, 0);
        crate::LocalDateTime::of_date_time(self, midnight)
    }

    // New java.time-like query methods
    pub fn day_of_week(&self) -> crate::DayOfWeek {
        crate::DayOfWeek::from(self.0.weekday())
    }

    pub fn day_of_month(&self) -> u8 { self.0.day() }

    pub fn day_of_year(&self) -> u16 { self.0.ordinal() }

    pub fn year(&self) -> crate::Year { crate::Year::of(self.0.year()) }

    pub fn length_of_month(&self) -> u8 {
        let month = crate::Month::from(self.0.month());
        let leap = time::util::is_leap_year(self.0.year());
        month.length(leap)
    }

    // New methods requested
    pub fn month(&self) -> crate::Month {
        crate::Month::from(self.0.month())
    }

    pub fn month_value(&self) -> i32 {
        self.month().value()
    }

    pub fn is_leap_year(&self) -> bool {
        crate::Year::of(self.0.year()).is_leap()
    }

    pub fn length_of_year(&self) -> i32 {
        crate::Year::of(self.0.year()).length()
    }

    pub fn plus_days(self, days: i64) -> Self {
        LocalDate(self.0.saturating_add(time::Duration::days(days)))
    }

    pub fn minus_days(self, days: i64) -> Self {
        LocalDate(self.0.saturating_sub(time::Duration::days(days)))
    }

    pub fn plus_weeks(self, weeks: i64) -> Self {
        self.plus_days(weeks.saturating_mul(7))
    }
    pub fn minus_weeks(self, weeks: i64) -> Self {
        self.plus_weeks(-weeks)
    }

    pub fn plus_months(self, months: i64) -> Self {
        // Compute target year and month with wrap-around, then clamp day if needed
        let y = self.0.year() as i64;
        let m0 = (self.0.month() as i32 - 1) as i64; // 0-based month
        let total = y.saturating_mul(12).saturating_add(m0).saturating_add(months);
        let new_year = (total.div_euclid(12)) as i32;
        let new_month0 = (total.rem_euclid(12)) as i32; // 0..11
        let new_month_enum = time::Month::try_from((new_month0 + 1) as u8).unwrap();
        let max_day = match new_month_enum {
            time::Month::January => 31,
            time::Month::February => if time::util::is_leap_year(new_year) { 29 } else { 28 },
            time::Month::March => 31,
            time::Month::April => 30,
            time::Month::May => 31,
            time::Month::June => 30,
            time::Month::July => 31,
            time::Month::August => 31,
            time::Month::September => 30,
            time::Month::October => 31,
            time::Month::November => 30,
            time::Month::December => 31,
        } as u8;
        let day = self.0.day().min(max_day);
        let d = time::Date::from_calendar_date(new_year, new_month_enum, day).expect("invalid date");
        LocalDate(d)
    }
    pub fn minus_months(self, months: i64) -> Self { self.plus_months(-months) }

    pub fn plus_years(self, years: i64) -> Self {
        let target_year_i64 = self.0.year() as i64 + years;
        if target_year_i64 > i32::MAX as i64 || target_year_i64 < i32::MIN as i64 { panic!("year overflow") }
        let new_year = target_year_i64 as i32;
        let month = self.0.month();
        let mut day = self.0.day();
        // Clamp Feb 29 to Feb 28 on non-leap years
        if month == time::Month::February && day == 29 && !time::util::is_leap_year(new_year) {
            day = 28;
        }
        let d = time::Date::from_calendar_date(new_year, month, day).expect("invalid date");
        LocalDate(d)
    }
    pub fn minus_years(self, years: i64) -> Self { self.plus_years(-years) }

    pub fn with_day_of_month(self, day: u8) -> Self {
        let d = time::Date::from_calendar_date(self.0.year(), self.0.month(), day).expect("invalid date");
        LocalDate(d)
    }

    pub fn with_day_of_year(self, day_of_year: u16) -> Self {
        if day_of_year == 0 { panic!("day-of-year must be 1..=365/366") }
        let jan1 = time::Date::from_calendar_date(self.0.year(), time::Month::January, 1).unwrap();
        LocalDate(jan1.saturating_add(time::Duration::days((day_of_year as i64) - 1)))
    }

    pub fn with_month(self, month: i32) -> Self {
        let m = crate::Month::of(month);
        let tm: time::Month = m.into();
        let d = time::Date::from_calendar_date(self.0.year(), tm, self.0.day()).expect("invalid date");
        LocalDate(d)
    }

    pub fn with_year(self, year: i32) -> Self {
        let d = time::Date::from_calendar_date(year, self.0.month(), self.0.day()).expect("invalid date");
        LocalDate(d)
    }

    pub fn is_before(&self, other: LocalDate) -> bool { self.0 < other.0 }
    pub fn is_after(&self, other: LocalDate) -> bool { self.0 > other.0 }
    pub fn is_on_or_before(&self, other: LocalDate) -> bool { self.0 <= other.0 }
    pub fn is_on_or_after(&self, other: LocalDate) -> bool { self.0 >= other.0 }
}
