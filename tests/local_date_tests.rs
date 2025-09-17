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
    assert_eq!(d.year(), 2020);

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


#[test]
fn first_and_last_day_of_month_helpers() {
    // Regular month
    let d = LocalDate::of(2021, 3, 14);
    assert_eq!(d.first_day_of_month(), LocalDate::of(2021, 3, 1));
    assert_eq!(d.last_day_of_month(), LocalDate::of(2021, 3, 31));

    // Leap February
    let feb_leap = LocalDate::of(2020, 2, 10);
    assert_eq!(feb_leap.first_day_of_month(), LocalDate::of(2020, 2, 1));
    assert_eq!(feb_leap.last_day_of_month(), LocalDate::of(2020, 2, 29));

    // Non-leap February
    let feb_non = LocalDate::of(2021, 2, 10);
    assert_eq!(feb_non.first_day_of_month(), LocalDate::of(2021, 2, 1));
    assert_eq!(feb_non.last_day_of_month(), LocalDate::of(2021, 2, 28));
}


#[test]
fn last_day_of_month_year_helper() {
    // Leap year February
    assert_eq!(LocalDate::last_day_of_month_year(2020, 2), LocalDate::of(2020, 2, 29));
    // Non-leap February
    assert_eq!(LocalDate::last_day_of_month_year(2021, 2), LocalDate::of(2021, 2, 28));
    // April 30 days
    assert_eq!(LocalDate::last_day_of_month_year(2021, 4), LocalDate::of(2021, 4, 30));
    // December 31 days
    assert_eq!(LocalDate::last_day_of_month_year(2023, 12), LocalDate::of(2023, 12, 31));
}

#[test]
fn year_and_weekday_helpers() {
    use joda_rs::DayOfWeek::*;
    // 2021 year checks
    let d = LocalDate::of(2021, 3, 14); // Sunday
    assert_eq!(d.first_day_of_year(), LocalDate::of(2021, 1, 1));
    assert_eq!(d.last_day_of_year(), LocalDate::of(2021, 12, 31));
    assert_eq!(d.first_day_of_next_month(), LocalDate::of(2021, 4, 1));
    assert_eq!(d.first_day_of_next_year(), LocalDate::of(2022, 1, 1));

    // first/last in month
    let jan2021 = LocalDate::of(2021, 1, 15);
    assert_eq!(jan2021.first_in_month(Monday), LocalDate::of(2021, 1, 4));
    assert_eq!(jan2021.first_in_month(Friday), LocalDate::of(2021, 1, 1));
    assert_eq!(jan2021.last_in_month(Sunday), LocalDate::of(2021, 1, 31));
    assert_eq!(jan2021.last_in_month(Tuesday), LocalDate::of(2021, 1, 26));

    // next/previous family from a Sunday (2021-03-14)
    assert_eq!(d.next(Monday), LocalDate::of(2021, 3, 15));
    assert_eq!(d.next_or_same(Sunday), d);
    assert_eq!(d.next_or_same(Monday), LocalDate::of(2021, 3, 15));
    assert_eq!(d.previous(Saturday), LocalDate::of(2021, 3, 13));
    assert_eq!(d.previous_or_same(Sunday), d);
    assert_eq!(d.previous_or_same(Friday), LocalDate::of(2021, 3, 12));

    // Leap-year month check (Feb 2020)
    let feb2020 = LocalDate::of(2020, 2, 10);
    assert_eq!(feb2020.first_in_month(Saturday), LocalDate::of(2020, 2, 1));
    assert_eq!(feb2020.last_in_month(Saturday), LocalDate::of(2020, 2, 29));
}


#[test]
fn plus_months_handles_january_31_days() {
    // Moving into January should preserve day 31 because January has 31 days.
    let d = LocalDate::of(2021, 12, 31).plus_months(1);
    assert_eq!(d, LocalDate::of(2022, 1, 31));

    // Also test via a negative move: March 31 back by 2 months -> January 31
    let d2 = LocalDate::of(2020, 3, 31).minus_months(2);
    assert_eq!(d2, LocalDate::of(2020, 1, 31));
}
