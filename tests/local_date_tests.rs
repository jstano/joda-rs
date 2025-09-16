use joda_rs::{DayOfWeek, LocalDate, LocalDateTime, LocalTime, Month};

#[test]
fn construction_and_parse() {
    let d = LocalDate::of(2025, 9, 15);
    assert_eq!(d, LocalDate::parse("2025-09-15"));
}

#[test]
fn at_time_and_start_of_day() {
    let d = LocalDate::of(2023, 5, 6);
    let lt = LocalTime::of(1, 2, 3);
    let ldt = d.at_time(lt);
    assert_eq!(ldt, LocalDateTime::of(2023, 5, 6, 1, 2, 3));

    let midnight = d.at_start_of_day();
    assert_eq!(midnight, LocalDateTime::of(2023, 5, 6, 0, 0, 0));
}

#[test]
fn queries_and_lengths() {
    let d = LocalDate::of(2020, 2, 29);
    assert_eq!(d.day_of_week(), DayOfWeek::Saturday);
    assert_eq!(d.day_of_month(), 29);
    assert_eq!(d.day_of_year(), 60);
    assert_eq!(d.length_of_month(), 29);
    assert_eq!(d.year().get_value(), 2020);

    assert_eq!(d.month(), Month::February);
    assert_eq!(d.month_value(), 2);
    assert!(d.is_leap_year());
    assert_eq!(d.length_of_year(), 366);

    let d2 = LocalDate::of(2021, 3, 14);
    assert_eq!(d2.month(), Month::March);
    assert_eq!(d2.month_value(), 3);
    assert!(!d2.is_leap_year());
    assert_eq!(d2.length_of_year(), 365);
}

#[test]
fn arithmetic_plus_minus_days_weeks_months_years() {
    let base = LocalDate::of(2020, 1, 31);
    // days
    assert_eq!(base.plus_days(1), LocalDate::of(2020, 2, 1));
    assert_eq!(base.minus_days(1), LocalDate::of(2020, 1, 30));
    // weeks
    assert_eq!(base.plus_weeks(1), LocalDate::of(2020, 2, 7));
    assert_eq!(base.minus_weeks(2), LocalDate::of(2020, 1, 17));
    // months (clamp end-of-month)
    assert_eq!(base.plus_months(1), LocalDate::of(2020, 2, 29)); // leap year Feb
    assert_eq!(LocalDate::of(2019, 1, 31).plus_months(1), LocalDate::of(2019, 2, 28));
    assert_eq!(LocalDate::of(2020, 3, 31).minus_months(1), LocalDate::of(2020, 2, 29));
    // years (Feb 29 clamped on non-leap)
    assert_eq!(LocalDate::of(2020, 2, 29).plus_years(1), LocalDate::of(2021, 2, 28));
    assert_eq!(LocalDate::of(2020, 2, 29).minus_years(1), LocalDate::of(2019, 2, 28));
}

#[test]
fn adjusters_with_methods() {
    let w = LocalDate::of(2021, 6, 15);
    assert_eq!(w.with_day_of_month(1), LocalDate::of(2021, 6, 1));
    assert_eq!(w.with_day_of_year(200), LocalDate::of(2021, 7, 19));
    assert_eq!(w.with_month(2), LocalDate::of(2021, 2, 15));
    assert_eq!(w.with_year(2020), LocalDate::of(2020, 6, 15));
}

#[test]
fn comparisons() {
    let a = LocalDate::of(2020, 1, 1);
    let b = LocalDate::of(2020, 1, 2);
    let a2 = LocalDate::of(2020, 1, 1);

    assert!(a.is_before(b));
    assert!(b.is_after(a));

    assert!(a.is_on_or_before(b));
    assert!(b.is_on_or_after(a));

    assert!(a.is_on_or_before(a2));
    assert!(a.is_on_or_after(a2));
}
