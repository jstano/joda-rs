use joda_rs::ZoneOffset;

#[test]
fn total_seconds_basic() {
    let z0 = ZoneOffset::of_hours(0);
    assert_eq!(z0.total_seconds(), 0);

    let z_pos = ZoneOffset::of_hours_minutes(5, 30);
    assert_eq!(z_pos.total_seconds(), 5 * 3600 + 30 * 60);

    let z_neg = ZoneOffset::of_hours(-7);
    assert_eq!(z_neg.total_seconds(), -7 * 3600);
}
