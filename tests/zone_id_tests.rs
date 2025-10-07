mod tests {
    use joda_rs::ZoneId;
    use time_tz::TimeZone;

    #[test]
    fn basic_equality_and_construction() {
        let a = ZoneId::UTC;
        let b = ZoneId::try_of("UTC");
        let c = ZoneId::try_of("America/New_York");
        let d = ZoneId::try_of("Invalid/Time_Zone");
        assert_eq!(b.unwrap().id(), "UTC");
        assert_eq!(c.unwrap().id(), "America/New_York");
        assert_eq!(d.unwrap_err(), "Unknown time zone");

        let a2 = a;
        let a3 = a2;
        assert_eq!(a, a2);
        assert_eq!(a2, a3);
    }

    #[test]
    fn system_default_matches_time_tz_and_is_known_id() {
        // Obtain the expected system zone from time_tz. If unavailable on this system/CI, skip the test.
        let tz = match time_tz::system::get_timezone() {
            Ok(tz) => tz,
            Err(e) => {
                eprintln!("Skipping system_default test: could not determine system tz: {}", e);
                return;
            }
        };

        let expected_name = tz.name();

        // Call the API under test
        let sys = ZoneId::system_default();

        // Its id should equal the name reported by time_tz
        assert_eq!(sys.id(), expected_name);

        // The id should resolve via try_of and equal the same ZoneId
        let via_try = ZoneId::try_of(expected_name).expect("expected system tz to be a known id");
        assert_eq!(sys, via_try);

        // And it should be present in the time_tz database
        assert!(time_tz::timezones::get_by_name(expected_name).is_some());
    }
}
