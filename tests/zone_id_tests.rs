use joda_rs::ZoneId;

#[test]
fn basic_equality_and_construction() {
    let a = ZoneId::UTC;
    let b = ZoneId::of("UTC");
    let c = ZoneId::of("America/New_York");
    assert_eq!(a, b);
    assert_ne!(a, c);

    // Clone/Copy behavior
    let a2 = a;
    let a3 = a2;
    assert_eq!(a, a2);
    assert_eq!(a2, a3);
}
