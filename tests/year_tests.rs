use joda_rs::{LocalDate, Month, Year, YearMonth};

#[test]
fn basic_queries_and_display() {
    let y = Year::of(2024);
    assert!(y.is_leap());
    assert_eq!(y.length(), 366);
    assert_eq!(y.plus(1).value(), 2025);
    assert_eq!(y.minus(25).value(), 1999);
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

#[test]
fn now_utc_matches_current_year() {
    let y = Year::now().value();
    let expected = time::OffsetDateTime::now_utc().date().year();
    assert_eq!(y, expected);
}

#[test]
fn comparison_helpers() {
    let a = Year::of(2023);
    let b = Year::of(2025);
    let c = Year::of(2025);

    assert!(a.is_before(b));
    assert!(!b.is_before(a));

    assert!(b.is_after(a));
    assert!(!a.is_after(b));

    assert!(a.is_on_or_before(b));
    assert!(b.is_on_or_before(c)); // equality case

    assert!(b.is_on_or_after(a));
    assert!(b.is_on_or_after(c)); // equality case
}

#[test]
fn at_month_returns_expected_year_month() {
    let y = Year::of(2024);
    let ym = y.at_month(Month::February);
    let expected = YearMonth::of_year_month(2024, Month::February);
    assert_eq!(ym, expected);
    assert!(ym.is_leap_year()); // 2024 is leap

    let y2 = Year::of(2023);
    let ym2 = y2.at_month(Month::February);
    assert!(!ym2.is_leap_year());
}
