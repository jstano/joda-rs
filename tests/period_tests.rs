mod tests {
    use joda_rs::Period;

    #[test]
    fn constructors_and_queries() {
        assert_eq!(Period::of(1, 2, 3), Period::new(1, 2, 3));
        assert_eq!(Period::of_years(2), Period::of(2, 0, 0));
        assert_eq!(Period::of_months(5), Period::of(0, 5, 0));
        assert_eq!(Period::of_weeks(2), Period::of(0, 0, 14));
        assert_eq!(Period::of_days(9), Period::of(0, 0, 9));
        assert!(Period::of(0, 0, 0).is_zero());
    }

    #[test]
    fn getters_and_total_months() {
        let p = Period::of(2, 3, 4);
        assert_eq!(p.years(), 2);
        assert_eq!(p.months(), 3);
        assert_eq!(p.days(), 4);
        assert_eq!(p.total_months(), 27);

        // Negative combinations
        let n1 = Period::of(-1, 6, 0); // -12 + 6 = -6
        assert_eq!(n1.total_months(), -6);
        let n2 = Period::of(-2, -5, 0); // -24 - 5 = -29
        assert_eq!(n2.total_months(), -29);
    }

    #[test]
    fn sign_checks_negative_and_positive() {
        // Negative if any component is negative
        assert!(Period::of(-1, 0, 0).is_negative());
        assert!(Period::of(0, -1, 0).is_negative());
        assert!(Period::of(0, 0, -1).is_negative());

        // Positive if any component is positive
        assert!(Period::of(1, 0, 0).is_positive());
        assert!(Period::of(0, 1, 0).is_positive());
        assert!(Period::of(0, 0, 1).is_positive());

        // Mixed signs: both flags can be true depending on components
        let mixed = Period::of(1, -1, 0);
        assert!(mixed.is_positive());
        assert!(mixed.is_negative());

        // Zero is neither positive nor negative
        let zero = Period::of(0, 0, 0);
        assert!(!zero.is_positive());
        assert!(!zero.is_negative());
    }

    #[test]
    fn arithmetic_plus_minus_and_negated() {
        let p = Period::of(1, 2, 3);
        let q = Period::of(2, 3, 4);
        assert_eq!(p.plus(q), Period::of(3, 5, 7));
        assert_eq!(p.minus(q), Period::of(-1, -1, -1));
        assert_eq!(p.negated(), Period::of(-1, -2, -3));
    }

    #[test]
    fn component_adders_and_subtractors() {
        let p = Period::of(1, 2, 3)
            .plus_years(4)
            .plus_months(5)
            .plus_days(6)
            .minus_years(1)
            .minus_months(2)
            .minus_days(3);
        // Start (1,2,3) + (4,5,6) - (1,2,3) = (4,5,6)
        assert_eq!(p, Period::of(4, 5, 6));
    }

    #[test]
    fn withers_update_single_component() {
        let base = Period::of(1, 2, 3);

        let y = base.with_years(10);
        assert_eq!(y, Period::of(10, 2, 3));

        let m = base.with_months(-5);
        assert_eq!(m, Period::of(1, -5, 3));

        let d = base.with_days(0);
        assert_eq!(d, Period::of(1, 2, 0));

        // Ensure original unchanged (Copy semantics)
        assert_eq!(base, Period::of(1, 2, 3));
    }

    #[test]
    fn withers_are_const_usable() {
        const P1: Period = Period::of(1, 2, 3).with_years(7);
        const P2: Period = Period::of_years(7).with_months(4).with_days(5);

        // Use them at runtime to assert
        assert_eq!(P1, Period::of(7, 2, 3));
        assert_eq!(P2, Period::of(7, 4, 5));
    }
}
