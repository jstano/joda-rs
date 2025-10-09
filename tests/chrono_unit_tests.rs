mod tests {
    use joda_rs::{ChronoUnit, Duration};

    #[test]
    fn is_time_based_flags() {
        // time-based
        assert!(ChronoUnit::Nanos.is_time_based());
        assert!(ChronoUnit::Millis.is_time_based());
        assert!(ChronoUnit::Seconds.is_time_based());
        assert!(ChronoUnit::Minutes.is_time_based());
        assert!(ChronoUnit::Hours.is_time_based());
        assert!(ChronoUnit::HalfDays.is_time_based());

        // not time-based
        assert!(!ChronoUnit::Days.is_time_based());
        assert!(!ChronoUnit::Weeks.is_time_based());
        assert!(!ChronoUnit::Months.is_time_based());
        assert!(!ChronoUnit::Years.is_time_based());
    }

    #[test]
    fn is_date_based_flags() {
        // date-based
        assert!(ChronoUnit::Days.is_date_based());
        assert!(ChronoUnit::Weeks.is_date_based());
        assert!(ChronoUnit::Months.is_date_based());
        assert!(ChronoUnit::Years.is_date_based());

        // not date-based
        assert!(!ChronoUnit::Nanos.is_date_based());
        assert!(!ChronoUnit::Millis.is_date_based());
        assert!(!ChronoUnit::Seconds.is_date_based());
        assert!(!ChronoUnit::Minutes.is_date_based());
        assert!(!ChronoUnit::Hours.is_date_based());
        assert!(!ChronoUnit::HalfDays.is_date_based());
    }

    #[test]
    fn duration_mappings() {
        assert_eq!(ChronoUnit::Nanos.duration(), Duration::of_nanoseconds(1));
        assert_eq!(ChronoUnit::Millis.duration(), Duration::of_milliseconds(1));
        assert_eq!(ChronoUnit::Seconds.duration(), Duration::of_seconds(1));
        assert_eq!(ChronoUnit::Minutes.duration(), Duration::of_minutes(1));
        assert_eq!(ChronoUnit::Hours.duration(), Duration::of_hours(1));
        assert_eq!(ChronoUnit::HalfDays.duration(), Duration::of_hours(12));
        assert_eq!(ChronoUnit::Days.duration(), Duration::of_days(1));
        assert_eq!(ChronoUnit::Weeks.duration(), Duration::of_days(7));
        // Approximations
        assert_eq!(ChronoUnit::Months.duration(), Duration::of_days(30));
        assert_eq!(ChronoUnit::Years.duration(), Duration::of_days(365));
    }

    #[test]
    fn display_strings() {
        assert_eq!(ChronoUnit::Nanos.to_string(), "NANOS");
        assert_eq!(ChronoUnit::Millis.to_string(), "MILLIS");
        assert_eq!(ChronoUnit::Seconds.to_string(), "SECONDS");
        assert_eq!(ChronoUnit::Minutes.to_string(), "MINUTES");
        assert_eq!(ChronoUnit::Hours.to_string(), "HOURS");
        assert_eq!(ChronoUnit::HalfDays.to_string(), "HALF_DAYS");
        assert_eq!(ChronoUnit::Days.to_string(), "DAYS");
        assert_eq!(ChronoUnit::Weeks.to_string(), "WEEKS");
        assert_eq!(ChronoUnit::Months.to_string(), "MONTHS");
        assert_eq!(ChronoUnit::Years.to_string(), "YEARS");
    }
}
