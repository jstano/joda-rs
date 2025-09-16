use joda_rs::{LocalDate, LocalDateTime, LocalTime, Month};

#[test]
fn plus_minus_days_weeks_and_time_units() {
    let ldt = LocalDateTime::of(2020, 1, 31, 23, 0, 0);
    // days
    assert_eq!(ldt.plus_days(1), LocalDateTime::of(2020, 2, 1, 23, 0, 0));
    assert_eq!(ldt.minus_days(1), LocalDateTime::of(2020, 1, 30, 23, 0, 0));
    // weeks
    assert_eq!(ldt.plus_weeks(1), LocalDateTime::of(2020, 2, 7, 23, 0, 0));
    assert_eq!(ldt.minus_weeks(2), LocalDateTime::of(2020, 1, 17, 23, 0, 0));
    // hours crossing midnight
    assert_eq!(LocalDateTime::of(2020, 1, 1, 23, 0, 0).plus_hours(2), LocalDateTime::of(2020, 1, 2, 1, 0, 0));
    assert_eq!(LocalDateTime::of(2020, 1, 2, 0, 0, 0).minus_seconds(1), LocalDateTime::of(2020, 1, 1, 23, 59, 59));
    // nanos
    let n = LocalDateTime::of(2020, 1, 1, 0, 0, 0).minus_nanos(1);
    // time::PrimitiveDateTime preserves nanoseconds; we moved to the last nanosecond of previous second
    assert_eq!(n, LocalDateTime::of(2019, 12, 31, 23, 59, 59).with_nano(999_999_999));
}

#[test]
fn plus_minus_months_years_clamping() {
    // end-of-month clamping like java.time
    assert_eq!(LocalDateTime::of(2020, 1, 31, 12, 34, 56).plus_months(1), LocalDateTime::of(2020, 2, 29, 12, 34, 56));
    assert_eq!(LocalDateTime::of(2019, 1, 31, 12, 0, 0).plus_months(1), LocalDateTime::of(2019, 2, 28, 12, 0, 0));
    assert_eq!(LocalDateTime::of(2020, 3, 31, 6, 0, 0).minus_months(1), LocalDateTime::of(2020, 2, 29, 6, 0, 0));

    // years: Feb 29 clamped on non-leap
    assert_eq!(LocalDateTime::of(2020, 2, 29, 1, 2, 3).plus_years(1), LocalDateTime::of(2021, 2, 28, 1, 2, 3));
    assert_eq!(LocalDateTime::of(2020, 2, 29, 1, 2, 3).minus_years(1), LocalDateTime::of(2019, 2, 28, 1, 2, 3));
}

#[test]
fn with_adjusters_date_and_time() {
    let ldt = LocalDateTime::of(2021, 6, 15, 10, 20, 30);
    assert_eq!(ldt.with_day_of_month(1), LocalDateTime::of(2021, 6, 1, 10, 20, 30));
    assert_eq!(ldt.with_day_of_year(200), LocalDateTime::of(2021, 7, 19, 10, 20, 30));
    assert_eq!(ldt.with_month(2), LocalDateTime::of(2021, 2, 15, 10, 20, 30));
    assert_eq!(ldt.with_year(2020), LocalDateTime::of(2020, 6, 15, 10, 20, 30));

    assert_eq!(ldt.with_hour(5), LocalDateTime::of(2021, 6, 15, 5, 20, 30));
    assert_eq!(ldt.with_minute(59), LocalDateTime::of(2021, 6, 15, 10, 59, 30));
    assert_eq!(ldt.with_second(0), LocalDateTime::of(2021, 6, 15, 10, 20, 0));
    let ldt2 = ldt.with_nano(999_999_999);
    assert_eq!(ldt2.to_local_time().hour(), 10);
}

#[test]
fn comparisons_and_conversions() {
    let a = LocalDateTime::of(2020, 1, 1, 0, 0, 0);
    let b = LocalDateTime::of(2020, 1, 1, 0, 0, 1);
    assert!(a.is_before(b));
    assert!(b.is_after(a));
    assert!(a.is_on_or_before(b));
    assert!(b.is_on_or_after(a));
    assert!(a.is_on_or_before(a));
    assert!(a.is_on_or_after(a));

    let d = a.to_local_date();
    let t = b.to_local_time();
    assert_eq!(d, LocalDate::of(2020, 1, 1));
    assert_eq!(t, LocalTime::of(0, 0, 1));

    // Month enum can be used by callers along with date part in other APIs; basic conversion check
    let _m = Month::February; // smoke
}


#[test]
fn last_day_of_month_year_preserves_time() {
    let ldt = LocalDateTime::of(2021, 1, 10, 8, 30, 0);
    // Non-leap Feb 2021
    let feb_last = ldt.last_day_of_month_year(2021, 2);
    assert_eq!(feb_last, LocalDateTime::of(2021, 2, 28, 8, 30, 0));
    // Leap Feb 2020
    let feb_last_leap = ldt.last_day_of_month_year(2020, 2);
    assert_eq!(feb_last_leap, LocalDateTime::of(2020, 2, 29, 8, 30, 0));
}

#[test]
fn date_part_helpers_preserve_time() {
    use joda_rs::DayOfWeek::*;
    let ldt = LocalDateTime::of(2021, 3, 14, 8, 30, 0); // Sunday 08:30
    assert_eq!(ldt.first_day_of_month(), LocalDateTime::of(2021, 3, 1, 8, 30, 0));
    assert_eq!(ldt.last_day_of_month(), LocalDateTime::of(2021, 3, 31, 8, 30, 0));
    assert_eq!(ldt.first_day_of_year(), LocalDateTime::of(2021, 1, 1, 8, 30, 0));
    assert_eq!(ldt.last_day_of_year(), LocalDateTime::of(2021, 12, 31, 8, 30, 0));
    assert_eq!(ldt.first_day_of_next_month(), LocalDateTime::of(2021, 4, 1, 8, 30, 0));
    assert_eq!(ldt.first_day_of_next_year(), LocalDateTime::of(2022, 1, 1, 8, 30, 0));

    assert_eq!(ldt.first_in_month(Monday), LocalDateTime::of(2021, 3, 1, 8, 30, 0));
    assert_eq!(ldt.last_in_month(Tuesday), LocalDateTime::of(2021, 3, 30, 8, 30, 0));

    assert_eq!(ldt.next(Monday), LocalDateTime::of(2021, 3, 15, 8, 30, 0));
    assert_eq!(ldt.next_or_same(Sunday), ldt);
    assert_eq!(ldt.previous(Saturday), LocalDateTime::of(2021, 3, 13, 8, 30, 0));
    assert_eq!(ldt.previous_or_same(Sunday), ldt);
}
