use crate::{Duration, Instant, LocalDate, LocalDateTime, LocalTime, TemporalInstant};
use std::fmt;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// A minimal equivalent of java.time.temporal.ChronoUnit.
///
/// This enum provides a set of commonly used time units and
/// helper methods to inspect whether a unit is date- or time-based
/// and to obtain an approximate Duration for the unit.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum ChronoUnit {
    Nanos,
    Millis,
    Seconds,
    Minutes,
    Hours,
    HalfDays,
    Days,
    Weeks,
    Months,
    Years,
}

impl ChronoUnit {
    /// Returns true if this unit is time-based (nano up to half-days/hours/minutes/seconds/millis).
    pub fn is_time_based(self) -> bool {
        matches!(
            self,
            ChronoUnit::Nanos
                | ChronoUnit::Millis
                | ChronoUnit::Seconds
                | ChronoUnit::Minutes
                | ChronoUnit::Hours
                | ChronoUnit::HalfDays
        )
    }

    /// Returns true if this unit is date-based (days and larger calendar-based units).
    pub fn is_date_based(self) -> bool {
        matches!(self, ChronoUnit::Days | ChronoUnit::Weeks | ChronoUnit::Months | ChronoUnit::Years)
    }

    /// Returns a Duration representing this unit.
    ///
    /// For non-fixed length calendar units (Months, Years) this uses simple approximations
    /// similar in spirit to java.time (but not the exact fractional day constants):
    /// - Months are approximated as 30 days
    /// - Years are approximated as 365 days
    pub fn duration(self) -> Duration {
        match self {
            ChronoUnit::Nanos => Duration::of_nanoseconds(1),
            ChronoUnit::Millis => Duration::of_milliseconds(1),
            ChronoUnit::Seconds => Duration::of_seconds(1),
            ChronoUnit::Minutes => Duration::of_minutes(1),
            ChronoUnit::Hours => Duration::of_hours(1),
            ChronoUnit::HalfDays => Duration::of_hours(12),
            ChronoUnit::Days => Duration::of_days(1),
            ChronoUnit::Weeks => Duration::of_days(7),
            ChronoUnit::Months => Duration::of_days(30),
            ChronoUnit::Years => Duration::of_days(365),
        }
    }

    /// Adds the specified amount of this unit to the given temporal and returns the result.
    ///
    /// Supported temporals: Instant, LocalDate, LocalTime, LocalDateTime.
    /// For unsupported unit/temporal combinations, this will panic with a clear message.
    pub fn add_to<T: ChronoAdd + Copy>(self, temporal: T, amount: i64) -> T {
        temporal.cu_add(self, amount)
    }

    /// Returns the number of whole units between two instants (truncating toward zero).
    ///
    /// This operates on types that expose an epoch-based timeline via TemporalInstant
    /// (e.g., Instant, LocalDateTime). For calendar-based approximations, Months are
    /// treated as 30 days and Years as 365 days, consistent with `duration()`.
    pub fn between<T: TemporalInstant>(self, start_inclusive: T, end_exclusive: T) -> i64 {
        let nanos_start = start_inclusive.epoch_nanoseconds();
        let nanos_end = end_exclusive.epoch_nanoseconds();
        let diff = nanos_end - nanos_start; // i128
        let unit_nanos: i128 = match self {
            ChronoUnit::Nanos => 1,
            ChronoUnit::Millis => 1_000_000,
            ChronoUnit::Seconds => 1_000_000_000,
            ChronoUnit::Minutes => 60 * 1_000_000_000,
            ChronoUnit::Hours => 3_600 * 1_000_000_000,
            ChronoUnit::HalfDays => 43_200 * 1_000_000_000, // 12h
            ChronoUnit::Days => 86_400 * 1_000_000_000,
            ChronoUnit::Weeks => 7 * 86_400 * 1_000_000_000,
            ChronoUnit::Months => 30 * 86_400 * 1_000_000_000,
            ChronoUnit::Years => 365 * 86_400 * 1_000_000_000,
        };
        (diff / unit_nanos) as i64
    }
}

/// Internal trait to support ChronoUnit::add_to over multiple temporal types without
/// exposing a large generic surface. Each implementor decides which units it supports.
pub trait ChronoAdd {
    fn cu_add(self, unit: ChronoUnit, amount: i64) -> Self;
}

impl ChronoAdd for Instant {
    fn cu_add(self, unit: ChronoUnit, amount: i64) -> Self {
        match unit {
            ChronoUnit::Nanos => self.plus_nanoseconds(amount),
            ChronoUnit::Millis => self.plus_milliseconds(amount),
            ChronoUnit::Seconds => self.plus_seconds(amount),
            ChronoUnit::Minutes => self.plus_seconds(amount.saturating_mul(60)),
            ChronoUnit::Hours => self.plus_seconds(amount.saturating_mul(3_600)),
            ChronoUnit::HalfDays => self.plus_seconds(amount.saturating_mul(43_200)),
            ChronoUnit::Days => panic!("ChronoUnit::Days not supported for Instant::add_to (no calendar context)"),
            ChronoUnit::Weeks => panic!("ChronoUnit::Weeks not supported for Instant::add_to (no calendar context)"),
            ChronoUnit::Months => panic!("ChronoUnit::Months not supported for Instant::add_to (no calendar context)"),
            ChronoUnit::Years => panic!("ChronoUnit::Years not supported for Instant::add_to (no calendar context)"),
        }
    }
}

impl ChronoAdd for LocalDate {
    fn cu_add(self, unit: ChronoUnit, amount: i64) -> Self {
        match unit {
            ChronoUnit::Days => self.plus_days(amount),
            ChronoUnit::Weeks => self.plus_weeks(amount),
            ChronoUnit::Months => self.plus_months(amount as i32),
            ChronoUnit::Years => self.plus_years(amount),
            _ => panic!("{} not supported for LocalDate::add_to", unit),
        }
    }
}

impl ChronoAdd for LocalTime {
    fn cu_add(self, unit: ChronoUnit, amount: i64) -> Self {
        match unit {
            ChronoUnit::Nanos => self.plus_nanoseconds(amount),
            ChronoUnit::Millis => self.plus_milliseconds(amount),
            ChronoUnit::Seconds => self.plus_seconds(amount),
            ChronoUnit::Minutes => self.plus_minutes(amount),
            ChronoUnit::Hours => self.plus_hours(amount),
            ChronoUnit::HalfDays => self.plus_hours(amount.saturating_mul(12)),
            _ => panic!("{} not supported for LocalTime::add_to", unit),
        }
    }
}

impl ChronoAdd for LocalDateTime {
    fn cu_add(self, unit: ChronoUnit, amount: i64) -> Self {
        match unit {
            ChronoUnit::Nanos => self.plus_nanoseconds(amount),
            ChronoUnit::Millis => self.plus_milliseconds(amount),
            ChronoUnit::Seconds => self.plus_seconds(amount),
            ChronoUnit::Minutes => self.plus_minutes(amount),
            ChronoUnit::Hours => self.plus_hours(amount),
            ChronoUnit::HalfDays => self.plus_hours(amount.saturating_mul(12)),
            ChronoUnit::Days => self.plus_days(amount),
            ChronoUnit::Weeks => self.plus_weeks(amount),
            ChronoUnit::Months => self.plus_months(amount as i32),
            ChronoUnit::Years => self.plus_years(amount),
        }
    }
}

impl fmt::Display for ChronoUnit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            ChronoUnit::Nanos => "NANOS",
            ChronoUnit::Millis => "MILLIS",
            ChronoUnit::Seconds => "SECONDS",
            ChronoUnit::Minutes => "MINUTES",
            ChronoUnit::Hours => "HOURS",
            ChronoUnit::HalfDays => "HALF_DAYS",
            ChronoUnit::Days => "DAYS",
            ChronoUnit::Weeks => "WEEKS",
            ChronoUnit::Months => "MONTHS",
            ChronoUnit::Years => "YEARS",
        };
        write!(f, "{}", s)
    }
}
