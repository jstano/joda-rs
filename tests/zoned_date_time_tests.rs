mod tests {
    use joda_rs::{LocalDateTime, OffsetDateTime, ZoneId, ZoneOffset, ZonedDateTime};

    #[test]
    fn of_utc_equals_offset_utc_assumption() {
        let ldt = LocalDateTime::of(2025, 9, 15, 10, 30, 0);

        // ZonedDateTime.of assumes UTC regardless of ZoneId (placeholder behavior)
        let zdt_utc = ZonedDateTime::of(ldt, ZoneId::UTC);
        let odt_utc = OffsetDateTime::of(ldt, ZoneOffset::of_hours(0));

        // Compare instants by unix_timestamp
        let zu = zdt_utc.epoch_seconds();
        let ou = odt_utc.epoch_seconds();
        assert_eq!(zu, ou);

        assert_eq!(
            (zdt_utc.year(), zdt_utc.month() as u8, zdt_utc.day_of_month(), zdt_utc.hour(), zdt_utc.minute(), zdt_utc.second()),
            (odt_utc.year(), odt_utc.month() as u8, odt_utc.day_of_month(), odt_utc.hour(), odt_utc.minute(), odt_utc.second())
        );
    }
}
