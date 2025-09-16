use joda_rs::{LocalDate, Month, MonthDay};

#[test]
fn constructors_and_queries() {
    let md = MonthDay::of(2, 29); // allow Feb 29 in MonthDay
    assert_eq!(md.month(), Month::February);
    assert_eq!(md.month_value(), 2);
    assert_eq!(md.day_of_month(), 29);

    let md2 = MonthDay::of_month_day(Month::April, 30);
    assert_eq!(md2.month(), Month::April);
    assert_eq!(md2.day_of_month(), 30);
}

#[test]
fn comparisons() {
    let a = MonthDay::of(1, 15);
    let b = MonthDay::of(2, 1);
    let c = MonthDay::of(2, 1);
    assert!(a.is_before(b));
    assert!(b.is_after(a));
    assert!(b.is_on_or_after(c));
    assert!(b.is_on_or_before(c));
}

#[test]
fn at_year_clamps_when_needed() {
    // Feb 29 -> Feb 28 on non-leap
    let md = MonthDay::of(2, 29);
    let d_non_leap = md.at_year(2023);
    assert_eq!(d_non_leap, LocalDate::of(2023, 2, 28));

    // Valid on leap
    let d_leap = md.at_year(2024);
    assert_eq!(d_leap, LocalDate::of(2024, 2, 29));
}
