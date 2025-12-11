pub mod local_date;
pub mod local_time;
pub mod local_date_time;
pub mod offset_date_time;
pub mod zoned_date_time;
pub mod instant;
pub mod duration;
pub mod zone_offset;
pub mod zone_id;
pub mod period;
pub mod day_of_week;
pub mod month;
pub mod year;
pub mod year_month;
pub mod month_day;
pub mod clock;
pub mod temporal;
pub mod chrono_unit;
pub mod constants;
pub mod serde_time;

pub use chrono_unit::ChronoUnit;
pub use clock::{Clock, FixedClock, SystemClock};
pub use day_of_week::DayOfWeek;
pub use duration::Duration;
pub use instant::Instant;
pub use local_date::LocalDate;
pub use local_date_time::LocalDateTime;
pub use local_time::LocalTime;
pub use month::Month;
pub use month_day::MonthDay;
pub use offset_date_time::OffsetDateTime;
pub use period::Period;
pub use temporal::TemporalInstant;
pub use year::Year;
pub use year_month::YearMonth;
pub use zone_id::ZoneId;
pub use zone_offset::ZoneOffset;
pub use zoned_date_time::ZonedDateTime;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn wrappers_smoke_test() {
        let ldt = LocalDateTime::of(2024, 1, 1, 12, 0, 0);
        let off = ZoneOffset::of_hours(0);
        let odt = OffsetDateTime::of(ldt, off);
        let _zid = ZoneId::UTC;
        let _zdt = ZonedDateTime::of(ldt, _zid);
        let _inst = Instant::now();
        let _dur = Duration::of_seconds(5);
        let p = Period::of(1, 2, 3);
        assert_eq!(p, Period::new(1, 2, 3));
        // Access underlying
        let _ = ldt.inner();
        let _ = odt.inner();
        // Check now() exists
        let _ = LocalDate::now();
        let _ = LocalTime::now();
        let _ = LocalDateTime::now();
        let _ = OffsetDateTime::now_utc();
        let _ = ZonedDateTime::now_utc();
        // Duration between instants
        let start = Instant::now();
        let end = Instant::now();
        let _ = Duration::between(start, end);

        // LocalDate now with ZoneId and parse
        let d_zone = LocalDate::now_with_zone(ZoneId::UTC);
        let parsed = LocalDate::parse("2025-09-15");
        let expected = LocalDate::of(2025, 9, 15);
        assert_eq!(parsed, expected);
        let _ = d_zone; // smoke

        // LocalDate.at_time and at_start_of_day
        let d2 = LocalDate::of(2023, 5, 6);
        let ltime = LocalTime::of(1, 2, 3);
        let ldt1 = d2.at_time(ltime);
        let ldt_expected = LocalDateTime::of(2023, 5, 6, 1, 2, 3);
        assert_eq!(ldt1, ldt_expected);
        let ldt_midnight = d2.at_start_of_day();
        let ldt_midnight_expected = LocalDateTime::of(2023, 5, 6, 0, 0, 0);
        assert_eq!(ldt_midnight, ldt_midnight_expected);

        // DayOfWeek basic behavior
        let mon = DayOfWeek::of(1);
        assert_eq!(mon, DayOfWeek::Monday);
        assert_eq!(DayOfWeek::Sunday.value(), 7);
        assert_eq!(DayOfWeek::Friday.plus(3), DayOfWeek::Monday);
        assert_eq!(DayOfWeek::Monday.minus(1), DayOfWeek::Sunday);

        // Month basic behavior
        let feb = Month::of(2);
        assert_eq!(feb, Month::February);
        assert_eq!(Month::December.value(), 12);
        assert_eq!(Month::January.minus(1), Month::December);
        assert_eq!(Month::November.plus(2), Month::January);
        assert_eq!(Month::February.length(false), 28);
        assert_eq!(Month::February.length(true), 29);

        // Year basic behavior
        let y = Year::of(2024);
        assert!(y.is_leap());
        assert_eq!(y.length(), 366);
        assert_eq!(y.plus(1).value(), 2025);
        assert_eq!(y.minus(25).value(), 1999);
        let d_jan1 = y.at_day(1);
        assert_eq!(d_jan1, LocalDate::of(2024, 1, 1));
        let d_feb29 = y.at_month_day(Month::February, 29);
        assert_eq!(d_feb29, LocalDate::of(2024, 2, 29));

        // LocalDate query methods
        let qd = LocalDate::of(2020, 2, 29);
        assert_eq!(qd.day_of_month(), 29);
        assert_eq!(qd.day_of_year(), 60); // 2020 is leap
        assert_eq!(qd.length_of_month(), 29);
        assert_eq!(qd.year(), 2020);
        let dow = qd.day_of_week();
        // 2020-02-29 was Saturday
        assert_eq!(dow, DayOfWeek::Saturday);
        // New queries
        assert_eq!(qd.month(), Month::February);
        assert_eq!(qd.month_value(), 2);
        assert!(qd.is_leap_year());
        assert_eq!(qd.length_of_year(), 366);

        // Non-leap sample
        let qd2 = LocalDate::of(2021, 3, 14);
        assert_eq!(qd2.month(), Month::March);
        assert_eq!(qd2.month_value(), 3);
        assert!(!qd2.is_leap_year());
        assert_eq!(qd2.length_of_year(), 365);

        // LocalDate arithmetic plus/minus weeks/months/years
        let base = LocalDate::of(2020, 1, 31);
        // weeks
        assert_eq!(base.plus_weeks(1), LocalDate::of(2020, 2, 7));
        assert_eq!(base.minus_weeks(2), LocalDate::of(2020, 1, 17));
        // months (clamp to month length like java.time)
        assert_eq!(base.plus_months(1), LocalDate::of(2020, 2, 29)); // leap year Feb
        assert_eq!(LocalDate::of(2019, 1, 31).plus_months(1), LocalDate::of(2019, 2, 28));
        assert_eq!(LocalDate::of(2020, 3, 31).minus_months(1), LocalDate::of(2020, 2, 29));
        // years (Feb 29 clamped on non-leap)
        assert_eq!(LocalDate::of(2020, 2, 29).plus_years(1), LocalDate::of(2021, 2, 28));
        assert_eq!(LocalDate::of(2020, 2, 29).minus_years(1), LocalDate::of(2019, 2, 28));

        // with_* methods
        let w = LocalDate::of(2021, 6, 15);
        assert_eq!(w.with_day_of_month(1), LocalDate::of(2021, 6, 1));
        assert_eq!(w.with_day_of_year(200), LocalDate::of(2021, 7, 19));
        assert_eq!(w.with_month(2), LocalDate::of(2021, 2, 15));
        assert_eq!(w.with_year(2020), LocalDate::of(2020, 6, 15));

        // is_before / is_after
        assert!(LocalDate::of(2020, 1, 1).is_before(LocalDate::of(2020, 1, 2)));
        assert!(LocalDate::of(2020, 1, 2).is_after(LocalDate::of(2020, 1, 1)));
        // is_on_or_before / is_on_or_after
        let a = LocalDate::of(2020, 1, 1);
        let b = LocalDate::of(2020, 1, 2);
        let a2 = LocalDate::of(2020, 1, 1);
        assert!(a.is_on_or_before(b));
        assert!(b.is_on_or_after(a));
        // equality cases should be true for both
        assert!(a.is_on_or_before(a2));
        assert!(a.is_on_or_after(a2));
    }
}
