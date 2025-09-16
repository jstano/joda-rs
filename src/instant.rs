#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Instant(pub std::time::Instant);

impl From<std::time::Instant> for Instant { fn from(t: std::time::Instant) -> Self { Instant(t) } }
impl From<Instant> for std::time::Instant { fn from(w: Instant) -> Self { w.0 } }
impl Instant {
    pub fn inner(&self) -> &std::time::Instant { &self.0 }

    // java.time.Instant.now()
    pub fn now() -> Self { Instant(std::time::Instant::now()) }

    // --- Time-based arithmetic (checked) ---
    pub fn plus_seconds(self, seconds: i64) -> Self {
        if seconds >= 0 {
            let d = std::time::Duration::from_secs(seconds as u64);
            Instant(self.0.checked_add(d).expect("instant overflow on plus_seconds"))
        } else {
            self.minus_seconds(-seconds)
        }
    }
    pub fn minus_seconds(self, seconds: i64) -> Self {
        if seconds >= 0 {
            let d = std::time::Duration::from_secs(seconds as u64);
            Instant(self.0.checked_sub(d).expect("instant underflow on minus_seconds"))
        } else {
            self.plus_seconds(-seconds)
        }
    }

    pub fn plus_millis(self, millis: i64) -> Self {
        if millis >= 0 {
            let d = std::time::Duration::from_millis(millis as u64);
            Instant(self.0.checked_add(d).expect("instant overflow on plus_millis"))
        } else {
            self.minus_millis(-millis)
        }
    }
    pub fn minus_millis(self, millis: i64) -> Self {
        if millis >= 0 {
            let d = std::time::Duration::from_millis(millis as u64);
            Instant(self.0.checked_sub(d).expect("instant underflow on minus_millis"))
        } else {
            self.plus_millis(-millis)
        }
    }

    pub fn plus_nanos(self, nanos: i64) -> Self {
        if nanos >= 0 {
            let d = std::time::Duration::from_nanos(nanos as u64);
            Instant(self.0.checked_add(d).expect("instant overflow on plus_nanos"))
        } else {
            self.minus_nanos(-nanos)
        }
    }
    pub fn minus_nanos(self, nanos: i64) -> Self {
        if nanos >= 0 {
            let d = std::time::Duration::from_nanos(nanos as u64);
            Instant(self.0.checked_sub(d).expect("instant underflow on minus_nanos"))
        } else {
            self.plus_nanos(-nanos)
        }
    }

    // --- Comparisons ---
    pub fn is_before(&self, other: Instant) -> bool { self.0 < other.0 }
    pub fn is_after(&self, other: Instant) -> bool { self.0 > other.0 }
    pub fn is_on_or_before(&self, other: Instant) -> bool { self.0 <= other.0 }
    pub fn is_on_or_after(&self, other: Instant) -> bool { self.0 >= other.0 }

    // Map this monotonic Instant to a wall-clock OffsetDateTime by using the
    // delta from Instant::now() applied to OffsetDateTime::now_utc().
    fn to_wallclock_odt(self) -> time::OffsetDateTime {
        let now_mono = std::time::Instant::now();
        let now_wall = time::OffsetDateTime::now_utc();
        if self.0 >= now_mono {
            let d = self.0.duration_since(now_mono);
            let tdur = time::Duration::seconds(d.as_secs() as i64)
                + time::Duration::nanoseconds(d.subsec_nanos() as i64);
            now_wall + tdur
        } else {
            let d = now_mono.duration_since(self.0);
            let tdur = time::Duration::seconds(d.as_secs() as i64)
                + time::Duration::nanoseconds(d.subsec_nanos() as i64);
            now_wall - tdur
        }
    }

    // --- Queries / conversions ---
    pub fn epoch_second(&self) -> i64 { self.to_wallclock_odt().unix_timestamp() }

    pub fn at_offset(&self, offset: crate::ZoneOffset) -> crate::OffsetDateTime {
        let odt = self.to_wallclock_odt();
        let off: time::UtcOffset = offset.into();
        crate::OffsetDateTime(odt.to_offset(off))
    }

    pub fn at_zone(&self, _zone: crate::ZoneId) -> crate::ZonedDateTime {
        // Placeholder: only UTC semantics; ignore zone id for now
        let odt = self.to_wallclock_odt();
        crate::ZonedDateTime(odt.to_offset(time::UtcOffset::UTC))
    }
}
