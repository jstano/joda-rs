mod tests {
    use joda_rs::DayOfWeek;

    #[test]
    fn of_and_value_and_display() {
        assert_eq!(DayOfWeek::of(1), DayOfWeek::Monday);
        assert_eq!(DayOfWeek::of(7), DayOfWeek::Sunday);
        assert_eq!(DayOfWeek::Friday.value(), 5);

        // Display uppercase
        assert_eq!(format!("{}", DayOfWeek::Wednesday), "WEDNESDAY");
    }

    #[test]
    fn of_maps_midweek_days() {
        assert_eq!(DayOfWeek::of(3), DayOfWeek::Wednesday);
        assert_eq!(DayOfWeek::of(4), DayOfWeek::Thursday);
        assert_eq!(DayOfWeek::of(5), DayOfWeek::Friday);
    }

    #[test]
    fn plus_minus_wrapping() {
        assert_eq!(DayOfWeek::Monday.plus(7), DayOfWeek::Monday);
        assert_eq!(DayOfWeek::Monday.plus(8), DayOfWeek::Tuesday);
        assert_eq!(DayOfWeek::Sunday.plus(1), DayOfWeek::Monday);

        assert_eq!(DayOfWeek::Monday.minus(1), DayOfWeek::Sunday);
        assert_eq!(DayOfWeek::Thursday.minus(5), DayOfWeek::Saturday);
    }

    #[test]
    #[should_panic(expected = "invalid day-of-week value (1-7)")]
    fn of_panics_below_range() {
        // 0 is below the valid range 1..=7
        let _ = DayOfWeek::of(0);
    }

    #[test]
    #[should_panic(expected = "invalid day-of-week value (1-7)")]
    fn of_panics_above_range() {
        // 8 is above the valid range 1..=7
        let _ = DayOfWeek::of(8);
    }

    #[test]
    fn display_uppercase_all_days() {
        let cases = [
            (DayOfWeek::Monday, "MONDAY"),
            (DayOfWeek::Tuesday, "TUESDAY"),
            (DayOfWeek::Wednesday, "WEDNESDAY"),
            (DayOfWeek::Thursday, "THURSDAY"),
            (DayOfWeek::Friday, "FRIDAY"),
            (DayOfWeek::Saturday, "SATURDAY"),
            (DayOfWeek::Sunday, "SUNDAY"),
        ];
        for (d, expected) in cases {
            assert_eq!(format!("{}", d), expected);
        }
    }
}
