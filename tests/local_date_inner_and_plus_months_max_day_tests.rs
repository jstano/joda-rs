use joda_rs::LocalDate;

#[test]
fn inner_exposes_time_date_and_into_roundtrip() {
    let ld = LocalDate::of(2025, 9, 16);
    let expected = time::Date::from_calendar_date(2025, time::Month::September, 16).unwrap();

    // inner() should reference the same underlying time::Date
    assert_eq!(*ld.inner(), expected);

    // Also verify Into<time::Date> for LocalDate yields the same value
    let back: time::Date = ld.into();
    assert_eq!(back, expected);
}

#[test]
fn plus_months_clamps_day_according_to_target_month_max_day() {
    // Use a non-leap year to clearly test February -> 28 and 30/31-day months
    let jan31_2021 = LocalDate::of(2021, 1, 31);

    // February (non-leap): 28
    assert_eq!(jan31_2021.plus_months(1), LocalDate::of(2021, 2, 28));
    // March: 31
    assert_eq!(jan31_2021.plus_months(2), LocalDate::of(2021, 3, 31));
    // April: 30
    assert_eq!(jan31_2021.plus_months(3), LocalDate::of(2021, 4, 30));
    // May: 31
    assert_eq!(jan31_2021.plus_months(4), LocalDate::of(2021, 5, 31));
    // June: 30
    assert_eq!(jan31_2021.plus_months(5), LocalDate::of(2021, 6, 30));
    // July: 31
    assert_eq!(jan31_2021.plus_months(6), LocalDate::of(2021, 7, 31));
    // August: 31
    assert_eq!(jan31_2021.plus_months(7), LocalDate::of(2021, 8, 31));
    // September: 30
    assert_eq!(jan31_2021.plus_months(8), LocalDate::of(2021, 9, 30));
    // October: 31
    assert_eq!(jan31_2021.plus_months(9), LocalDate::of(2021, 10, 31));
    // November: 30
    assert_eq!(jan31_2021.plus_months(10), LocalDate::of(2021, 11, 30));
    // December: 31
    assert_eq!(jan31_2021.plus_months(11), LocalDate::of(2021, 12, 31));

    // Also verify leap-year February path (max_day = 29)
    let jan31_2024 = LocalDate::of(2024, 1, 31);
    assert_eq!(jan31_2024.plus_months(1), LocalDate::of(2024, 2, 29));
}
