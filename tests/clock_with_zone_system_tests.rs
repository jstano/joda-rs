use joda_rs::{Clock, OffsetDateTime, ZoneId, ZoneOffset};

#[test]
fn with_zone_on_system_updates_zone_and_preserves_variant() {
    let z1 = ZoneId::of("Europe/London");
    let z2 = ZoneId::of("Asia/Tokyo");

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
        .inner()
        .unix_timestamp();
    let b = sys2
        .instant()
        .at_offset(ZoneOffset::of_hours(0))
        .inner()
        .unix_timestamp();
    let now = OffsetDateTime::now_utc().inner().unix_timestamp();

    let diff_sys = (a - b).abs();
    let diff_now_a = (now - a).abs();
    let diff_now_b = (now - b).abs();

    // These calls should be within a few seconds on CI environments
    assert!(diff_sys < 3, "system clocks before/after with_zone too far apart: {}s", diff_sys);
    assert!(diff_now_a < 3, "system clock instant too far from now: {}s", diff_now_a);
    assert!(diff_now_b < 3, "system clock instant too far from now: {}s", diff_now_b);
}
