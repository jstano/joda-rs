use joda_rs::{LocalDate, LocalDateTime, LocalTime, OffsetDateTime, ZoneOffset};

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

#[test]
fn from_time_offsetdatetime_and_into_roundtrip() {
    // Build a time::OffsetDateTime directly and convert to our type and back
    let date = time::Date::from_calendar_date(2020, time::Month::January, 2).unwrap();
    let time_ = time::Time::from_hms(3, 4, 5).unwrap();
    let pdt = time::PrimitiveDateTime::new(date, time_);
    let off = time::UtcOffset::from_hms(5, 30, 0).unwrap();
    let todt = pdt.assume_offset(off);

    let ours: OffsetDateTime = todt.into();
    assert_eq!(*ours.inner(), todt);

    let back: time::OffsetDateTime = ours.into();
    assert_eq!(back, todt);
}

#[test]
fn time_based_arithmetic_hours_minutes_seconds_nanos() {
    // Base: 2020-01-01T00:00:00Z
    let base = OffsetDateTime::of(LocalDateTime::of(2020, 1, 1, 0, 0, 0), ZoneOffset::of_hours(0));

    // hours
    assert_eq!(base.plus_hours(2), OffsetDateTime::of(LocalDateTime::of(2020, 1, 1, 2, 0, 0), ZoneOffset::of_hours(0)));
    assert_eq!(base.minus_hours(1), OffsetDateTime::of(LocalDateTime::of(2019, 12, 31, 23, 0, 0), ZoneOffset::of_hours(0)));

    // minutes
    assert_eq!(base.plus_minutes(61), OffsetDateTime::of(LocalDateTime::of(2020, 1, 1, 1, 1, 0), ZoneOffset::of_hours(0)));
    assert_eq!(base.minus_minutes(1), OffsetDateTime::of(LocalDateTime::of(2019, 12, 31, 23, 59, 0), ZoneOffset::of_hours(0)));

    // seconds
    assert_eq!(base.plus_seconds(75), OffsetDateTime::of(LocalDateTime::of(2020, 1, 1, 0, 1, 15), ZoneOffset::of_hours(0)));
    assert_eq!(base.minus_seconds(1), OffsetDateTime::of(LocalDateTime::of(2019, 12, 31, 23, 59, 59), ZoneOffset::of_hours(0)));

    // nanos
    let base0 = base.with_nano(0);
    assert_eq!(base0.plus_nanos(1_234).inner().time().nanosecond(), 1_234);
    assert_eq!(base0.plus_nanos(1_000_000_000), OffsetDateTime::of(LocalDateTime::of(2020, 1, 1, 0, 0, 1), ZoneOffset::of_hours(0)));
    let prev = base0.minus_nanos(1);
    assert_eq!(prev, OffsetDateTime::of(LocalDateTime::of(2019, 12, 31, 23, 59, 59), ZoneOffset::of_hours(0)).with_nano(999_999_999));
}

#[test]
fn date_based_arithmetic_days_weeks_months_years() {
    // days and weeks
    let base = OffsetDateTime::of(LocalDateTime::of(2020, 1, 1, 12, 0, 0), ZoneOffset::of_hours(0));
    assert_eq!(base.plus_days(1), OffsetDateTime::of(LocalDateTime::of(2020, 1, 2, 12, 0, 0), ZoneOffset::of_hours(0)));
    assert_eq!(base.minus_days(2), OffsetDateTime::of(LocalDateTime::of(2019, 12, 30, 12, 0, 0), ZoneOffset::of_hours(0)));
    assert_eq!(base.plus_weeks(1), base.plus_days(7));
    assert_eq!(base.minus_weeks(2), base.minus_days(14));

    // months with clamping
    assert_eq!(
        OffsetDateTime::of(LocalDateTime::of(2020, 1, 31, 12, 0, 0), ZoneOffset::of_hours(0)).plus_months(1),
        OffsetDateTime::of(LocalDateTime::of(2020, 2, 29, 12, 0, 0), ZoneOffset::of_hours(0))
    );
    assert_eq!(
        OffsetDateTime::of(LocalDateTime::of(2019, 1, 31, 12, 0, 0), ZoneOffset::of_hours(0)).plus_months(1),
        OffsetDateTime::of(LocalDateTime::of(2019, 2, 28, 12, 0, 0), ZoneOffset::of_hours(0))
    );
    assert_eq!(
        OffsetDateTime::of(LocalDateTime::of(2020, 3, 31, 12, 0, 0), ZoneOffset::of_hours(0)).minus_months(1),
        OffsetDateTime::of(LocalDateTime::of(2020, 2, 29, 12, 0, 0), ZoneOffset::of_hours(0))
    );

    // years with Feb 29 clamping
    assert_eq!(
        OffsetDateTime::of(LocalDateTime::of(2020, 2, 29, 8, 30, 0), ZoneOffset::of_hours(0)).plus_years(1),
        OffsetDateTime::of(LocalDateTime::of(2021, 2, 28, 8, 30, 0), ZoneOffset::of_hours(0))
    );
    assert_eq!(
        OffsetDateTime::of(LocalDateTime::of(2020, 2, 29, 8, 30, 0), ZoneOffset::of_hours(0)).minus_years(1),
        OffsetDateTime::of(LocalDateTime::of(2019, 2, 28, 8, 30, 0), ZoneOffset::of_hours(0))
    );
}

#[test]
fn comparison_helpers_for_offset_date_time() {
    let a = OffsetDateTime::of(LocalDateTime::of(2020, 1, 1, 0, 0, 0), ZoneOffset::of_hours(0));
    let b = a.plus_seconds(1);
    let c = b;

    assert!(a.is_before(b));
    assert!(!b.is_before(a));

    assert!(b.is_after(a));
    assert!(!a.is_after(b));

    assert!(a.is_on_or_before(b));
    assert!(b.is_on_or_before(c)); // equality case

    assert!(b.is_on_or_after(a));
    assert!(b.is_on_or_after(c)); // equality case
}

#[test]
fn with_setters_for_both_date_and_time_parts() {
    let odt = OffsetDateTime::of(LocalDateTime::of(2021, 6, 15, 10, 20, 30), ZoneOffset::of_hours(0));

    assert_eq!(odt.with_day_of_month(1), OffsetDateTime::of(LocalDateTime::of(2021, 6, 1, 10, 20, 30), ZoneOffset::of_hours(0)));
    assert_eq!(odt.with_day_of_year(200), OffsetDateTime::of(LocalDateTime::of(2021, 7, 19, 10, 20, 30), ZoneOffset::of_hours(0)));
    assert_eq!(odt.with_month(2), OffsetDateTime::of(LocalDateTime::of(2021, 2, 15, 10, 20, 30), ZoneOffset::of_hours(0)));
    assert_eq!(odt.with_year(2020), OffsetDateTime::of(LocalDateTime::of(2020, 6, 15, 10, 20, 30), ZoneOffset::of_hours(0)));

    assert_eq!(odt.with_hour(5), OffsetDateTime::of(LocalDateTime::of(2021, 6, 15, 5, 20, 30), ZoneOffset::of_hours(0)));
    assert_eq!(odt.with_minute(59), OffsetDateTime::of(LocalDateTime::of(2021, 6, 15, 10, 59, 30), ZoneOffset::of_hours(0)));
    assert_eq!(odt.with_second(0), OffsetDateTime::of(LocalDateTime::of(2021, 6, 15, 10, 20, 0), ZoneOffset::of_hours(0)));
    let odt2 = odt.with_nano(123_000_000);
    assert_eq!(odt2.inner().time().nanosecond(), 123_000_000);
}

#[test]
fn conversions_to_local_types() {
    let odt = OffsetDateTime::of(LocalDateTime::of(2023, 5, 6, 1, 2, 3), ZoneOffset::of_hours(0)).with_nano(987_654_321);
    let d: LocalDate = odt.to_local_date();
    let t: LocalTime = odt.to_local_time();
    let ldt: LocalDateTime = odt.to_local_date_time();

    assert_eq!(d, LocalDate::of(2023, 5, 6));
    assert_eq!(t, LocalTime::of(1, 2, 3).with_nano(987_654_321));
    assert_eq!(ldt, LocalDateTime::of(2023, 5, 6, 1, 2, 3).with_nano(987_654_321));

}
