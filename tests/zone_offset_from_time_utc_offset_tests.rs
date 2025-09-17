use joda_rs::ZoneOffset;

#[test]
fn from_time_utc_offset_converts_correctly() {
    // UTC zero
    let t0 = time::UtcOffset::UTC;
    let z0: ZoneOffset = t0.into();
    assert_eq!(*z0.inner(), t0);

    // Positive offset +05:30
    let t_pos = time::UtcOffset::from_hms(5, 30, 0).unwrap();
    let z_pos: ZoneOffset = t_pos.into();
    assert_eq!(*z_pos.inner(), t_pos);
    // Round-trip back to time::UtcOffset
    let back_pos: time::UtcOffset = z_pos.into();
    assert_eq!(back_pos, t_pos);

    // Negative offset -07:00
    let t_neg = time::UtcOffset::from_hms(-7, 0, 0).unwrap();
    let z_neg: ZoneOffset = t_neg.into();
    assert_eq!(*z_neg.inner(), t_neg);
    let back_neg: time::UtcOffset = z_neg.into();
    assert_eq!(back_neg, t_neg);
}
