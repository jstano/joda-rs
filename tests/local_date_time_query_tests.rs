use joda_rs::{DayOfWeek, LocalDateTime};

#[test]
fn day_of_year_and_day_of_week_queries() {
    // 2020-02-29 was a Saturday, day-of-year 60 (leap year)
    let ldt = LocalDateTime::of(2020, 2, 29, 12, 34, 56);
    assert_eq!(ldt.day_of_year(), 60);
    assert_eq!(ldt.day_of_week(), DayOfWeek::Saturday);

    // Another sample: 2021-03-14 was a Sunday, doy 73
    let ldt2 = LocalDateTime::of(2021, 3, 14, 0, 0, 0);
    assert_eq!(ldt2.day_of_year(), 73);
    assert_eq!(ldt2.day_of_week(), DayOfWeek::Sunday);
}
