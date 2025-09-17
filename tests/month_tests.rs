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

#[test]
fn length_for_may_through_november() {
    let cases = [
        (Month::May, 31u8),
        (Month::June, 30u8),
        (Month::July, 31u8),
        (Month::August, 31u8),
        (Month::September, 30u8),
        (Month::October, 31u8),
        (Month::November, 30u8),
    ];
    for (m, days) in cases {
        assert_eq!(m.length(false), days, "min length for {:?}", m);
        assert_eq!(m.length(true), days, "max length for {:?}", m);
        assert_eq!(m.min_length(), days);
        assert_eq!(m.max_length(), days);
    }
}


#[test]
fn from_time_month_maps_all_months() {
    use joda_rs::Month;
    let cases = [
        (time::Month::January, Month::January),
        (time::Month::February, Month::February),
        (time::Month::March, Month::March),
        (time::Month::April, Month::April),
        (time::Month::May, Month::May),
        (time::Month::June, Month::June),
        (time::Month::July, Month::July),
        (time::Month::August, Month::August),
        (time::Month::September, Month::September),
        (time::Month::October, Month::October),
        (time::Month::November, Month::November),
        (time::Month::December, Month::December),
    ];
    for (t, expected) in cases {
        let got: Month = t.into();
        assert_eq!(got, expected, "mapping for {:?}", t);
    }
}

#[test]
fn into_time_month_maps_all_months() {
    use joda_rs::Month;
    let cases = [
        (Month::January, time::Month::January),
        (Month::February, time::Month::February),
        (Month::March, time::Month::March),
        (Month::April, time::Month::April),
        (Month::May, time::Month::May),
        (Month::June, time::Month::June),
        (Month::July, time::Month::July),
        (Month::August, time::Month::August),
        (Month::September, time::Month::September),
        (Month::October, time::Month::October),
        (Month::November, time::Month::November),
        (Month::December, time::Month::December),
    ];
    for (m, expected) in cases {
        let got: time::Month = m.into();
        assert_eq!(got, expected, "mapping for {:?}", m);
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
