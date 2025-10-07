mod tests {
    use joda_rs::{LocalDate, LocalDateTime, LocalTime};

    #[test]
    fn accessors_and_of() {
        let t = LocalTime::of(1, 2, 3);
        assert_eq!(t.hour(), 1);
        assert_eq!(t.minute(), 2);
        assert_eq!(t.second(), 3);
    }

    #[test]
    fn arithmetic_wrap_and_nanos() {
        let t = LocalTime::of(23, 0, 0);
        assert_eq!(t.plus_hours(2), LocalTime::of(1, 0, 0));
        let t2 = LocalTime::of(0, 0, 1);
        assert_eq!(t2.minus_seconds(2), LocalTime::of(23, 59, 59));
        // nanos
        let t3 = LocalTime::of(0, 0, 0);
        let t3n = t3.minus_nanoseconds(1);
        assert_eq!(t3n.hour(), 23);
        assert_eq!(t3n.minute(), 59);
        assert_eq!(t3n.second(), 59);
    }

    #[test]
    fn with_setters() {
        let t = LocalTime::of(10, 20, 30);
        assert_eq!(t.with_hour(5).hour(), 5);
        assert_eq!(t.with_minute(59).minute(), 59);
        assert_eq!(t.with_second(0).second(), 0);
        let tn = t.with_nanosecond(999_999_999);
        assert_eq!(tn.hour(), 10);
    }

    #[test]
    fn at_date_and_compare() {
        let d = LocalDate::of(2024, 1, 1);
        let t = LocalTime::of(12, 0, 0);
        let ldt = t.at_date(d);
        assert_eq!(ldt, LocalDateTime::of(2024, 1, 1, 12, 0, 0));

        let a = LocalTime::of(1, 0, 0);
        let b = LocalTime::of(2, 0, 0);
        assert!(a.is_before(b));
        assert!(b.is_after(a));
        assert!(a.is_on_or_before(b));
        assert!(b.is_on_or_after(a));
        assert!(a.is_on_or_before(a));
        assert!(a.is_on_or_after(a));
    }
}
