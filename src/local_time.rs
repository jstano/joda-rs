#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct LocalTime(pub time::Time);

impl From<time::Time> for LocalTime { fn from(t: time::Time) -> Self { LocalTime(t) } }
impl From<LocalTime> for time::Time { fn from(w: LocalTime) -> Self { w.0 } }
impl LocalTime {
    pub fn inner(&self) -> &time::Time { &self.0 }

    pub fn now() -> Self {
        let t = time::OffsetDateTime::now_utc().time();
        LocalTime(t)
    }

    pub fn of(hour: u8, minute: u8, second: u8) -> Self {
        let t = time::Time::from_hms(hour, minute, second).expect("invalid time");
        LocalTime(t)
    }

    pub fn hour(&self) -> u8 { self.0.hour() }
    pub fn minute(&self) -> u8 { self.0.minute() }
    pub fn seconds(&self) -> u8 { self.0.second() }

    fn total_nanos_of_day(&self) -> i128 {
        // h,m,s,n -> total nanoseconds
        let h = self.0.hour() as i128;
        let m = self.0.minute() as i128;
        let s = self.0.second() as i128;
        let n = self.0.nanosecond() as i128;
        (((h * 60 + m) * 60 + s) * 1_000_000_000) + n
    }
    fn from_total_nanos_of_day(total: i128) -> Self {
        // wrap around 24h like java.time
        const DAY_NANOS: i128 = 86_400_i128 * 1_000_000_000_i128;
        let mut n = total % DAY_NANOS;
        if n < 0 { n += DAY_NANOS; }
        let hour = (n / 3_600_000_000_000) as u8; // 3600*1e9
        n %= 3_600_000_000_000;
        let minute = (n / 60_000_000_000) as u8;
        n %= 60_000_000_000;
        let second = (n / 1_000_000_000) as u8;
        let nano = (n % 1_000_000_000) as u32;
        LocalTime(time::Time::from_hms_nano(hour, minute, second, nano).unwrap())
    }

    pub fn plus_hours(self, hours: i64) -> Self {
        let delta = (hours as i128) * 3_600_000_000_000_i128;
        Self::from_total_nanos_of_day(self.total_nanos_of_day() + delta)
    }
    pub fn minus_hours(self, hours: i64) -> Self { self.plus_hours(-hours) }

    pub fn plus_minutes(self, minutes: i64) -> Self {
        let delta = (minutes as i128) * 60_000_000_000_i128;
        Self::from_total_nanos_of_day(self.total_nanos_of_day() + delta)
    }
    pub fn minus_minutes(self, minutes: i64) -> Self { self.plus_minutes(-minutes) }

    pub fn plus_seconds(self, seconds: i64) -> Self {
        let delta = (seconds as i128) * 1_000_000_000_i128;
        Self::from_total_nanos_of_day(self.total_nanos_of_day() + delta)
    }
    pub fn minus_seconds(self, seconds: i64) -> Self { self.plus_seconds(-seconds) }

    pub fn plus_nanos(self, nanos: i64) -> Self {
        let delta = nanos as i128;
        Self::from_total_nanos_of_day(self.total_nanos_of_day() + delta)
    }
    pub fn minus_nanos(self, nanos: i64) -> Self { self.plus_nanos(-nanos) }

    // with_* setters (panic on invalid, like constructors)
    pub fn with_hour(self, hour: u8) -> Self {
        let t = time::Time::from_hms_nano(hour, self.0.minute(), self.0.second(), self.0.nanosecond())
            .expect("invalid hour");
        LocalTime(t)
    }
    pub fn with_minute(self, minute: u8) -> Self {
        let t = time::Time::from_hms_nano(self.0.hour(), minute, self.0.second(), self.0.nanosecond())
            .expect("invalid minute");
        LocalTime(t)
    }
    pub fn with_second(self, second: u8) -> Self {
        let t = time::Time::from_hms_nano(self.0.hour(), self.0.minute(), second, self.0.nanosecond())
            .expect("invalid second");
        LocalTime(t)
    }
    pub fn with_nano(self, nano: u32) -> Self {
        let t = time::Time::from_hms_nano(self.0.hour(), self.0.minute(), self.0.second(), nano)
            .expect("invalid nano");
        LocalTime(t)
    }

    pub fn at_date(self, date: crate::LocalDate) -> crate::LocalDateTime {
        crate::LocalDateTime::of_date_time(date, self)
    }

    pub fn is_before(&self, other: LocalTime) -> bool { self.0 < other.0 }
    pub fn is_after(&self, other: LocalTime) -> bool { self.0 > other.0 }
    pub fn is_on_or_before(&self, other: LocalTime) -> bool { self.0 <= other.0 }
    pub fn is_on_or_after(&self, other: LocalTime) -> bool { self.0 >= other.0 }
}
