use crate::{Instant, LocalDateTime, ZoneId, ZoneOffset};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct FixedClock {
    instant: Instant,
    zone: ZoneId,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SystemClock {
    zone: ZoneId,
}

/// Enum representing different types of clocks that can be used in a system.
///
/// This enum provides two variants:
/// - `Fixed(FixedClock)`:
///   The fixed clock variant, which uses a `FixedClock` value. This clock type is suitable for use cases where
///   deterministic or unchanging time values are required, such as in tests.
///
/// - `System(SystemClock)`:
///   The system clock variant, which uses a `SystemClock` value. This clock type provides access to real-world time.
///
/// ```rust
/// let fixed_clock = Clock::fixed(Instant::now(), ZoneId::UTC);
/// let system_clock = Clock::system_utc();
///
/// match fixed_clock {
///     Clock::Fixed(_) => println!("This is a fixed clock."),
///     Clock::System(_) => println!("This is a system clock."),
/// }
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Clock {
    Fixed(FixedClock),
    System(SystemClock),
}

impl Clock {
    /// Creates a fixed clock instance in a given `zone`.
    ///
    /// ```rust
    /// let instant = Instant::now();
    /// let zone = ZoneId::UTC;
    /// let fixed_clock = Clock::fixed(instant, zone);
    /// ```
    pub fn fixed(instant: Instant, zone: ZoneId) -> Self {
        Clock::Fixed(FixedClock { instant, zone })
    }

    /// Creates a fixed clock instance in the UTC time zone.
    ///
    /// ```rust
    /// let instant = Instant::now();
    /// let zone = ZoneId::UTC;
    /// let fixed_clock = Clock::fixed(instant, zone);
    /// ```
    pub fn fixed_utc(instant: Instant) -> Self {
        Clock::Fixed(FixedClock {
            instant,
            zone: ZoneId::UTC,
        })
    }

    /// Creates a fixed `Clock` at the given local date-time in the specified offset.
    ///
    /// ```rust
    /// let datetime = LocalDateTime::new(2023, 10, 6, 8, 30, 45);
    /// let fixed_clock = Clock::fixed_at_local(datetime, ZoneId::UTC);
    /// ```
    pub fn fixed_at_local(ldt: LocalDateTime, zone: ZoneId) -> Self {
        Clock::fixed(ldt.at_zone(zone).to_instant(), zone)
    }

    /// Creates a new `Clock` instance configured to use the system clock in the specified `zone`.
    ///
    /// ```rust
    /// let clock = Clock::system(zone, ZoneId::new("America/New_York"));
    /// ```
    pub fn system(zone: ZoneId) -> Self {
        Clock::System(SystemClock { zone })
    }

    /// Creates a new `Clock` instance configured to use the system clock in the system default time zone.
    ///
    /// ```rust
    /// let system_clock = Clock::system_default_zone();
    /// ```
    pub fn system_default_zone() -> Self {
        Clock::System(SystemClock { zone: ZoneId::UTC })
    }

    /// Creates a new `Clock` instance configured to use the system clock in the UTC time zone..
    ///
    /// ```rust
    /// let utc_clock = Clock::system_utc();
    /// ```
    pub fn system_utc() -> Self {
        Clock::System(SystemClock { zone: ZoneId::UTC })
    }

    /// Returns the current instant based on the clock type.
    ///
    /// ```rust
    /// let fixed_clock = Clock::fixed_utc(Instant::of_epoch_second(42));
    /// assert_eq!(fixed_clock.instant(), Instant::of_epoch_second(42));
    ///
    /// println!("Current instant: {}", Clock::system_utc().instant());
    /// ```
    pub fn instant(&self) -> Instant {
        match self {
            Clock::Fixed(fc) => fc.instant,
            Clock::System(_) => Instant::now(),
        }
    }

    /// Retrieves the `ZoneId` associated with the clock.
    ///
    /// This method returns the time zone identifier (`ZoneId`) for the `Clock` instance.
    /// The behavior varies depending on whether the `Clock` is `Fixed` or `System`:
    ///
    /// ```rust
    /// let fixed_clock = Clock::Fixed(FixedClock { zone: ZoneId::UTC });
    /// assert_eq!(fixed_clock.zone(), ZoneId::UTC);
    ///
    /// let system_clock = Clock::System(SystemClock { zone: ZoneId::Local });
    /// assert_eq!(system_clock.zone(), ZoneId::Local);
    /// ```
    pub fn zone(self) -> ZoneId {
        match self {
            Clock::Fixed(fc) => fc.zone,
            Clock::System(sc) => sc.zone,
        }
    }

    /// Returns the number of milliseconds since the Unix epoch (January 1, 1970 00:00:00 UTC).
    ///
    /// This method calculates the milliseconds by using the UTC time zone (offset of 0) for consistency.
    /// It converts the high-precision nanosecond timestamp retrieved from the internal `instant` object
    /// to milliseconds using the `nanos_to_ms_saturated` function, which ensures a safe and bounded
    /// conversion.
    ///
    /// - The result relies on the UTC time zone for calculations, even if the original time might
    ///   have been in a different zone. This ensures consistency in calculations.
    /// - The `nanos_to_ms_saturated` method is employed to handle edge cases of overflowing or
    ///   out-of-bounds nanosecond values.
    ///
    /// ```rust
    /// let timestamp = some_object.millis();
    /// println!("Milliseconds since epoch: {}", timestamp);
    /// ```
    pub fn milliseconds(self) -> i64 {
        let odt = self
            .instant()
            .at_offset(ZoneOffset::of_hours(0))
            .inner()
            .unix_timestamp_nanos();
        nanos_to_ms_saturated(odt)
    }

    /// Creates a new `Clock` with the specified `zone` preserving its type (fixed or system).
    ///
    /// ```rust
    /// let fixed_clock = Clock::Fixed(FixedClock { instant, zone: ZoneId::UTC });
    /// let updated_clock = fixed_clock.with_zone(ZoneId::new("America/New_York"));
    /// assert!(matches!(updated_clock, Clock::Fixed(_)));
    ///
    /// let system_clock = Clock::System(SystemClock { zone: ZoneId::
    pub fn with_zone(self, zone: ZoneId) -> Self {
        match self {
            Clock::Fixed(fc) => Clock::Fixed(FixedClock {
                instant: fc.instant,
                zone,
            }),
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
