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

#[test]
fn plus_seconds_negative_delegates_to_minus_seconds() {
    let base = Instant::now();
    let a = base.plus_seconds(-3);
    let b = base.minus_seconds(3);
    assert_eq!(a, b);
}

#[test]
fn minus_seconds_negative_delegates_to_plus_seconds() {
    let base = Instant::now();
    let a = base.minus_seconds(-7);
    let b = base.plus_seconds(7);
    assert_eq!(a, b);
}

#[test]
fn plus_millis_negative_delegates_to_minus_millis() {
    let base = Instant::now();
    let a = base.plus_millis(-250);
    let b = base.minus_millis(250);
    assert_eq!(a, b);
}


#[test]
fn minus_millis_negative_delegates_to_plus_millis() {
    let base = Instant::now();
    let a = base.minus_millis(-123);
    let b = base.plus_millis(123);
    assert_eq!(a, b);
}


#[test]
fn plus_nanos_negative_delegates_to_minus_nanos() {
    let base = Instant::now();
    let a = base.plus_nanos(-987_654_321);
    let b = base.minus_nanos(987_654_321);
    assert_eq!(a, b);
}

#[test]
fn minus_nanos_negative_delegates_to_plus_nanos() {
    let base = Instant::now();
    let a = base.minus_nanos(-4_242);
    let b = base.plus_nanos(4_242);
    assert_eq!(a, b);
}


#[test]
fn from_std_instant_and_inner_exposes_inner_std() {
    let std_i = std::time::Instant::now();
    let ours: Instant = std_i.into();
    // Round-trip back to std::time::Instant
    let back: std::time::Instant = ours.into();
    assert_eq!(back, std_i);
    // inner() should expose the same underlying instant
    assert_eq!(*ours.inner(), std_i);
}
