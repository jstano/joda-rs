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
}
