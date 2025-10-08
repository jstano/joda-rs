use crate::{DayOfWeek, LocalDate, LocalDateTime, LocalTime, Month};

pub trait Temporal: Ord + Copy {
    fn is_before(self, other: Self) -> bool;

    fn is_after(self, other: Self) -> bool;

    fn is_on_or_before(self, other: Self) -> bool;

    fn is_on_or_after(self, other: Self) -> bool;
}

pub trait TemporalInstant {
    fn epoch_seconds(self) -> i64;

    fn epoch_milliseconds(self) -> i128;

    fn epoch_nanoseconds(self) -> i128;
}

pub trait TemporalDate: Temporal {
    fn year(self) -> i32;

    fn month(self) -> Month;

    fn month_value(self) -> i32;

    fn day_of_year(self) -> u16;

    fn day_of_month(self) -> u8;

    fn day_of_week(self) -> DayOfWeek;

    fn length_of_month(self) -> i32;

    fn is_leap_year(self) -> bool;

    fn length_of_year(self) -> i32;

    fn plus_years(self, years: i64) -> Self;

    fn minus_years(self, years: i64) -> Self;

    fn plus_months(self, months: i64) -> Self;

    fn minus_months(self, months: i64) -> Self;

    fn plus_weeks(self, weeks: i64) -> Self;

    fn minus_weeks(self, weeks: i64) -> Self;

    fn plus_days(self, days: i64) -> Self;

    fn minus_days(self, days: i64) -> Self;

    fn with_year(self, year: i32) -> Self;

    fn with_month(self, month: i32) -> Self;

    fn with_day_of_month(self, day: u8) -> Self;

    fn with_day_of_year(self, day_of_year: u16) -> Self;

    fn first_day_of_month(self) -> Self;

    fn last_day_of_month(self) -> Self;

    fn first_day_of_year(self) -> Self;

    fn last_day_of_year(self) -> Self;

    fn first_in_month(self, dow: DayOfWeek) -> Self;

    fn last_in_month(self, dow: DayOfWeek) -> Self;

    fn next(self, dow: DayOfWeek) -> Self;

    fn next_or_same(self, dow: DayOfWeek) -> Self;

    fn previous(self, dow: DayOfWeek) -> Self;

    fn previous_or_same(self, dow: DayOfWeek) -> Self;
}

pub trait TemporalTime: Temporal {
    fn hour(self) -> u8;

    fn minute(self) -> u8;

    fn second(self) -> u8;

    fn millisecond(self) -> u16;

    fn nanosecond(self) -> i32;

    fn total_nanoseconds_of_day(self) -> u128;

    fn plus_hours(self, hours: i64) -> Self;

    fn minus_hours(self, hours: i64) -> Self;

    fn plus_minutes(self, minutes: i64) -> Self;

    fn minus_minutes(self, minutes: i64) -> Self;

    fn plus_seconds(self, seconds: i64) -> Self;

    fn minus_seconds(self, seconds: i64) -> Self;

    fn plus_milliseconds(self, milliseconds: i64) -> Self;

    fn minus_milliseconds(self, milliseconds: i64) -> Self;

    fn plus_nanoseconds(self, nanoseconds: i64) -> Self;

    fn minus_nanoseconds(self, nanoseconds: i64) -> Self;

    fn with_hour(self, hour: u8) -> Self;

    fn with_minute(self, minute: u8) -> Self;

    fn with_second(self, second: u8) -> Self;

    fn with_millisecond(self, millisecond: u16) -> Self;

    fn with_nanosecond(self, nanosecond: u32) -> Self;
}

pub trait TemporalDateTime: TemporalDate + TemporalTime {
    fn to_local_date(self) -> LocalDate;

    fn to_local_time(self) -> LocalTime;

    fn to_local_date_time(self) -> LocalDateTime;
}
