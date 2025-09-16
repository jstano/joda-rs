#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Duration(pub time::Duration);

impl From<time::Duration> for Duration { fn from(t: time::Duration) -> Self { Duration(t) } }
impl From<Duration> for time::Duration { fn from(w: Duration) -> Self { w.0 } }
impl Duration {
    pub fn inner(&self) -> &time::Duration { &self.0 }

    // java.time.Duration.ofSeconds
    pub fn of_seconds(secs: i64) -> Self { Duration(time::Duration::seconds(secs)) }
    // java.time.Duration.ofMillis
    pub fn of_millis(ms: i64) -> Self { Duration(time::Duration::milliseconds(ms)) }
    // java.time.Duration.ofNanos
    pub fn of_nanos(ns: i64) -> Self { Duration(time::Duration::nanoseconds(ns)) }

    // --- Conversions to whole units (truncate toward zero) ---
    pub fn to_days(&self) -> i64 { self.0.whole_days() }
    pub fn to_hours(&self) -> i64 { self.0.whole_hours() }
    pub fn to_minutes(&self) -> i64 { self.0.whole_minutes() }
    pub fn to_seconds(&self) -> i64 { self.0.whole_seconds() }
    pub fn to_millis(&self) -> i64 {
        let ms = self.0.whole_milliseconds();
        let ms = ms.clamp(i128::from(i64::MIN), i128::from(i64::MAX));
        ms as i64
    }
    pub fn to_nanos(&self) -> i128 { self.0.whole_nanoseconds() }

    // --- Sign checks ---
    pub fn is_negative(&self) -> bool { self.0.is_negative() }
    pub fn is_zero(&self) -> bool { self.0.is_zero() }
    pub fn is_positive(&self) -> bool { self.0.is_positive() }

    // java.time.Duration.between(Instant, Instant)
    pub fn between(start: crate::Instant, end: crate::Instant) -> Self {
        let s: std::time::Instant = start.into();
        let e: std::time::Instant = end.into();
        let std_dur = e.duration_since(s);
        let secs_u = std_dur.as_secs();
        let nanos = std_dur.subsec_nanos() as i64;
        let secs = if secs_u > i64::MAX as u64 { i64::MAX } else { secs_u as i64 };
        let mut d = time::Duration::seconds(secs);
        d = d + time::Duration::nanoseconds(nanos);
        Duration(d)
    }

    pub fn plus(self, other: Duration) -> Self { Duration(self.0 + other.0) }
    pub fn minus(self, other: Duration) -> Self { Duration(self.0 - other.0) }

    // --- Arithmetic by unit amounts (saturating) ---
    pub fn plus_days(self, days: i64) -> Self { Duration(self.0.saturating_add(time::Duration::days(days))) }
    pub fn minus_days(self, days: i64) -> Self { Duration(self.0.saturating_sub(time::Duration::days(days))) }

    pub fn plus_hours(self, hours: i64) -> Self { Duration(self.0.saturating_add(time::Duration::hours(hours))) }
    pub fn minus_hours(self, hours: i64) -> Self { Duration(self.0.saturating_sub(time::Duration::hours(hours))) }

    pub fn plus_minutes(self, minutes: i64) -> Self { Duration(self.0.saturating_add(time::Duration::minutes(minutes))) }
    pub fn minus_minutes(self, minutes: i64) -> Self { Duration(self.0.saturating_sub(time::Duration::minutes(minutes))) }

    pub fn plus_seconds(self, seconds: i64) -> Self { Duration(self.0.saturating_add(time::Duration::seconds(seconds))) }
    pub fn minus_seconds(self, seconds: i64) -> Self { Duration(self.0.saturating_sub(time::Duration::seconds(seconds))) }

    pub fn plus_millis(self, millis: i64) -> Self { Duration(self.0.saturating_add(time::Duration::milliseconds(millis))) }
    pub fn minus_millis(self, millis: i64) -> Self { Duration(self.0.saturating_sub(time::Duration::milliseconds(millis))) }

    pub fn plus_nanos(self, nanos: i64) -> Self { Duration(self.0.saturating_add(time::Duration::nanoseconds(nanos))) }
    pub fn minus_nanos(self, nanos: i64) -> Self { Duration(self.0.saturating_sub(time::Duration::nanoseconds(nanos))) }

    pub fn abs(self) -> Self { Duration(self.0.abs()) }
}
