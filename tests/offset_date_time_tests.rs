use joda_rs::{LocalDateTime, OffsetDateTime, ZoneOffset};

#[test]
fn of_constructor_and_offset_behavior() {
    // Build a LocalDateTime and an offset of +02:00
    let ldt = LocalDateTime::of(2025, 9, 15, 10, 30, 0);
    let off = ZoneOffset::of_hours(2);
    let odt = OffsetDateTime::of(ldt, off);

    // Check that converting to UTC changes the wall time by -2 hours
    let inner = *odt.inner();
    let utc = inner.to_offset(time::UtcOffset::UTC);
    assert_eq!(utc.hour(), 8);
    assert_eq!(utc.minute(), 30);
    assert_eq!(utc.second(), 0);

    // And that the original offset is indeed +02:00
    let (h, m, s) = inner.offset().as_hms();
    assert_eq!((h, m, s), (2, 0, 0));

    // Round-trip: if we construct with offset 0, the UTC wall time should match utc time above
    let odt_utc = OffsetDateTime::of(ldt, ZoneOffset::of_hours(0));
    let inner_rt = *odt_utc.inner();
    assert_eq!(inner_rt.hour(), 10); // ldt is naive; with +00 it stays 10:30 local at UTC offset
    let inner_rt_as_utc = inner_rt.to_offset(time::UtcOffset::UTC);
    assert_eq!(inner_rt_as_utc.hour(), 10);
}

#[test]
fn now_utc_is_reasonable() {
    let now = OffsetDateTime::now_utc();
    // Just basic invariants: the offset should be UTC when converted
    let (h, m, s) = now.inner().offset().as_hms();
    // now_utc may return +00:00
    assert_eq!((h, m, s), (0, 0, 0));

    // Construct another now, ensure it is not wildly different (sanity check) by comparing seconds difference
    let a = *now.inner();
    let b = *OffsetDateTime::now_utc().inner();
    let diff = (b.unix_timestamp() - a.unix_timestamp()).abs();
    assert!(diff < 5, "now calls should be within a few seconds on CI: {}s", diff);
}
