use joda_rs::{Instant, OffsetDateTime, ZoneId, ZoneOffset};

#[test]
fn arithmetic_and_comparisons() {
    let a = Instant::now();
    let b = a.plus_seconds(1);
    assert!(b.is_after(a));
    assert!(a.is_before(b));
    assert!(a.is_on_or_before(b));
    assert!(b.is_on_or_after(a));

    let c = b.minus_millis(500);
    // c should be between a and b
    assert!(c.is_after(a));
    assert!(c.is_before(b));

    let d = c.plus_nanos(1_000_000);
    assert!(d.is_after(c));
}

#[test]
fn epoch_and_at_offset_zone_sanity() {
    let i = Instant::now();
    let epoch = i.epoch_second();
    // Expect a plausible epoch (>= 2000-01-01)
    assert!(epoch >= 946684800);

    // at_offset: using UTC offset (0) should equal converting to UTC
    let odt: OffsetDateTime = i.at_offset(ZoneOffset::of_hours(0));
    let zdt = i.at_zone(ZoneId::UTC);
    assert_eq!(odt.inner().unix_timestamp(), zdt.inner().unix_timestamp());
}
