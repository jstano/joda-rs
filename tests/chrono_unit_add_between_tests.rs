mod tests {
    use joda_rs::{ChronoUnit, Instant, LocalDate, LocalDateTime};

    #[test]
    fn add_to_instant_time_units() {
        let base = Instant::of_epoch_second(1_000);
        assert_eq!(ChronoUnit::Seconds.add_to(base, 5), base.plus_seconds(5));
        assert_eq!(ChronoUnit::Minutes.add_to(base, 2), base.plus_seconds(120));
        assert_eq!(ChronoUnit::Hours.add_to(base, 1), base.plus_seconds(3_600));
        assert_eq!(ChronoUnit::Nanos.add_to(base, 1_000_000), base.plus_milliseconds(1));
    }

    #[test]
    fn between_instants() {
        let a = Instant::of_epoch_second(0);
        let b = Instant::of_epoch_second(3661); // 1h 1m 1s
        assert_eq!(ChronoUnit::Seconds.between(a, b), 3_661);
        assert_eq!(ChronoUnit::Minutes.between(a, b), 61);
        assert_eq!(ChronoUnit::Hours.between(a, b), 1);
        assert_eq!(ChronoUnit::Days.between(a, b), 0);
    }

    #[test]
    fn add_to_local_date_date_units() {
        let d = LocalDate::of(2020, 1, 31);
        assert_eq!(ChronoUnit::Days.add_to(d, 1), LocalDate::of(2020, 2, 1));
        assert_eq!(ChronoUnit::Weeks.add_to(d, 1), LocalDate::of(2020, 2, 7));
        assert_eq!(ChronoUnit::Months.add_to(d, 1), LocalDate::of(2020, 2, 29));
        assert_eq!(ChronoUnit::Years.add_to(d, 1), LocalDate::of(2021, 1, 31));
    }

    #[test]
    fn add_to_local_date_time_mixed_units() {
        let ldt = LocalDateTime::of(2020, 2, 28, 23, 0, 0);
        let ldt_plus = ChronoUnit::Hours.add_to(ldt, 2); // should roll to Feb 29 01:00
        assert_eq!(ldt_plus, LocalDateTime::of(2020, 2, 29, 1, 0, 0));
        let ldt_month = ChronoUnit::Months.add_to(ldt, 1);
        assert_eq!(ldt_month, LocalDateTime::of(2020, 3, 28, 23, 0, 0));
    }

    #[test]
    fn between_local_date_times_via_epoch() {
        let a = LocalDateTime::of(2020, 1, 1, 0, 0, 0);
        let b = LocalDateTime::of(2020, 1, 2, 0, 0, 0);
        assert_eq!(ChronoUnit::Days.between(a, b), 1);
        assert_eq!(ChronoUnit::Hours.between(a, b), 24);
    }
}
