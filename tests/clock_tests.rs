use joda_rs::{Clock, Instant, OffsetDateTime, ZoneId, ZoneOffset};

fn now_ms_utc() -> i64 {
    let nanos = OffsetDateTime::now_utc().inner().unix_timestamp_nanos();
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
    let zid = ZoneId::of("TEST/Zone");
    let c = Clock::system(zid);
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
    // Convert the clock instant to a wallclock ODT and compare to now
    let a = c
        .instant()
        .at_offset(ZoneOffset::of_hours(0))
        .inner()
        .unix_timestamp();
    let b = OffsetDateTime::now_utc().inner().unix_timestamp();
    let diff = (b - a).abs();
    // Calls should be within a couple of seconds on CI
    assert!(diff < 3, "system clock instant too far from now: {}s", diff);
}

#[test]
fn millis_is_monotonic_and_reasonable_for_system() {
    let c = Clock::system_utc();
    let m1 = c.millis();
    // small sleep to ensure time moves forward (1ms)
    std::thread::sleep(std::time::Duration::from_millis(1));
    let m2 = c.millis();
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
            .inner()
            .unix_timestamp_nanos();
        (nanos / 1_000_000) as i128
    };
    let expected_ms_i64 = if expected_ms > i64::MAX as i128 {
        i64::MAX
    } else if expected_ms < i64::MIN as i128 {
        i64::MIN
    } else {
        expected_ms as i64
    };

    let actual = c.millis();
    assert_eq!(actual, expected_ms_i64);
}
