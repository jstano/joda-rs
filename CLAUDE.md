# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

`joda_rs` is a Rust wrapper around the `time` and `time-tz` crates that provides a Java java.time/Joda-Time style API. It offers friendly types and convenience constructors for working with dates, times, and time zones.

## Building and Testing

```bash
# Build the project
cargo build

# Run all tests
cargo test

# Run tests for a specific module (e.g., local_date)
cargo test --test local_date_tests

# Run a single test by name
cargo test test_name

# Build documentation
cargo doc --open

# Check formatting
cargo fmt --check

# Lint with clippy
cargo clippy

# Package for publication
cargo package
```

## Architecture

### Wrapper Pattern
All primary types (`LocalDate`, `LocalTime`, `LocalDateTime`, `OffsetDateTime`, `ZonedDateTime`, etc.) are newtype wrappers around `time` crate types. Each wrapper struct contains a single field holding the underlying `time` type (e.g., `LocalDate(time::Date)`).

The wrappers expose:
- An `inner()` method to access the underlying `time` type
- Joda-Time/java.time style constructors (`of()`, `new()`)
- Now helpers (`now()`, `now_with_clock()`, `now_with_zone()`)
- Arithmetic and query methods matching java.time semantics

### Trait-Based Design
The `temporal` module defines traits that formalize the API contract:
- `Temporal`: Basic comparison operations (`is_before`, `is_after`, `is_on_or_before`, `is_on_or_after`)
- `TemporalInstant`: Epoch-based time methods
- `TemporalDate`: Date-specific operations (year, month, day queries and arithmetic)
- `TemporalTime`: Time-specific operations (hour, minute, second queries and arithmetic)
- `TemporalDateTime`: Combined date-time operations

These traits are implemented by the wrapper types to ensure consistency across the API.

### Month Arithmetic Clamping
Following java.time behavior, month and year arithmetic clamps to valid end-of-month values:
- `LocalDate::of(2020, 1, 31).plus_months(1)` → `2020-02-29` (leap year)
- `LocalDate::of(2019, 1, 31).plus_months(1)` → `2019-02-28` (non-leap year)
- `LocalDate::of(2020, 2, 29).plus_years(1)` → `2021-02-28` (clamped to valid Feb date)

This is intentional and matches Java's `java.time` behavior, not a bug.

### Re-exports
All primary types are re-exported at the crate root (`lib.rs`) for ergonomic imports. Users can write `use joda_rs::{LocalDate, LocalDateTime, ZoneId};` instead of importing from individual modules.

## Module Organization

- `src/lib.rs`: Module declarations and re-exports
- `src/local_date.rs`: Date without time-of-day
- `src/local_time.rs`: Time-of-day without date
- `src/local_date_time.rs`: Combined date and time, no timezone
- `src/offset_date_time.rs`: Date-time with UTC offset
- `src/zoned_date_time.rs`: Date-time with timezone (IANA database)
- `src/instant.rs`: Point in time (Unix timestamp)
- `src/duration.rs`: Time-based duration (hours, minutes, seconds)
- `src/period.rs`: Date-based period (years, months, days)
- `src/zone_id.rs`: Timezone identifier wrapper
- `src/zone_offset.rs`: UTC offset wrapper
- `src/clock.rs`: Clock abstraction (system and fixed clocks)
- `src/chrono_unit.rs`: Time units for arithmetic (Days, Weeks, Months, etc.)
- `src/temporal.rs`: Trait definitions
- `src/constants.rs`: Standard constants for date/time operations
- `src/day_of_week.rs`, `src/month.rs`, `src/year.rs`, `src/year_month.rs`, `src/month_day.rs`: Supporting calendar types

Tests are in `tests/` with one file per module (e.g., `local_date_tests.rs`, `zoned_date_time_tests.rs`).

## Key Implementation Details

### `inner()` Pattern
Every wrapper type exposes an `inner()` method that returns a reference to the underlying `time` crate type. Use this when you need to interoperate with code expecting raw `time` types.

### Parsing
Parsing uses ISO 8601 format by default. The `parse()` methods expect well-formed ISO strings (e.g., `"2025-09-15"` for dates).

### Feature Flag: `serde`
The optional `serde` feature enables serialization support. When adding serde support to types, use conditional compilation with `#[cfg(feature = "serde")]`.

### Error Handling
Many methods use `.expect()` for invalid inputs, following the philosophy that constructors like `of(year, month, day)` should panic on invalid data (similar to java.time). This is intentional for ergonomic constructor APIs.

## Publishing

Version is currently `0.1.3` (edition 2024). Update `Cargo.toml` version before publishing:
```bash
cargo package --allow-dirty  # preview the package
cargo publish
```
