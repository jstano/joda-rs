# joda-rs

A small, ergonomic wrapper around the Rust `time` and `time-tz` crates inspired by Java’s java.time/Joda-Time style API. It provides friendly types like LocalDate, LocalTime, LocalDateTime, OffsetDateTime, ZonedDateTime, Period, Duration, and more, with convenience constructors and methods reminiscent of the Joda/java.time experience.

- Crate: `joda_rs`
- License: Apache-2.0
- MSRV: same as your toolchain supporting edition 2024 (tested with recent stable)

## Features
- Simple constructors like `LocalDate::of(yyyy, mm, dd)` and `LocalTime::of(h, m, s)`.
- Now/clock helpers: `LocalDate::now()`, `LocalDateTime::now()`, `ZonedDateTime::now_utc()`, and `Clock` utilities.
- Arithmetic and queries: add/subtract days/weeks/months/years, clamp end-of-month behavior like java.time, day-of-week/month helpers, etc.
- Period and Duration types with intuitive constructors.
- Re-exports of the primary types at crate root for easy use: `use joda_rs::{LocalDate, LocalDateTime, ZonedDateTime, ZoneId, ZoneOffset, Duration, Period, ChronoUnit, …};`

Under the hood, this crate delegates to the excellent `time` and `time-tz` crates.

## Installation
Add to your Cargo.toml:

```toml
[dependencies]
joda_rs = "^0.1.1"
```

The crate enables useful features on `time` and `time-tz` (parsing, local-offset, tz database) by default via its own dependencies.

## Quick start

```rust
use joda_rs::{LocalDate, LocalTime, LocalDateTime, OffsetDateTime, ZonedDateTime};
use joda_rs::{ZoneId, ZoneOffset, Duration, Period, ChronoUnit};

fn main() {
    // Construct dates and times
    let d  = LocalDate::of(2025, 9, 15);
    let t  = LocalTime::of(13, 37, 42);
    let ldt = LocalDateTime::of(2025, 9, 15, 13, 37, 42);

    // Start of day and combining date/time
    let midnight = d.at_start_of_day();
    let at_time  = d.at_time(t);

    // Offsets and zones
    let utc = ZoneOffset::of_hours(0);
    let odt = OffsetDateTime::of(ldt, utc);
    let zdt = ZonedDateTime::of(ldt, ZoneId::UTC);

    // Now helpers
    let today       = LocalDate::now();
    let now_local   = LocalDateTime::now();
    let now_utc_zdt = ZonedDateTime::now_utc();

    // Periods and durations
    let p = Period::of(1, 2, 3);        // 1 year, 2 months, 3 days
    let five_sec = Duration::of_seconds(5);

    // Arithmetic (java.time-like semantics)
    let end_jan = LocalDate::of(2020, 1, 31);
    let feb_end = end_jan.plus_months(1); // -> 2020-02-29 (clamped)

    // ChronoUnit usage example (between, add)
    let added = ChronoUnit::Days.add_to(d, 10); // 2025-09-25
    let between_days = ChronoUnit::Days.between(d, added);

    // Queries
    let dow = d.day_of_week(); // Monday..Sunday
    let mon = d.month();       // Month enum

    // Parse
    let parsed = LocalDate::parse("2025-09-15");
    assert_eq!(parsed, d);

    // Comparisons
    assert!(d.is_on_or_before(added));
    assert!(added.is_after(d));

    // Use values to avoid warnings
    let _ = (midnight, at_time, odt, zdt, today, now_local, now_utc_zdt,
             p, five_sec, feb_end, between_days, dow, mon);
}
```

## Design notes
- API aims to be convenient and familiar to users of Java’s date/time libraries, while relying on Rust’s `time` crate internally.
- Types are re-exported at the crate root for succinct `use` statements.
- Some methods intentionally clamp to valid end-of-month values, similar to java.time behavior.

## License
Licensed under the Apache License, Version 2.0 (Apache-2.0). See the LICENSE file for details.

## Links
- Crate repository: https://github.com/jstano/joda-rs
- Depends on: https://crates.io/crates/time and https://crates.io/crates/time-tz
