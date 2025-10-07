mod tests {
    use joda_rs::{Clock, Instant, LocalDate, LocalDateTime, LocalTime, OffsetDateTime, ZoneId, ZoneOffset, ZonedDateTime};

    #[test]
    fn now_with_clock_matches_instant_conversions_utc() {
        // Create a deterministic clock fixed at a known offset from "now"
        let base = Instant::now().minus_seconds(12_345); // some seconds in the past
        let clock = Clock::fixed(base, ZoneId::UTC);

        // Reference conversions derived directly from the Instant
        let ref_zdt: ZonedDateTime = clock.instant().at_zone(clock.zone());
        let ref_ldt: LocalDateTime = ref_zdt.to_local_date_time();
        let ref_ld: LocalDate = ref_zdt.to_local_date();
        let ref_lt: LocalTime = ref_zdt.to_local_time();
        let ref_odt: OffsetDateTime = clock.instant().at_offset(ZoneOffset::of_hours(0));

        // Verify now_with_clock for each type approximately matches reference.
        // Because Instant->wallclock mapping uses a live "now" anchor internally twice,
        // allow for sub-microsecond jitter by comparing components or epoch seconds.
        let ldt_now = LocalDateTime::now_with_clock(&clock);
        let ld_now = LocalDate::now_with_clock(&clock);
        let lt_now = LocalTime::now_with_clock(&clock);
        let zdt_now = ZonedDateTime::now_with_clock(&clock);
        let odt_now = OffsetDateTime::now_with_clock(&clock);

        // LocalDate: exact date match
        assert_eq!(ld_now, ref_ld);

        // LocalTime: match to the second (ignore nanos jitter)
        assert_eq!(lt_now.hour(), ref_lt.hour());
        assert_eq!(lt_now.minute(), ref_lt.minute());
        assert_eq!(lt_now.second(), ref_lt.second());

        // LocalDateTime: date equal and time equal to the second
        assert_eq!(ldt_now.year(), ref_ldt.year());
        assert_eq!(ldt_now.month_value(), ref_ldt.month_value());
        assert_eq!(ldt_now.day_of_month(), ref_ldt.day_of_month());
        assert_eq!(ldt_now.hour(), ref_ldt.hour());
        assert_eq!(ldt_now.minute(), ref_ldt.minute());
        assert_eq!(ldt_now.second(), ref_ldt.second());

        // ZonedDateTime and OffsetDateTime: epoch seconds must match
        assert_eq!(zdt_now.epoch_seconds(), ref_zdt.epoch_seconds());
        assert_eq!(odt_now.epoch_seconds(), ref_odt.epoch_seconds());
    }

    #[test]
    fn now_with_clock_respects_with_zone_on_clock_placeholder_semantics() {
        // Even though zone handling is placeholder (UTC-only), ensure the API wires through correctly.
        let base = Instant::now().plus_seconds(7_890);
        let clock_utc = Clock::fixed(base, ZoneId::UTC);
        let clock_ny = clock_utc.with_zone(ZoneId::try_of("America/New_York").unwrap());

        let zdt_utc = ZonedDateTime::now_with_clock(&clock_utc);
        let zdt_ny = ZonedDateTime::now_with_clock(&clock_ny);

        // With current placeholder behavior, both represent the same instant (UTC-based)
        assert_eq!(zdt_utc.epoch_seconds(), zdt_ny.epoch_seconds());

        // And they each match the underlying instant-conversion at the epoch-second level
        assert_eq!(zdt_utc.epoch_seconds(), base.at_zone(ZoneId::UTC).epoch_seconds());
        assert_eq!(zdt_ny.epoch_seconds(), base.at_zone(ZoneId::try_of("America/New_York").unwrap()).epoch_seconds());
    }
}
