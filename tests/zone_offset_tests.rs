mod tests {
    use joda_rs::ZoneOffset;

    #[test]
    fn utc_constant_should_have_zero_offset() {
        let utc_offset = ZoneOffset::UTC;
        assert_eq!(utc_offset.total_seconds(), 0);
    }

    #[test]
    fn total_seconds_basic() {
        let z0 = ZoneOffset::of_hours(0);
        assert_eq!(z0.total_seconds(), 0);

        let z_pos = ZoneOffset::of_hours_minutes(5, 30);
        assert_eq!(z_pos.total_seconds(), 5 * 3600 + 30 * 60);

        let z_neg = ZoneOffset::of_hours(-7);
        assert_eq!(z_neg.total_seconds(), -7 * 3600);
    }
}
