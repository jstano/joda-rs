use joda_rs::{YearMonth, Month, LocalDate};

#[test]
fn construction_and_accessors() {
    let ym = YearMonth::of(2024, 2);
    assert_eq!(ym.year(), 2024);
    assert_eq!(ym.month(), Month::February);
    assert_eq!(ym.month_value(), 2);
}

#[test]
fn now_exists() {
    let _ = YearMonth::now();
}

#[test]
fn lengths_and_leap() {
    let feb_leap = YearMonth::of(2020, 2);
    assert!(feb_leap.is_leap_year());
    assert_eq!(feb_leap.length_of_month(), 29);

    let feb_non = YearMonth::of(2021, 2);
    assert!(!feb_non.is_leap_year());
    assert_eq!(feb_non.length_of_month(), 28);
}

#[test]
fn at_day_builds_local_date_and_validates() {
    let ym = YearMonth::of(2024, 2);
    let d = ym.at_day(29);
    assert_eq!(d, LocalDate::of(2024, 2, 29));
    // first_day_of_month and last_day_of_month
    let first = ym.first_day_of_month();
    let last = ym.last_day_of_month();
    assert_eq!(first, LocalDate::of(2024, 2, 1));
    assert_eq!(last, LocalDate::of(2024, 2, 29));

    let ym_non = YearMonth::of(2021, 2);
    assert_eq!(ym_non.first_day_of_month(), LocalDate::of(2021, 2, 1));
    assert_eq!(ym_non.last_day_of_month(), LocalDate::of(2021, 2, 28));
}

#[test]
#[should_panic]
fn at_day_panics_on_invalid() {
    let ym = YearMonth::of(2021, 2);
    let _ = ym.at_day(29);
}

#[test]
fn arithmetic_and_adjusters() {
    let ym = YearMonth::of(2020, 1);
    assert_eq!(ym.plus_months(1), YearMonth::of(2020, 2));
    assert_eq!(ym.minus_months(1), YearMonth::of(2019, 12));
    assert_eq!(YearMonth::of(2020, 12).plus_months(1), YearMonth::of(2021, 1));

    assert_eq!(ym.plus_years(2), YearMonth::of(2022, 1));
    assert_eq!(ym.minus_years(1), YearMonth::of(2019, 1));

    assert_eq!(ym.with_month(12), YearMonth::of(2020, 12));
    assert_eq!(ym.with_year(1999), YearMonth::of(1999, 1));
}

#[test]
fn comparisons_and_display() {
    let a = YearMonth::of(2020, 5);
    let b = YearMonth::of(2020, 6);
    assert!(a.is_before(b));
    assert!(b.is_after(a));
    assert!(a.is_on_or_before(b));
    assert!(b.is_on_or_after(a));
    assert_eq!(format!("{}", a), "2020-05");
}
