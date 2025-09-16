use joda_rs::{LocalDateTime, OffsetDateTime, ZoneId, ZoneOffset, ZonedDateTime};

#[test]
fn of_utc_equals_offset_utc_assumption() {
    let ldt = LocalDateTime::of(2025, 9, 15, 10, 30, 0);

    // ZonedDateTime.of assumes UTC regardless of ZoneId (placeholder behavior)
    let zdt_utc = ZonedDateTime::of(ldt, ZoneId::UTC);
    let odt_utc = OffsetDateTime::of(ldt, ZoneOffset::of_hours(0));

    // Compare instants by unix_timestamp
    let zu = zdt_utc.inner().unix_timestamp();
    let ou = odt_utc.inner().unix_timestamp();
    assert_eq!(zu, ou);

    // Wall-clock at UTC must match
    let zdt_utc_inner = zdt_utc.inner().to_offset(time::UtcOffset::UTC);
    let odt_utc_inner = odt_utc.inner().to_offset(time::UtcOffset::UTC);
    assert_eq!(
        (zdt_utc_inner.year(), zdt_utc_inner.month() as u8, zdt_utc_inner.day(), zdt_utc_inner.hour(), zdt_utc_inner.minute(), zdt_utc_inner.second()),
        (odt_utc_inner.year(), odt_utc_inner.month() as u8, odt_utc_inner.day(), odt_utc_inner.hour(), odt_utc_inner.minute(), odt_utc_inner.second())
    );
}

#[test]
fn of_non_utc_zone_still_assumes_utc_for_now() {
    let ldt = LocalDateTime::of(2025, 1, 1, 0, 0, 0);
    let zdt_ny = ZonedDateTime::of(ldt, ZoneId::of("America/New_York"));
    let zdt_utc = ZonedDateTime::of(ldt, ZoneId::UTC);

    // With current placeholder behavior, both should represent the same UTC-based instant
    assert_eq!(zdt_ny.inner().unix_timestamp(), zdt_utc.inner().unix_timestamp());

    // And both match the OffsetDateTime constructed with +00:00
    let odt_utc = OffsetDateTime::of(ldt, ZoneOffset::of_hours(0));
    assert_eq!(zdt_ny.inner().unix_timestamp(), odt_utc.inner().unix_timestamp());
}

#[test]
fn now_utc_exists_and_is_utc() {
    let now = ZonedDateTime::now_utc();
    let (h, m, s) = now.inner().offset().as_hms();
    assert_eq!((h, m, s), (0, 0, 0));
}
