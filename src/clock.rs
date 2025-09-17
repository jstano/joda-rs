#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct FixedClock {
    instant: crate::Instant,
    zone: crate::ZoneId,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SystemClock {
    zone: crate::ZoneId,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Clock {
    Fixed(FixedClock),
    System(SystemClock),
}

impl Clock {
    // Constructors
    pub fn fixed(instant: crate::Instant, zone: crate::ZoneId) -> Self {
        Clock::Fixed(FixedClock { instant, zone })
    }

    pub fn system(zone: crate::ZoneId) -> Self { Clock::System(SystemClock { zone }) }

    pub fn system_default_zone() -> Self { Clock::System(SystemClock { zone: crate::ZoneId::UTC }) }

    pub fn system_utc() -> Self { Clock::System(SystemClock { zone: crate::ZoneId::UTC }) }

    // Queries
    pub fn instant(&self) -> crate::Instant {
        match self {
            Clock::Fixed(fc) => fc.instant,
            Clock::System(_) => crate::Instant::now(),
        }
    }

    pub fn zone(&self) -> crate::ZoneId {
        match self {
            Clock::Fixed(fc) => fc.zone,
            Clock::System(sc) => sc.zone,
        }
    }

    pub fn millis(&self) -> i64 {
        // Use UTC epoch milliseconds for now; zone is kept for API completeness.
        let odt = self
            .instant()
            .at_offset(crate::ZoneOffset::of_hours(0))
            .inner()
            .unix_timestamp_nanos();
        nanos_to_ms_saturated(odt)
    }

    pub fn with_zone(&self, zone: crate::ZoneId) -> Self {
        match self {
            Clock::Fixed(fc) => Clock::Fixed(FixedClock { instant: fc.instant, zone }),
            Clock::System(_) => Clock::System(SystemClock { zone }),
        }
    }
}

fn nanos_to_ms_saturated(nanos: i128) -> i64 {
    let ms = nanos / 1_000_000; // i128
    if ms > i64::MAX as i128 {
        i64::MAX
    } else if ms < i64::MIN as i128 {
        i64::MIN
    } else {
        ms as i64
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn nanos_to_ms_saturated_clamps_overflow() {
        let nanos = (i128::from(i64::MAX) * 1_000_000) + 1_000_000; // ms would be i64::MAX + 1
        assert_eq!(nanos_to_ms_saturated(nanos), i64::MAX);
    }

    #[test]
    fn nanos_to_ms_saturated_clamps_underflow() {
        let nanos = (i128::from(i64::MIN) * 1_000_000) - 1_000_000; // ms would be i64::MIN - 1
        assert_eq!(nanos_to_ms_saturated(nanos), i64::MIN);
    }

    #[test]
    fn nanos_to_ms_saturated_handles_boundaries() {
        let nanos_max = i128::from(i64::MAX) * 1_000_000;
        let nanos_min = i128::from(i64::MIN) * 1_000_000;
        assert_eq!(nanos_to_ms_saturated(nanos_max), i64::MAX);
        assert_eq!(nanos_to_ms_saturated(nanos_min), i64::MIN);
    }

    #[test]
    fn nanos_to_ms_saturated_normal_case() {
        let nanos = 12_345_i128 * 1_000_000; // 12_345 ms
        assert_eq!(nanos_to_ms_saturated(nanos), 12_345);
    }
}
