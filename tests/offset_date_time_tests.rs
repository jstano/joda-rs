mod tests {
    use joda_rs::{LocalDate, LocalDateTime, LocalTime, OffsetDateTime, ZoneOffset};

    #[test]
    fn of_constructor_and_offset_behavior() {
        let ldt = LocalDateTime::of(2025, 9, 15, 10, 30, 0);
        let odt_utc = OffsetDateTime::of(ldt, ZoneOffset::of_hours(0));
        let odt_minus_2 = odt_utc.with_offset_same_instant(ZoneOffset::of_hours(-2));

        assert_eq!(odt_minus_2.hour(), 8);
        assert_eq!(odt_minus_2.minute(), 30);
        assert_eq!(odt_minus_2.second(), 0);

        assert_eq!(odt_utc.offset(), 0);
        assert_eq!(odt_minus_2.offset(), -7200);
    }

    #[test]
    fn now_utc_is_reasonable() {
        let now = OffsetDateTime::now_utc();
        // Just basic invariants: the offset should be UTC when converted
        let offset = now.offset();
        // now_utc may return +00:00
        assert_eq!(offset, 0);
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
        let base0 = base.with_nanosecond(0);
        assert_eq!(base0.plus_nanoseconds(1_234).nanosecond(), 1_234);
        assert_eq!(base0.plus_nanoseconds(1_000_000_000), OffsetDateTime::of(LocalDateTime::of(2020, 1, 1, 0, 0, 1), ZoneOffset::of_hours(0)));
        let prev = base0.minus_nanoseconds(1);
        assert_eq!(prev, OffsetDateTime::of(LocalDateTime::of(2019, 12, 31, 23, 59, 59), ZoneOffset::of_hours(0)).with_nanosecond(999_999_999));
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
        let odt2 = odt.with_nanosecond(123_000_000);
        assert_eq!(odt2.nanosecond(), 123_000_000);
    }

    #[test]
    fn conversions_to_local_types() {
        let odt = OffsetDateTime::of(LocalDateTime::of(2023, 5, 6, 1, 2, 3), ZoneOffset::of_hours(0)).with_nanosecond(987_654_321);
        let d: LocalDate = odt.to_local_date();
        let t: LocalTime = odt.to_local_time();
        let ldt: LocalDateTime = odt.to_local_date_time();

        assert_eq!(d, LocalDate::of(2023, 5, 6));
        assert_eq!(t, LocalTime::of(1, 2, 3).with_nanosecond(987_654_321));
        assert_eq!(ldt, LocalDateTime::of(2023, 5, 6, 1, 2, 3).with_nanosecond(987_654_321));

    }
}
