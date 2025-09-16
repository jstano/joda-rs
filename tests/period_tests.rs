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
