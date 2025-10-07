mod tests {
    use joda_rs::{Clock, Instant, OffsetDateTime, ZoneId, ZoneOffset};

    fn now_ms_utc() -> i64 {
        let nanos = OffsetDateTime::now_utc().epoch_nanoseconds();
        // saturating division to i64 range
        let ms = nanos / 1_000_000; // i128
        if ms > i64::MAX as i128 {
            i64::MAX
        } else if ms < i64::MIN as i128 {
            i64::MIN
        } else {
            ms as i64
        }
    }

    #[test]
    fn system_sets_zone() {
        let zid = ZoneId::try_of("America/New_York").unwrap();
        let c = Clock::system(zid);
        Clock::fixed(Instant::now(), zid);
        assert_eq!(c.zone(), zid);
    }

    #[test]
    fn system_default_zone_is_utc() {
        let c = Clock::system_default_zone();
        assert_eq!(c.zone(), ZoneId::UTC);
    }

    #[test]
    fn system_utc_is_utc() {
        let c = Clock::system_utc();
        assert_eq!(c.zone(), ZoneId::UTC);
    }

    #[test]
    fn system_instant_is_reasonable() {
        let c = Clock::system_utc();
        // Convert the clock instant to a wallclock ODT and compare now
        let a = c
            .instant()
            .at_offset(ZoneOffset::of_hours(0))
            .epoch_seconds();
        let b = OffsetDateTime::now_utc().epoch_seconds();
        let diff = (b - a).abs();
        // Calls should be within a couple of seconds on CI
        assert!(diff < 3, "system clock instant too far from now: {}s", diff);
    }

    #[test]
    fn millis_is_monotonic_and_reasonable_for_system() {
        let c = Clock::system_utc();
        let m1 = c.milliseconds();
        // small sleep to ensure time moves forward (1ms)
        std::thread::sleep(std::time::Duration::from_millis(1));
        let m2 = c.milliseconds();
        assert!(m2 >= m1, "millis should be non-decreasing: {} -> {}", m1, m2);

        // Compare against a now() snapshot with a generous tolerance (2 seconds)
        let approx_now = now_ms_utc();
        let delta = (m2 - approx_now).abs();
        assert!(delta < 2_000, "millis should be close to now (|Δ|={}ms)", delta);
    }

    #[test]
    fn millis_for_fixed_clock_matches_fixed_instant() {
        // Capture a reference instant and zone
        let inst = Instant::now();
        let zid = ZoneId::UTC; // zone is ignored for millis, but part of API
        let c = Clock::fixed(inst, zid);

        // Compute expected ms using the same approach used by clock.millis()
        let expected_ms = {
            let nanos = inst
                .at_offset(ZoneOffset::of_hours(0))
                .epoch_nanoseconds();
            (nanos / 1_000_000) as i128
        };
        let expected_ms_i64 = if expected_ms > i64::MAX as i128 {
            i64::MAX
        } else if expected_ms < i64::MIN as i128 {
            i64::MIN
        } else {
            expected_ms as i64
        };

        let actual = c.milliseconds();
        assert_eq!(actual, expected_ms_i64);
    }

    #[test]
    fn with_zone_on_system_updates_zone_and_preserves_variant() {
        let z1 = ZoneId::try_of("Europe/London").unwrap();
        let z2 = ZoneId::try_of("Asia/Tokyo").unwrap();

        let sys = Clock::system(z1);
        assert_eq!(sys.zone(), z1);

        let sys2 = sys.with_zone(z2);

        // Variant should remain System
        assert!(matches!(sys2, Clock::System(_)));
        // Zone must be updated
        assert_eq!(sys2.zone(), z2);

        // Instants from system clocks should both reflect "now" and be close to each other
        let a = sys
            .instant()
            .at_offset(ZoneOffset::of_hours(0))
            .epoch_seconds();
        let b = sys2
            .instant()
            .at_offset(ZoneOffset::of_hours(0))
            .epoch_seconds();
        let now = OffsetDateTime::now_utc().epoch_seconds();

        let diff_sys = (a - b).abs();
        let diff_now_a = (now - a).abs();
        let diff_now_b = (now - b).abs();

        // These calls should be within a few seconds on CI environments
        assert!(diff_sys < 3, "system clocks before/after with_zone too far apart: {}s", diff_sys);
        assert!(diff_now_a < 3, "system clock instant too far from now: {}s", diff_now_a);
        assert!(diff_now_b < 3, "system clock instant too far from now: {}s", diff_now_b);
    }
}
