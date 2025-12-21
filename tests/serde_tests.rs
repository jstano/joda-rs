#![cfg(feature = "serde")]

use joda_rs::*;
use serde_json;

#[test]
fn local_date_serde_round_trip() {
    let date = LocalDate::of(2025, 12, 12);
    let json = serde_json::to_string(&date).expect("serialization failed");
    let deserialized: LocalDate = serde_json::from_str(&json).expect("deserialization failed");
    assert_eq!(date, deserialized);
}

#[test]
fn local_date_deserializes_from_iso8601() {
    let json = r#""2025-12-12""#;
    let date: LocalDate = serde_json::from_str(json).expect("deserialization failed");
    assert_eq!(date, LocalDate::of(2025, 12, 12));
}

#[test]
fn local_date_serializes_to_iso8601() {
    let date = LocalDate::of(2025, 1, 5);
    let json = serde_json::to_string(&date).expect("serialization failed");
    assert_eq!(json, r#""2025-01-05""#);
}

#[test]
fn local_time_serde_round_trip() {
    let time = LocalTime::of(14, 30, 45);
    let json = serde_json::to_string(&time).expect("serialization failed");
    let deserialized: LocalTime = serde_json::from_str(&json).expect("deserialization failed");
    assert_eq!(time, deserialized);
}

#[test]
fn local_time_deserializes_from_iso8601() {
    let json = r#""14:30:45""#;
    let time: LocalTime = serde_json::from_str(json).expect("deserialization failed");
    assert_eq!(time, LocalTime::of(14, 30, 45));
}

#[test]
fn local_time_serializes_to_iso8601() {
    let time = LocalTime::of(9, 5, 3);
    let json = serde_json::to_string(&time).expect("serialization failed");
    // Note: subseconds are included even when zero
    assert_eq!(json, r#""09:05:03.0""#);
}

#[test]
fn local_time_with_nanoseconds_round_trip() {
    let time = LocalTime::of_hms_nano(14, 30, 45, 123456789);
    let json = serde_json::to_string(&time).expect("serialization failed");
    let deserialized: LocalTime = serde_json::from_str(&json).expect("deserialization failed");
    assert_eq!(time, deserialized);
}

#[test]
fn local_date_time_serde_round_trip() {
    let datetime = LocalDateTime::of(2025, 12, 12, 14, 30, 45);
    let json = serde_json::to_string(&datetime).expect("serialization failed");
    let deserialized: LocalDateTime = serde_json::from_str(&json).expect("deserialization failed");
    assert_eq!(datetime, deserialized);
}

#[test]
fn local_date_time_deserializes_from_iso8601() {
    let json = r#""2025-12-12T14:30:45""#;
    let datetime: LocalDateTime = serde_json::from_str(json).expect("deserialization failed");
    assert_eq!(datetime, LocalDateTime::of(2025, 12, 12, 14, 30, 45));
}

#[test]
fn local_date_time_serializes_to_iso8601() {
    let datetime = LocalDateTime::of(2025, 1, 5, 9, 5, 3);
    let json = serde_json::to_string(&datetime).expect("serialization failed");
    assert_eq!(json, r#""2025-01-05T09:05:03""#);
}

#[test]
fn offset_date_time_serde_round_trip() {
    let datetime = LocalDateTime::of(2025, 12, 12, 14, 30, 45);
    let offset = ZoneOffset::of_hours(0);
    let odt = OffsetDateTime::of(datetime, offset);
    let json = serde_json::to_string(&odt).expect("serialization failed");
    let deserialized: OffsetDateTime = serde_json::from_str(&json).expect("deserialization failed");
    assert_eq!(odt, deserialized);
}

#[test]
fn offset_date_time_with_offset_round_trip() {
    let datetime = LocalDateTime::of(2025, 12, 12, 14, 30, 45);
    let offset = ZoneOffset::of_hours(-5);
    let odt = OffsetDateTime::of(datetime, offset);
    let json = serde_json::to_string(&odt).expect("serialization failed");
    let deserialized: OffsetDateTime = serde_json::from_str(&json).expect("deserialization failed");
    assert_eq!(odt, deserialized);
}

#[test]
fn zoned_date_time_serde_round_trip() {
    let datetime = LocalDateTime::of(2025, 12, 12, 14, 30, 45);
    let zdt = ZonedDateTime::of(datetime, ZoneId::UTC);
    let json = serde_json::to_string(&zdt).expect("serialization failed");
    let deserialized: ZonedDateTime = serde_json::from_str(&json).expect("deserialization failed");
    assert_eq!(zdt, deserialized);
}

#[test]
fn instant_serde_round_trip() {
    let instant = Instant::of_epoch_millisecond(1734012645000);
    let json = serde_json::to_string(&instant).expect("serialization failed");
    let deserialized: Instant = serde_json::from_str(&json).expect("deserialization failed");
    assert_eq!(instant, deserialized);
}

#[test]
fn duration_serde_round_trip() {
    let duration = Duration::of_seconds(3600);
    let json = serde_json::to_string(&duration).expect("serialization failed");
    let deserialized: Duration = serde_json::from_str(&json).expect("deserialization failed");
    assert_eq!(duration, deserialized);
}

#[test]
fn duration_with_nanos_round_trip() {
    let duration = Duration::of_seconds(3600).plus_nanoseconds(123456789);
    let json = serde_json::to_string(&duration).expect("serialization failed");
    let deserialized: Duration = serde_json::from_str(&json).expect("deserialization failed");
    assert_eq!(duration, deserialized);
}

#[test]
fn period_serde_round_trip() {
    let period = Period::of(1, 2, 15);
    let json = serde_json::to_string(&period).expect("serialization failed");
    let deserialized: Period = serde_json::from_str(&json).expect("deserialization failed");
    assert_eq!(period, deserialized);
}

#[test]
fn day_of_week_serde_round_trip() {
    let dow = DayOfWeek::Monday;
    let json = serde_json::to_string(&dow).expect("serialization failed");
    let deserialized: DayOfWeek = serde_json::from_str(&json).expect("deserialization failed");
    assert_eq!(dow, deserialized);
}

#[test]
fn day_of_week_all_values_round_trip() {
    for day in [
        DayOfWeek::Monday,
        DayOfWeek::Tuesday,
        DayOfWeek::Wednesday,
        DayOfWeek::Thursday,
        DayOfWeek::Friday,
        DayOfWeek::Saturday,
        DayOfWeek::Sunday,
    ] {
        let json = serde_json::to_string(&day).expect("serialization failed");
        let deserialized: DayOfWeek = serde_json::from_str(&json).expect("deserialization failed");
        assert_eq!(day, deserialized);
    }
}

#[test]
fn month_serde_round_trip() {
    let month = Month::December;
    let json = serde_json::to_string(&month).expect("serialization failed");
    let deserialized: Month = serde_json::from_str(&json).expect("deserialization failed");
    assert_eq!(month, deserialized);
}

#[test]
fn month_all_values_round_trip() {
    for month in [
        Month::January,
        Month::February,
        Month::March,
        Month::April,
        Month::May,
        Month::June,
        Month::July,
        Month::August,
        Month::September,
        Month::October,
        Month::November,
        Month::December,
    ] {
        let json = serde_json::to_string(&month).expect("serialization failed");
        let deserialized: Month = serde_json::from_str(&json).expect("deserialization failed");
        assert_eq!(month, deserialized);
    }
}

#[test]
fn year_serde_round_trip() {
    let year = Year::of(2025);
    let json = serde_json::to_string(&year).expect("serialization failed");
    let deserialized: Year = serde_json::from_str(&json).expect("deserialization failed");
    assert_eq!(year, deserialized);
}

#[test]
fn year_month_serde_round_trip() {
    let ym = YearMonth::of(2025, 12);
    let json = serde_json::to_string(&ym).expect("serialization failed");
    let deserialized: YearMonth = serde_json::from_str(&json).expect("deserialization failed");
    assert_eq!(ym, deserialized);
}

#[test]
fn month_day_serde_round_trip() {
    let md = MonthDay::of(12, 25);
    let json = serde_json::to_string(&md).expect("serialization failed");
    let deserialized: MonthDay = serde_json::from_str(&json).expect("deserialization failed");
    assert_eq!(md, deserialized);
}

#[test]
fn zone_offset_serde_round_trip() {
    let offset = ZoneOffset::of_hours(-5);
    let json = serde_json::to_string(&offset).expect("serialization failed");
    let deserialized: ZoneOffset = serde_json::from_str(&json).expect("deserialization failed");
    assert_eq!(offset, deserialized);
}

#[test]
fn chrono_unit_serde_round_trip() {
    let units = [
        ChronoUnit::Nanos,
        ChronoUnit::Millis,
        ChronoUnit::Seconds,
        ChronoUnit::Minutes,
        ChronoUnit::Hours,
        ChronoUnit::HalfDays,
        ChronoUnit::Days,
        ChronoUnit::Weeks,
        ChronoUnit::Months,
        ChronoUnit::Years,
    ];

    for unit in units {
        let json = serde_json::to_string(&unit).expect("serialization failed");
        let deserialized: ChronoUnit = serde_json::from_str(&json).expect("deserialization failed");
        assert_eq!(unit, deserialized);
    }
}

#[test]
fn local_date_in_struct_round_trip() {
    #[derive(Debug, PartialEq, serde::Serialize, serde::Deserialize)]
    struct Event {
        name: String,
        date: LocalDate,
    }

    let event = Event {
        name: "Meeting".to_string(),
        date: LocalDate::of(2025, 12, 12),
    };

    let json = serde_json::to_string(&event).expect("serialization failed");
    let deserialized: Event = serde_json::from_str(&json).expect("deserialization failed");
    assert_eq!(event, deserialized);
}

#[test]
fn local_date_time_in_struct_round_trip() {
    #[derive(Debug, PartialEq, serde::Serialize, serde::Deserialize)]
    struct Appointment {
        title: String,
        start: LocalDateTime,
        end: LocalDateTime,
    }

    let appt = Appointment {
        title: "Standup".to_string(),
        start: LocalDateTime::of(2025, 12, 12, 9, 0, 0),
        end: LocalDateTime::of(2025, 12, 12, 9, 30, 0),
    };

    let json = serde_json::to_string(&appt).expect("serialization failed");
    let deserialized: Appointment = serde_json::from_str(&json).expect("deserialization failed");
    assert_eq!(appt, deserialized);
}

#[test]
fn duration_and_period_in_struct_round_trip() {
    #[derive(Debug, PartialEq, serde::Serialize, serde::Deserialize)]
    struct TimeSpan {
        duration: Duration,
        period: Period,
    }

    let span = TimeSpan {
        duration: Duration::of_hours(24),
        period: Period::of(1, 0, 0),
    };

    let json = serde_json::to_string(&span).expect("serialization failed");
    let deserialized: TimeSpan = serde_json::from_str(&json).expect("deserialization failed");
    assert_eq!(span, deserialized);
}

#[test]
fn leap_year_date_round_trip() {
    let date = LocalDate::of(2020, 2, 29);
    let json = serde_json::to_string(&date).expect("serialization failed");
    let deserialized: LocalDate = serde_json::from_str(&json).expect("deserialization failed");
    assert_eq!(date, deserialized);
    assert_eq!(json, r#""2020-02-29""#);
}

#[test]
fn negative_year_date_round_trip() {
    let date = LocalDate::of(-100, 6, 15);
    let json = serde_json::to_string(&date).expect("serialization failed");
    let deserialized: LocalDate = serde_json::from_str(&json).expect("deserialization failed");
    assert_eq!(date, deserialized);
}

#[test]
fn midnight_time_round_trip() {
    let time = LocalTime::of(0, 0, 0);
    let json = serde_json::to_string(&time).expect("serialization failed");
    let deserialized: LocalTime = serde_json::from_str(&json).expect("deserialization failed");
    assert_eq!(time, deserialized);
}

#[test]
fn end_of_day_time_round_trip() {
    let time = LocalTime::of(23, 59, 59);
    let json = serde_json::to_string(&time).expect("serialization failed");
    let deserialized: LocalTime = serde_json::from_str(&json).expect("deserialization failed");
    assert_eq!(time, deserialized);
}

#[test]
fn negative_duration_round_trip() {
    let duration = Duration::of_seconds(-3600);
    let json = serde_json::to_string(&duration).expect("serialization failed");
    let deserialized: Duration = serde_json::from_str(&json).expect("deserialization failed");
    assert_eq!(duration, deserialized);
}

#[test]
fn negative_period_round_trip() {
    let period = Period::of(-1, -2, -15);
    let json = serde_json::to_string(&period).expect("serialization failed");
    let deserialized: Period = serde_json::from_str(&json).expect("deserialization failed");
    assert_eq!(period, deserialized);
}

#[test]
fn zero_offset_round_trip() {
    let offset = ZoneOffset::of_hours(0);
    let json = serde_json::to_string(&offset).expect("serialization failed");
    let deserialized: ZoneOffset = serde_json::from_str(&json).expect("deserialization failed");
    assert_eq!(offset, deserialized);
}

#[test]
fn positive_offset_round_trip() {
    let offset = ZoneOffset::of_hours(12);
    let json = serde_json::to_string(&offset).expect("serialization failed");
    let deserialized: ZoneOffset = serde_json::from_str(&json).expect("deserialization failed");
    assert_eq!(offset, deserialized);
}
