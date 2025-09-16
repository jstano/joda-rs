use joda_rs::{LocalDate, Month, Year};

#[test]
fn basic_queries_and_display() {
    let y = Year::of(2024);
    assert!(y.is_leap());
    assert_eq!(y.length(), 366);
    assert_eq!(y.plus(1).get_value(), 2025);
    assert_eq!(y.minus(25).get_value(), 1999);
    assert_eq!(format!("{}", y), "2024");
}

#[test]
fn at_day_and_at_month_day() {
    let y = Year::of(2024);
    let jan1 = y.at_day(1);
    assert_eq!(jan1, LocalDate::of(2024, 1, 1));
    let feb29 = y.at_month_day(Month::February, 29);
    assert_eq!(feb29, LocalDate::of(2024, 2, 29));
}

#[test]
fn non_leap_year_values() {
    let y = Year::of(2023);
    assert!(!y.is_leap());
    assert_eq!(y.length(), 365);
}
