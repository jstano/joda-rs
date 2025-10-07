mod tests {
    use joda_rs::{Instant, ZoneId, ZoneOffset};

    #[test]
    fn arithmetic_and_comparisons() {
        let a = Instant::now();
        let b = a.plus_seconds(1);
        assert!(b.is_after(a));
        assert!(a.is_before(b));
        assert!(a.is_on_or_before(b));
        assert!(b.is_on_or_after(a));

        let c = b.minus_milliseconds(500);
        // c should be between a and b
        assert!(c.is_after(a));
        assert!(c.is_before(b));

        let d = c.plus_nanoseconds(1_000_000);
        assert!(d.is_after(c));
    }

    #[test]
    fn epoch_and_at_offset_zone_sanity() {
        let now = Instant::now();
        let epoch = now.epoch_seconds();
        // Expect a plausible epoch (>= 2000-01-01)
        assert!(epoch >= 946684800);

        // at_offset: using UTC offset (0) should equal converting to UTC
        let odt = now.at_offset(ZoneOffset::of_hours(0));
        let zdt = now.at_zone(ZoneId::UTC);
        assert_eq!(odt.epoch_seconds(), zdt.epoch_seconds());
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
        let a = base.plus_milliseconds(-250);
        let b = base.minus_milliseconds(250);
        assert_eq!(a, b);
    }


    #[test]
    fn minus_millis_negative_delegates_to_plus_millis() {
        let base = Instant::now();
        let a = base.minus_milliseconds(-123);
        let b = base.plus_milliseconds(123);
        assert_eq!(a, b);
    }


    #[test]
    fn plus_nanos_negative_delegates_to_minus_nanos() {
        let base = Instant::now();
        let a = base.plus_nanoseconds(-987_654_321);
        let b = base.minus_nanoseconds(987_654_321);
        assert_eq!(a, b);
    }

    #[test]
    fn minus_nanos_negative_delegates_to_plus_nanos() {
        let base = Instant::now();
        let a = base.minus_nanoseconds(-4_242);
        let b = base.plus_nanoseconds(4_242);
        assert_eq!(a, b);
    }
}
