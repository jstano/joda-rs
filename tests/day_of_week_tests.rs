use joda_rs::{DayOfWeek, LocalDate};

#[test]
fn of_and_value_and_display() {
    assert_eq!(DayOfWeek::of(1), DayOfWeek::Monday);
    assert_eq!(DayOfWeek::of(7), DayOfWeek::Sunday);
    assert_eq!(DayOfWeek::Friday.value(), 5);

    // Display uppercase
    assert_eq!(format!("{}", DayOfWeek::Wednesday), "WEDNESDAY");
}

#[test]
fn plus_minus_wrapping() {
    assert_eq!(DayOfWeek::Monday.plus(7), DayOfWeek::Monday);
    assert_eq!(DayOfWeek::Monday.plus(8), DayOfWeek::Tuesday);
    assert_eq!(DayOfWeek::Sunday.plus(1), DayOfWeek::Monday);

    assert_eq!(DayOfWeek::Monday.minus(1), DayOfWeek::Sunday);
    assert_eq!(DayOfWeek::Thursday.minus(5), DayOfWeek::Saturday);
}

#[test]
fn from_date_weekday() {
    // Known date: 2024-09-02 was Monday
    let d = LocalDate::of(2024, 9, 2);
    assert_eq!(DayOfWeek::from_date(d), DayOfWeek::Monday);

    // Another known: 2020-02-29 was Saturday
    let d2 = LocalDate::of(2020, 2, 29);
    assert_eq!(DayOfWeek::from_date(d2), DayOfWeek::Saturday);
}
