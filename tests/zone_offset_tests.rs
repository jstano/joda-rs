use joda_rs::ZoneOffset;

#[test]
fn of_hours_and_hours_minutes() {
    let z0 = ZoneOffset::of_hours(0);
    let (h0, m0, s0) = z0.inner().as_hms();
    assert_eq!((h0, m0, s0), (0, 0, 0));

    let z5 = ZoneOffset::of_hours(5);
    let (h5, m5, s5) = z5.inner().as_hms();
    assert_eq!((h5, m5, s5), (5, 0, 0));

    let z5_30 = ZoneOffset::of_hours_minutes(5, 30);
    let (h53, m53, s53) = z5_30.inner().as_hms();
    assert_eq!((h53, m53, s53), (5, 30, 0));

    let zn = ZoneOffset::of_hours(-7);
    let (hn, mn, sn) = zn.inner().as_hms();
    assert_eq!((hn, mn, sn), (-7, 0, 0));
}
