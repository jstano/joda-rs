mod tests {
    use joda_rs::Month;

    #[test]
    fn of_value_and_display() {
        assert_eq!(Month::of(1), Month::January);
        assert_eq!(Month::of(12), Month::December);
        assert_eq!(Month::March.value(), 3);
        assert_eq!(format!("{}", Month::September), "SEPTEMBER");
    }

    #[test]
    fn of_maps_july_through_november() {
        assert_eq!(Month::of(7), Month::July);
        assert_eq!(Month::of(8), Month::August);
        assert_eq!(Month::of(9), Month::September);
        assert_eq!(Month::of(10), Month::October);
        assert_eq!(Month::of(11), Month::November);
    }

    #[test]
    #[should_panic(expected = "invalid month value (1-12)")]
    fn of_panics_below_range() {
        let _ = Month::of(0);
    }

    #[test]
    #[should_panic(expected = "invalid month value (1-12)")]
    fn of_panics_above_range() {
        let _ = Month::of(13);
    }

    #[test]
    fn length_min_max_and_leap() {
        assert_eq!(Month::February.length(false), 28);
        assert_eq!(Month::February.length(true), 29);
        assert_eq!(Month::April.min_length(), 30);
        assert_eq!(Month::January.max_length(), 31);
    }

    #[test]
    fn plus_minus() {
        assert_eq!(Month::November.plus(2), Month::January);
        assert_eq!(Month::March.plus(24), Month::March);
        assert_eq!(Month::May.plus(8), Month::January);
        assert_eq!(Month::May.plus(-7), Month::October);

        assert_eq!(Month::January.minus(1), Month::December);
        assert_eq!(Month::August.minus(14), Month::June);
        assert_eq!(Month::May.minus(8), Month::September);
        assert_eq!(Month::May.minus(-7), Month::December);
    }

    #[test]
    fn length_for_may_through_november() {
        let cases = [
            (Month::May, 31),
            (Month::June, 30),
            (Month::July, 31),
            (Month::August, 31),
            (Month::September, 30),
            (Month::October, 31),
            (Month::November, 30),
        ];
        for (m, days) in cases {
            assert_eq!(m.length(false), days, "min length for {:?}", m);
            assert_eq!(m.length(true), days, "max length for {:?}", m);
            assert_eq!(m.min_length(), days);
            assert_eq!(m.max_length(), days);
        }
    }

    #[test]
    fn display_uppercase_all_months() {
        let cases = [
            (Month::January, "JANUARY"),
            (Month::February, "FEBRUARY"),
            (Month::March, "MARCH"),
            (Month::April, "APRIL"),
            (Month::May, "MAY"),
            (Month::June, "JUNE"),
            (Month::July, "JULY"),
            (Month::August, "AUGUST"),
            (Month::September, "SEPTEMBER"),
            (Month::October, "OCTOBER"),
            (Month::November, "NOVEMBER"),
            (Month::December, "DECEMBER"),
        ];
        for (m, expected) in cases {
            assert_eq!(format!("{}", m), expected);
        }
    }
}
