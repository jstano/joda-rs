#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct OffsetDateTime(pub time::OffsetDateTime);

impl From<time::OffsetDateTime> for OffsetDateTime { fn from(t: time::OffsetDateTime) -> Self { OffsetDateTime(t) } }
impl From<OffsetDateTime> for time::OffsetDateTime { fn from(w: OffsetDateTime) -> Self { w.0 } }
impl OffsetDateTime {
    pub fn inner(&self) -> &time::OffsetDateTime { &self.0 }

    // java.time.OffsetDateTime.now()
    pub fn now_utc() -> Self { OffsetDateTime(time::OffsetDateTime::now_utc()) }

    pub fn now_with_clock(clock: crate::Clock) -> Self {
        // Use the provided Clock; we only support UTC offset at the moment
        clock
            .instant()
            .at_offset(crate::ZoneOffset::of_hours(0))
    }

    // java.time.OffsetDateTime.of(LocalDateTime, ZoneOffset)
    pub fn of(ldt: crate::LocalDateTime, offset: crate::ZoneOffset) -> Self {
        let pdt: time::PrimitiveDateTime = ldt.into();
        let off: time::UtcOffset = offset.into();
        OffsetDateTime(pdt.assume_offset(off))
    }

    // --- Time-based arithmetic ---
    pub fn plus_hours(self, hours: i64) -> Self { OffsetDateTime(self.0 + time::Duration::hours(hours)) }
    pub fn minus_hours(self, hours: i64) -> Self { OffsetDateTime(self.0 - time::Duration::hours(hours)) }

    pub fn plus_minutes(self, minutes: i64) -> Self { OffsetDateTime(self.0 + time::Duration::minutes(minutes)) }
    pub fn minus_minutes(self, minutes: i64) -> Self { OffsetDateTime(self.0 - time::Duration::minutes(minutes)) }

    pub fn plus_seconds(self, seconds: i64) -> Self { OffsetDateTime(self.0 + time::Duration::seconds(seconds)) }
    pub fn minus_seconds(self, seconds: i64) -> Self { OffsetDateTime(self.0 - time::Duration::seconds(seconds)) }

    pub fn plus_nanos(self, nanos: i64) -> Self { OffsetDateTime(self.0 + time::Duration::nanoseconds(nanos)) }
    pub fn minus_nanos(self, nanos: i64) -> Self { OffsetDateTime(self.0 - time::Duration::nanoseconds(nanos)) }

    // --- Date-based arithmetic ---
    pub fn plus_days(self, days: i64) -> Self { OffsetDateTime(self.0 + time::Duration::days(days)) }
    pub fn minus_days(self, days: i64) -> Self { OffsetDateTime(self.0 - time::Duration::days(days)) }

    pub fn plus_weeks(self, weeks: i64) -> Self { self.plus_days(weeks * 7) }
    pub fn minus_weeks(self, weeks: i64) -> Self { self.minus_days(weeks * 7) }

    pub fn plus_months(self, months: i64) -> Self {
        let date = self.0.date();
        let time_ = self.0.time();
        let pdt = time::PrimitiveDateTime::new(date, time_);
        let ldt: crate::LocalDateTime = crate::LocalDateTime::from(pdt);
        let ldt2 = ldt.plus_months(months);
        let pdt2: time::PrimitiveDateTime = ldt2.into();
        let off = self.0.offset();
        OffsetDateTime(pdt2.assume_offset(off))
    }
    pub fn minus_months(self, months: i64) -> Self { self.plus_months(-months) }

    pub fn plus_years(self, years: i64) -> Self {
        let date = self.0.date();
        let time_ = self.0.time();
        let pdt = time::PrimitiveDateTime::new(date, time_);
        let ldt: crate::LocalDateTime = crate::LocalDateTime::from(pdt);
        let ldt2 = ldt.plus_years(years);
        let pdt2: time::PrimitiveDateTime = ldt2.into();
        let off = self.0.offset();
        OffsetDateTime(pdt2.assume_offset(off))
    }
    pub fn minus_years(self, years: i64) -> Self { self.plus_years(-years) }

    // --- Comparisons ---
    pub fn is_before(&self, other: OffsetDateTime) -> bool { self.0 < other.0 }
    pub fn is_after(&self, other: OffsetDateTime) -> bool { self.0 > other.0 }
    pub fn is_on_or_before(&self, other: OffsetDateTime) -> bool { self.0 <= other.0 }
    pub fn is_on_or_after(&self, other: OffsetDateTime) -> bool { self.0 >= other.0 }

    // --- Withers (setters) ---
    pub fn with_day_of_month(self, day: u8) -> Self {
        let pdt = time::PrimitiveDateTime::new(self.0.date(), self.0.time());
        let ldt: crate::LocalDateTime = pdt.into();
        let ldt2 = ldt.with_day_of_month(day);
        let pdt2: time::PrimitiveDateTime = ldt2.into();
        let off = self.0.offset();
        OffsetDateTime(pdt2.assume_offset(off))
    }
    pub fn with_day_of_year(self, day_of_year: u16) -> Self {
        let pdt = time::PrimitiveDateTime::new(self.0.date(), self.0.time());
        let ldt: crate::LocalDateTime = pdt.into();
        let ldt2 = ldt.with_day_of_year(day_of_year);
        let pdt2: time::PrimitiveDateTime = ldt2.into();
        let off = self.0.offset();
        OffsetDateTime(pdt2.assume_offset(off))
    }
    pub fn with_month(self, month: i32) -> Self {
        let pdt = time::PrimitiveDateTime::new(self.0.date(), self.0.time());
        let ldt: crate::LocalDateTime = pdt.into();
        let ldt2 = ldt.with_month(month);
        let pdt2: time::PrimitiveDateTime = ldt2.into();
        let off = self.0.offset();
        OffsetDateTime(pdt2.assume_offset(off))
    }
    pub fn with_year(self, year: i32) -> Self {
        let pdt = time::PrimitiveDateTime::new(self.0.date(), self.0.time());
        let ldt: crate::LocalDateTime = pdt.into();
        let ldt2 = ldt.with_year(year);
        let pdt2: time::PrimitiveDateTime = ldt2.into();
        let off = self.0.offset();
        OffsetDateTime(pdt2.assume_offset(off))
    }
    pub fn with_hour(self, hour: u8) -> Self {
        let pdt = time::PrimitiveDateTime::new(self.0.date(), self.0.time());
        let ldt: crate::LocalDateTime = pdt.into();
        let ldt2 = ldt.with_hour(hour);
        let pdt2: time::PrimitiveDateTime = ldt2.into();
        let off = self.0.offset();
        OffsetDateTime(pdt2.assume_offset(off))
    }
    pub fn with_minute(self, minute: u8) -> Self {
        let pdt = time::PrimitiveDateTime::new(self.0.date(), self.0.time());
        let ldt: crate::LocalDateTime = pdt.into();
        let ldt2 = ldt.with_minute(minute);
        let pdt2: time::PrimitiveDateTime = ldt2.into();
        let off = self.0.offset();
        OffsetDateTime(pdt2.assume_offset(off))
    }
    pub fn with_second(self, second: u8) -> Self {
        let pdt = time::PrimitiveDateTime::new(self.0.date(), self.0.time());
        let ldt: crate::LocalDateTime = pdt.into();
        let ldt2 = ldt.with_second(second);
        let pdt2: time::PrimitiveDateTime = ldt2.into();
        let off = self.0.offset();
        OffsetDateTime(pdt2.assume_offset(off))
    }
    pub fn with_nano(self, nano: u32) -> Self {
        let pdt = time::PrimitiveDateTime::new(self.0.date(), self.0.time());
        let ldt: crate::LocalDateTime = pdt.into();
        let ldt2 = ldt.with_nano(nano);
        let pdt2: time::PrimitiveDateTime = ldt2.into();
        let off = self.0.offset();
        OffsetDateTime(pdt2.assume_offset(off))
    }

    // --- Conversions ---
    pub fn to_local_date(self) -> crate::LocalDate { self.0.date().into() }
    pub fn to_local_time(self) -> crate::LocalTime { self.0.time().into() }
    pub fn to_local_date_time(self) -> crate::LocalDateTime {
        let pdt = time::PrimitiveDateTime::new(self.0.date(), self.0.time());
        pdt.into()
    }
}
