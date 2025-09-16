use joda_rs::Month;

#[test]
fn of_value_and_display() {
    assert_eq!(Month::of(1), Month::January);
    assert_eq!(Month::of(12), Month::December);
    assert_eq!(Month::March.value(), 3);
    assert_eq!(format!("{}", Month::September), "SEPTEMBER");
}

#[test]
fn length_min_max_and_leap() {
    assert_eq!(Month::February.length(false), 28);
    assert_eq!(Month::February.length(true), 29);
    assert_eq!(Month::April.min_length(), 30);
    assert_eq!(Month::January.max_length(), 31);
}

#[test]
fn plus_minus_wrap() {
    assert_eq!(Month::January.minus(1), Month::December);
    assert_eq!(Month::November.plus(2), Month::January);
    assert_eq!(Month::March.plus(24), Month::March);
    assert_eq!(Month::August.minus(14), Month::June);
}

#[test]
fn conversion_to_from_time_month() {
    let tm: time::Month = Month::May.into();
    let m2: Month = tm.into();
    assert_eq!(m2, Month::May);
}
