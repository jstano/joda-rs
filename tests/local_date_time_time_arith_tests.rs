use joda_rs::LocalDateTime;

#[test]
fn nano_getter_returns_expected_value() {
    // Start with a known datetime and set a specific nano-of-second
    let base = LocalDateTime::of(2021, 3, 14, 1, 2, 3).with_nano(123_456_789);
    assert_eq!(base.nano(), 123_456_789);

    // Ensure with_nano doesn't affect other fields
    let same = LocalDateTime::of(2021, 3, 14, 1, 2, 3);
    assert_eq!(base.with_nano(0).day(), same.day());
    assert_eq!(base.with_nano(0).hour(), same.hour());
}

#[test]
fn minus_hours_matches_plus_hours_negative_and_crosses_day_boundary() {
    let base = LocalDateTime::of(2020, 1, 2, 1, 0, 0);
    // minus 2 hours == plus (-2) hours
    let a = base.minus_hours(2);
    let b = base.plus_hours(-2);
    assert_eq!(a, b);
    assert_eq!(a, LocalDateTime::of(2020, 1, 1, 23, 0, 0));
}

#[test]
fn plus_and_minus_minutes_behavior_including_negative_values() {
    let base = LocalDateTime::of(2020, 1, 1, 0, 0, 0);

    // plus minutes forward
    assert_eq!(base.plus_minutes(61), LocalDateTime::of(2020, 1, 1, 1, 1, 0));

    // plus negative minutes is equivalent to minus positive minutes
    let a = base.plus_minutes(-1);
    let b = base.minus_minutes(1);
    assert_eq!(a, b);
    assert_eq!(a, LocalDateTime::of(2019, 12, 31, 23, 59, 0));

    // minus negative minutes is equivalent to plus positive minutes
    let c = base.minus_minutes(-90);
    let d = base.plus_minutes(90);
    assert_eq!(c, d);
    assert_eq!(c, LocalDateTime::of(2020, 1, 1, 1, 30, 0));
}

#[test]
fn plus_and_minus_seconds_behavior_including_negative_values() {
    let base = LocalDateTime::of(2020, 1, 1, 0, 0, 0);

    // forward seconds
    assert_eq!(base.plus_seconds(75), LocalDateTime::of(2020, 1, 1, 0, 1, 15));

    // plus negative seconds is equivalent to minus positive seconds
    let a = base.plus_seconds(-1);
    let b = base.minus_seconds(1);
    assert_eq!(a, b);
    assert_eq!(a, LocalDateTime::of(2019, 12, 31, 23, 59, 59));

    // minus negative seconds is equivalent to plus positive seconds
    let c = base.minus_seconds(-3601);
    let d = base.plus_seconds(3601);
    assert_eq!(c, d);
    assert_eq!(c, LocalDateTime::of(2020, 1, 1, 1, 0, 1));
}

#[test]
fn plus_nanos_and_negative_equivalences() {
    let base = LocalDateTime::of(2020, 1, 1, 0, 0, 0).with_nano(0);

    // plus nanos within the same second
    let a = base.plus_nanos(1_234);
    assert_eq!(a.nano(), 1_234);

    // crossing second boundary with nanos
    let b = base.plus_nanos(1_000_000_000);
    assert_eq!(b, LocalDateTime::of(2020, 1, 1, 0, 0, 1));

    // plus negative nanos equals minus positive nanos
    let c = base.plus_nanos(-42);
    let d = base.minus_nanos(42);
    assert_eq!(c, d);
    assert_eq!(c, LocalDateTime::of(2019, 12, 31, 23, 59, 59).with_nano(1_000_000_000 - 42));
}
