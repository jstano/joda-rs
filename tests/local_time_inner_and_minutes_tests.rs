use joda_rs::LocalTime;

#[test]
fn inner_exposes_time_time_and_into_roundtrip() {
    let lt = LocalTime::of(12, 34, 56);
    let expected = time::Time::from_hms(12, 34, 56).unwrap();

    // inner() should reference the same underlying time::Time
    assert_eq!(*lt.inner(), expected);

    // Also verify Into<time::Time> for LocalTime yields the same value
    let back: time::Time = lt.into();
    assert_eq!(back, expected);
}

#[test]
fn minus_hours_delegates_to_plus_hours_negated() {
    let base = LocalTime::of(10, 0, 0);
    // For a positive input: minus_hours(h) == plus_hours(-h)
    assert_eq!(base.minus_hours(5), base.plus_hours(-5));

    // For a negative input, the implementation should still match the identity
    // minus_hours(-h) == plus_hours(h)
    assert_eq!(base.minus_hours(-7), base.plus_hours(7));
}

#[test]
fn plus_minutes_arithmetic_and_wrapping() {
    // Simple add without wrapping
    let t = LocalTime::of(1, 0, 0).plus_minutes(61);
    assert_eq!(t, LocalTime::of(2, 1, 0));

    // Negative add (subtract) with wrapping across midnight
    let t2 = LocalTime::of(0, 0, 0).plus_minutes(-1);
    assert_eq!(t2, LocalTime::of(23, 59, 0));

    // Large positive add with multiple hour wraps
    let t3 = LocalTime::of(23, 30, 0).plus_minutes(120); // +2h -> 01:30
    assert_eq!(t3, LocalTime::of(1, 30, 0));
}

#[test]
fn minus_minutes_delegates_to_plus_minutes_negated() {
    let base = LocalTime::of(3, 15, 0);
    // Positive input
    assert_eq!(base.minus_minutes(45), base.plus_minutes(-45));
    // Negative input
    assert_eq!(base.minus_minutes(-75), base.plus_minutes(75));
}
