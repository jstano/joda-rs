#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct LocalDateTime(pub time::PrimitiveDateTime);

impl From<time::PrimitiveDateTime> for LocalDateTime { fn from(t: time::PrimitiveDateTime) -> Self { LocalDateTime(t) } }
impl From<LocalDateTime> for time::PrimitiveDateTime { fn from(w: LocalDateTime) -> Self { w.0 } }
impl LocalDateTime {
    pub fn inner(&self) -> &time::PrimitiveDateTime { &self.0 }

    // --- Queries ---
    pub fn year(&self) -> i32 { self.0.date().year() }
    pub fn month(&self) -> crate::Month { self.0.date().month().into() }
    pub fn month_value(&self) -> i32 { self.month().value() }
    pub fn day(&self) -> u8 { self.0.date().day() }
    pub fn hour(&self) -> u8 { self.0.time().hour() }
    pub fn minute(&self) -> u8 { self.0.time().minute() }
    pub fn second(&self) -> u8 { self.0.time().second() }
    pub fn nano(&self) -> u32 { self.0.time().nanosecond() }

    pub fn day_of_year(&self) -> u16 { self.0.date().ordinal() }
    pub fn day_of_week(&self) -> crate::DayOfWeek { crate::DayOfWeek::from(self.0.date().weekday()) }

    // java.time.LocalDateTime.now()
    pub fn now() -> Self {
        let odt = time::OffsetDateTime::now_utc();
        LocalDateTime(time::PrimitiveDateTime::new(odt.date(), odt.time()))
    }

    // java.time.LocalDateTime.of(date, time)
    pub fn of_date_time(date: crate::LocalDate, time_: crate::LocalTime) -> Self {
        LocalDateTime(time::PrimitiveDateTime::new(date.into(), time_.into()))
    }

    // java.time.LocalDateTime.of(y,m,d,h,min,sec)
    pub fn of(y: i32, m: u8, d: u8, h: u8, min: u8, s: u8) -> Self {
        let date = crate::LocalDate::of(y, m, d);
        let time = crate::LocalTime::of(h, min, s);
        Self::of_date_time(date, time)
    }

    // --- Arithmetic with Duration (days/weeks/hours/minutes/seconds/nanos) ---
    pub fn plus_days(self, days: i64) -> Self { LocalDateTime(self.0.saturating_add(time::Duration::days(days))) }
    pub fn minus_days(self, days: i64) -> Self { LocalDateTime(self.0.saturating_sub(time::Duration::days(days))) }

    pub fn plus_weeks(self, weeks: i64) -> Self { self.plus_days(weeks.saturating_mul(7)) }
    pub fn minus_weeks(self, weeks: i64) -> Self { self.plus_weeks(-weeks) }

    pub fn plus_hours(self, hours: i64) -> Self { LocalDateTime(self.0.saturating_add(time::Duration::hours(hours))) }
    pub fn minus_hours(self, hours: i64) -> Self { LocalDateTime(self.0.saturating_sub(time::Duration::hours(hours))) }

    pub fn plus_minutes(self, minutes: i64) -> Self { LocalDateTime(self.0.saturating_add(time::Duration::minutes(minutes))) }
    pub fn minus_minutes(self, minutes: i64) -> Self { LocalDateTime(self.0.saturating_sub(time::Duration::minutes(minutes))) }

    pub fn plus_seconds(self, seconds: i64) -> Self { LocalDateTime(self.0.saturating_add(time::Duration::seconds(seconds))) }
    pub fn minus_seconds(self, seconds: i64) -> Self { LocalDateTime(self.0.saturating_sub(time::Duration::seconds(seconds))) }

    pub fn plus_nanos(self, nanos: i64) -> Self { LocalDateTime(self.0.saturating_add(time::Duration::nanoseconds(nanos))) }
    pub fn minus_nanos(self, nanos: i64) -> Self { LocalDateTime(self.0.saturating_sub(time::Duration::nanoseconds(nanos))) }

    // --- Months/Years via date-part adjustments with clamping handled by LocalDate ---
    pub fn plus_months(self, months: i64) -> Self {
        let date: crate::LocalDate = self.0.date().into();
        let time: crate::LocalTime = self.0.time().into();
        let new_date = date.plus_months(months);
        LocalDateTime::of_date_time(new_date, time)
    }
    pub fn minus_months(self, months: i64) -> Self { self.plus_months(-months) }

    pub fn plus_years(self, years: i64) -> Self {
        let date: crate::LocalDate = self.0.date().into();
        let time: crate::LocalTime = self.0.time().into();
        let new_date = date.plus_years(years);
        LocalDateTime::of_date_time(new_date, time)
    }
    pub fn minus_years(self, years: i64) -> Self { self.plus_years(-years) }

    // --- with_* adjusters for date part ---
    pub fn with_day_of_month(self, day: u8) -> Self {
        let date: crate::LocalDate = self.0.date().into();
        let time: crate::LocalTime = self.0.time().into();
        LocalDateTime::of_date_time(date.with_day_of_month(day), time)
    }
    pub fn with_day_of_year(self, day_of_year: u16) -> Self {
        let date: crate::LocalDate = self.0.date().into();
        let time: crate::LocalTime = self.0.time().into();
        LocalDateTime::of_date_time(date.with_day_of_year(day_of_year), time)
    }
    pub fn with_month(self, month: i32) -> Self {
        let date: crate::LocalDate = self.0.date().into();
        let time: crate::LocalTime = self.0.time().into();
        LocalDateTime::of_date_time(date.with_month(month), time)
    }
    pub fn with_year(self, year: i32) -> Self {
        let date: crate::LocalDate = self.0.date().into();
        let time: crate::LocalTime = self.0.time().into();
        LocalDateTime::of_date_time(date.with_year(year), time)
    }

    // --- with_* adjusters for time part ---
    pub fn with_hour(self, hour: u8) -> Self {
        let date = self.0.date();
        let time: crate::LocalTime = self.0.time().into();
        let new_time = time.with_hour(hour);
        LocalDateTime(time::PrimitiveDateTime::new(date, new_time.into()))
    }
    pub fn with_minute(self, minute: u8) -> Self {
        let date = self.0.date();
        let time: crate::LocalTime = self.0.time().into();
        let new_time = time.with_minute(minute);
        LocalDateTime(time::PrimitiveDateTime::new(date, new_time.into()))
    }
    pub fn with_second(self, second: u8) -> Self {
        let date = self.0.date();
        let time: crate::LocalTime = self.0.time().into();
        let new_time = time.with_second(second);
        LocalDateTime(time::PrimitiveDateTime::new(date, new_time.into()))
    }
    pub fn with_nano(self, nano: u32) -> Self {
        let date = self.0.date();
        let time: crate::LocalTime = self.0.time().into();
        let new_time = time.with_nano(nano);
        LocalDateTime(time::PrimitiveDateTime::new(date, new_time.into()))
    }

    // --- comparisons ---
    pub fn is_before(&self, other: LocalDateTime) -> bool { self.0 < other.0 }
    pub fn is_after(&self, other: LocalDateTime) -> bool { self.0 > other.0 }
    pub fn is_on_or_before(&self, other: LocalDateTime) -> bool { self.0 <= other.0 }
    pub fn is_on_or_after(&self, other: LocalDateTime) -> bool { self.0 >= other.0 }

    // --- conversions ---
    pub fn to_local_date(self) -> crate::LocalDate { self.0.date().into() }
    pub fn to_local_time(self) -> crate::LocalTime { self.0.time().into() }

    // --- date-based helpers mirroring LocalDate ones, preserving time part ---
    pub fn first_day_of_month(self) -> Self {
        let date: crate::LocalDate = self.0.date().into();
        let time: crate::LocalTime = self.0.time().into();
        let nd = date.first_day_of_month();
        Self::of_date_time(nd, time)
    }
    pub fn last_day_of_month(self) -> Self {
        let date: crate::LocalDate = self.0.date().into();
        let time: crate::LocalTime = self.0.time().into();
        let nd = date.last_day_of_month();
        Self::of_date_time(nd, time)
    }
    
    // New: last day of a specific month/year, preserving time component
    pub fn last_day_of_month_year(self, year: i32, month: i32) -> Self {
        let time: crate::LocalTime = self.0.time().into();
        let nd = crate::LocalDate::last_day_of_month_year(year, month);
        Self::of_date_time(nd, time)
    }
    pub fn first_day_of_next_month(self) -> Self {
        let date: crate::LocalDate = self.0.date().into();
        let time: crate::LocalTime = self.0.time().into();
        let nd = date.first_day_of_next_month();
        Self::of_date_time(nd, time)
    }
    pub fn first_day_of_year(self) -> Self {
        let date: crate::LocalDate = self.0.date().into();
        let time: crate::LocalTime = self.0.time().into();
        let nd = date.first_day_of_year();
        Self::of_date_time(nd, time)
    }
    pub fn first_day_of_next_year(self) -> Self {
        let date: crate::LocalDate = self.0.date().into();
        let time: crate::LocalTime = self.0.time().into();
        let nd = date.first_day_of_next_year();
        Self::of_date_time(nd, time)
    }
    pub fn last_day_of_year(self) -> Self {
        let date: crate::LocalDate = self.0.date().into();
        let time: crate::LocalTime = self.0.time().into();
        let nd = date.last_day_of_year();
        Self::of_date_time(nd, time)
    }
    pub fn first_in_month(self, dow: crate::DayOfWeek) -> Self {
        let date: crate::LocalDate = self.0.date().into();
        let time: crate::LocalTime = self.0.time().into();
        let nd = date.first_in_month(dow);
        Self::of_date_time(nd, time)
    }
    pub fn last_in_month(self, dow: crate::DayOfWeek) -> Self {
        let date: crate::LocalDate = self.0.date().into();
        let time: crate::LocalTime = self.0.time().into();
        let nd = date.last_in_month(dow);
        Self::of_date_time(nd, time)
    }
    pub fn next(self, dow: crate::DayOfWeek) -> Self {
        let date: crate::LocalDate = self.0.date().into();
        let time: crate::LocalTime = self.0.time().into();
        let nd = date.next(dow);
        Self::of_date_time(nd, time)
    }
    pub fn next_or_same(self, dow: crate::DayOfWeek) -> Self {
        let date: crate::LocalDate = self.0.date().into();
        let time: crate::LocalTime = self.0.time().into();
        let nd = date.next_or_same(dow);
        Self::of_date_time(nd, time)
    }
    pub fn previous(self, dow: crate::DayOfWeek) -> Self {
        let date: crate::LocalDate = self.0.date().into();
        let time: crate::LocalTime = self.0.time().into();
        let nd = date.previous(dow);
        Self::of_date_time(nd, time)
    }
    pub fn previous_or_same(self, dow: crate::DayOfWeek) -> Self {
        let date: crate::LocalDate = self.0.date().into();
        let time: crate::LocalTime = self.0.time().into();
        let nd = date.previous_or_same(dow);
        Self::of_date_time(nd, time)
    }
}

impl core::ops::Sub for LocalDateTime {
    type Output = crate::Duration;
    fn sub(self, rhs: Self) -> Self::Output {
        let d: time::Duration = self.0 - rhs.0;
        crate::Duration::from(d)
    }
}
