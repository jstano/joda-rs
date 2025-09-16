use joda_rs::{LocalDateTime, ZoneId, ZonedDateTime};

#[test]
fn to_local_date_time_round_trip() {
    let ldt = LocalDateTime::of(2025, 9, 15, 10, 30, 0);
    let zdt = ZonedDateTime::of(ldt, ZoneId::UTC);
    assert_eq!(zdt.to_local_date_time(), ldt);
}
