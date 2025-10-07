mod tests {
    use joda_rs::{LocalDate, LocalDateTime, LocalTime, ZoneId, ZonedDateTime};

    #[test]
    fn time_based_arithmetic_hours_minutes_seconds_nanos_for_zoned() {
        // Base: 2020-01-01T00:00:00Z
        let base = ZonedDateTime::of(LocalDateTime::of(2020, 1, 1, 0, 0, 0), ZoneId::UTC);

        // hours
        assert_eq!(
            base.plus_hours(2),
            ZonedDateTime::of(LocalDateTime::of(2020, 1, 1, 2, 0, 0), ZoneId::UTC)
        );
        assert_eq!(
            base.minus_hours(1),
            ZonedDateTime::of(LocalDateTime::of(2019, 12, 31, 23, 0, 0), ZoneId::UTC)
        );

        // minutes
        assert_eq!(
            base.plus_minutes(61),
            ZonedDateTime::of(LocalDateTime::of(2020, 1, 1, 1, 1, 0), ZoneId::UTC)
        );
        assert_eq!(
            base.minus_minutes(1),
            ZonedDateTime::of(LocalDateTime::of(2019, 12, 31, 23, 59, 0), ZoneId::UTC)
        );

        // seconds
        assert_eq!(
            base.plus_seconds(75),
            ZonedDateTime::of(LocalDateTime::of(2020, 1, 1, 0, 1, 15), ZoneId::UTC)
        );
        assert_eq!(
            base.minus_seconds(1),
            ZonedDateTime::of(LocalDateTime::of(2019, 12, 31, 23, 59, 59), ZoneId::UTC)
        );

        // nanos
        let base0 = base.with_nanosecond(0);
        assert_eq!(base0.plus_nanoseconds(1_234).nanosecond(), 1_234);
        assert_eq!(
            base0.plus_nanoseconds(1_000_000_000),
            ZonedDateTime::of(LocalDateTime::of(2020, 1, 1, 0, 0, 1), ZoneId::UTC)
        );
        let prev = base0.minus_nanoseconds(1);
        assert_eq!(
            prev,
            ZonedDateTime::of(LocalDateTime::of(2019, 12, 31, 23, 59, 59), ZoneId::UTC).with_nanosecond(999_999_999)
        );
    }

    #[test]
    fn date_based_arithmetic_days_weeks_months_years_for_zoned() {
        // days and weeks
        let base = ZonedDateTime::of(LocalDateTime::of(2020, 1, 1, 12, 0, 0), ZoneId::UTC);
        assert_eq!(
            base.plus_days(1),
            ZonedDateTime::of(LocalDateTime::of(2020, 1, 2, 12, 0, 0), ZoneId::UTC)
        );
        assert_eq!(
            base.minus_days(2),
            ZonedDateTime::of(LocalDateTime::of(2019, 12, 30, 12, 0, 0), ZoneId::UTC)
        );
        assert_eq!(base.plus_weeks(1), base.plus_days(7));
        assert_eq!(base.minus_weeks(2), base.minus_days(14));

        // months with clamping (leverages LocalDateTime logic)
        assert_eq!(
            ZonedDateTime::of(LocalDateTime::of(2020, 1, 31, 12, 0, 0), ZoneId::UTC).plus_months(1),
            ZonedDateTime::of(LocalDateTime::of(2020, 2, 29, 12, 0, 0), ZoneId::UTC)
        );
        assert_eq!(
            ZonedDateTime::of(LocalDateTime::of(2019, 1, 31, 12, 0, 0), ZoneId::UTC).plus_months(1),
            ZonedDateTime::of(LocalDateTime::of(2019, 2, 28, 12, 0, 0), ZoneId::UTC)
        );
        assert_eq!(
            ZonedDateTime::of(LocalDateTime::of(2020, 3, 31, 12, 0, 0), ZoneId::UTC).minus_months(1),
            ZonedDateTime::of(LocalDateTime::of(2020, 2, 29, 12, 0, 0), ZoneId::UTC)
        );

        // years with Feb 29 clamping
        assert_eq!(
            ZonedDateTime::of(LocalDateTime::of(2020, 2, 29, 8, 30, 0), ZoneId::UTC).plus_years(1),
            ZonedDateTime::of(LocalDateTime::of(2021, 2, 28, 8, 30, 0), ZoneId::UTC)
        );
        assert_eq!(
            ZonedDateTime::of(LocalDateTime::of(2020, 2, 29, 8, 30, 0), ZoneId::UTC).minus_years(1),
            ZonedDateTime::of(LocalDateTime::of(2019, 2, 28, 8, 30, 0), ZoneId::UTC)
        );
    }

    #[test]
    fn comparison_helpers_for_zoned_date_time() {
        let a = ZonedDateTime::of(LocalDateTime::of(2020, 1, 1, 0, 0, 0), ZoneId::UTC);
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
    fn with_setters_for_both_date_and_time_parts_for_zoned() {
        let zdt = ZonedDateTime::of(LocalDateTime::of(2021, 6, 15, 10, 20, 30), ZoneId::UTC);

        assert_eq!(
            zdt.with_day_of_month(1),
            ZonedDateTime::of(LocalDateTime::of(2021, 6, 1, 10, 20, 30), ZoneId::UTC)
        );
        assert_eq!(
            zdt.with_day_of_year(200),
            ZonedDateTime::of(LocalDateTime::of(2021, 7, 19, 10, 20, 30), ZoneId::UTC)
        );
        assert_eq!(
            zdt.with_month(2),
            ZonedDateTime::of(LocalDateTime::of(2021, 2, 15, 10, 20, 30), ZoneId::UTC)
        );
        assert_eq!(
            zdt.with_year(2020),
            ZonedDateTime::of(LocalDateTime::of(2020, 6, 15, 10, 20, 30), ZoneId::UTC)
        );

        assert_eq!(
            zdt.with_hour(5),
            ZonedDateTime::of(LocalDateTime::of(2021, 6, 15, 5, 20, 30), ZoneId::UTC)
        );
        assert_eq!(
            zdt.with_minute(59),
            ZonedDateTime::of(LocalDateTime::of(2021, 6, 15, 10, 59, 30), ZoneId::UTC)
        );
        assert_eq!(
            zdt.with_second(0),
            ZonedDateTime::of(LocalDateTime::of(2021, 6, 15, 10, 20, 0), ZoneId::UTC)
        );
        let zdt2 = zdt.with_nanosecond(123_000_000);
        assert_eq!(zdt2.nanosecond(), 123_000_000);
    }

    #[test]
    fn conversions_to_local_types_for_zoned() {
        let zdt = ZonedDateTime::of(LocalDateTime::of(2023, 5, 6, 1, 2, 3), ZoneId::UTC).with_nanosecond(987_654_321);
        let d: LocalDate = zdt.to_local_date();
        let t: LocalTime = zdt.to_local_time();
        let ldt: LocalDateTime = zdt.to_local_date_time();

        assert_eq!(d, LocalDate::of(2023, 5, 6));
        assert_eq!(t, LocalTime::of(1, 2, 3).with_nanosecond(987_654_321));
        assert_eq!(ldt, LocalDateTime::of(2023, 5, 6, 1, 2, 3).with_nanosecond(987_654_321));
    }
}
