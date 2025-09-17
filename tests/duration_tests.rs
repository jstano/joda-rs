use joda_rs::{Duration, Instant};

#[test]
fn constructors_and_arithmetic() {
    let s = Duration::of_seconds(2);
    let ms = Duration::of_millis(500);
    let ns = Duration::of_nanos(1_000_000);

    // 500ms + 1_000_000ns == 501ms
    let ms_plus_ns = ms.plus(ns);
    let expected = Duration::of_millis(501);
    assert_eq!(ms_plus_ns, expected);

    // (2s + 500ms) - 1_000_000ns == 2.499s
    let total = s.plus(ms).minus(ns);
    assert_eq!(total, Duration::of_millis(2499));
}

#[test]
fn between_and_monotonic_non_negative() {
    let start = Instant::now();
    // do a tiny busy wait to ensure some time passes
    let mut x = 0u64;
    for _ in 0..50_000 { x = x.wrapping_add(1); }
    let end = Instant::now();

    let d = Duration::between(start, end);
    // Between should be >= 0
    assert!(d.inner().is_positive() || d.inner().is_zero());

    // Adding between(start,end) to start should not be after end by more than small epsilon,
    // but since we don't have direct add to Instant, just sanity check it's not absurdly large.
    // Expect less than 1 second for such a short loop on CI environments.
    assert!(d <= Duration::of_seconds(1));
}

#[test]
fn to_whole_units_truncate_toward_zero() {
    // 2 days + 12 hours = 2.5 days
    let d_days_pos = Duration::of_seconds(2 * 86_400).plus_hours(12);
    assert_eq!(d_days_pos.to_days(), 2);

    // -2 days - 12 hours = -2.5 days
    let d_days_neg = Duration::of_seconds(-2 * 86_400).minus_hours(12);
    assert_eq!(d_days_neg.to_days(), -2);

    // 3h 59m 59s -> to_hours = 3
    let d_hours_pos = Duration::of_seconds(3 * 3600 + 59 * 60 + 59);
    assert_eq!(d_hours_pos.to_hours(), 3);

    // -3h -59m -59s -> to_hours = -3
    let d_hours_neg = Duration::of_seconds(-(3 * 3600 + 59 * 60 + 59));
    assert_eq!(d_hours_neg.to_hours(), -3);

    // 2m 59s -> to_minutes = 2
    let d_minutes_pos = Duration::of_seconds(2 * 60 + 59);
    assert_eq!(d_minutes_pos.to_minutes(), 2);

    // -2m -59s -> to_minutes = -2
    let d_minutes_neg = Duration::of_seconds(-(2 * 60 + 59));
    assert_eq!(d_minutes_neg.to_minutes(), -2);

    // 1s 999ms -> to_seconds = 1
    let d_seconds_pos = Duration::of_seconds(1).plus_millis(999);
    assert_eq!(d_seconds_pos.to_seconds(), 1);

    // -1s -999ms -> to_seconds = -1
    let d_seconds_neg = Duration::of_seconds(-1).minus_millis(999);
    assert_eq!(d_seconds_neg.to_seconds(), -1);
}

#[test]
fn to_millis_and_nanos_behavior() {
    // 1234ms + 567ns -> to_millis = 1234, to_nanos = 1_234_000_567
    let d = Duration::of_millis(1234).plus_nanos(567);
    assert_eq!(d.to_millis(), 1234);
    assert_eq!(d.to_nanos(), 1_234_000_567);

    // Negative case: -1234ms - 567ns -> to_millis = -1234, to_nanos = -1_234_000_567
    let d_neg = Duration::of_millis(-1234).minus_nanos(567);
    assert_eq!(d_neg.to_millis(), -1234);
    assert_eq!(d_neg.to_nanos(), -1_234_000_567);
}

#[test]
fn sign_checks() {
    let neg = Duration::of_seconds(-1);
    let zero = Duration::of_seconds(0);
    let pos = Duration::of_seconds(1);

    assert!(neg.is_negative());
    assert!(!neg.is_zero());
    assert!(!neg.is_positive());

    assert!(!zero.is_negative());
    assert!(zero.is_zero());
    assert!(!zero.is_positive());

    assert!(!pos.is_negative());
    assert!(!pos.is_zero());
    assert!(pos.is_positive());
}


#[test]
fn unit_add_sub_basic() {
    // Days
    assert_eq!(Duration::of_seconds(0).plus_days(1), Duration::of_seconds(86_400));
    assert_eq!(Duration::of_seconds(86_400).minus_days(1), Duration::of_seconds(0));

    // Hours
    assert_eq!(Duration::of_seconds(0).plus_hours(2), Duration::of_seconds(2 * 3_600));
    assert_eq!(Duration::of_seconds(2 * 3_600).minus_hours(2), Duration::of_seconds(0));

    // Minutes
    assert_eq!(Duration::of_seconds(0).plus_minutes(3), Duration::of_seconds(3 * 60));
    assert_eq!(Duration::of_seconds(3 * 60).minus_minutes(3), Duration::of_seconds(0));

    // Seconds
    assert_eq!(Duration::of_seconds(10).plus_seconds(5), Duration::of_seconds(15));
    assert_eq!(Duration::of_seconds(10).minus_seconds(5), Duration::of_seconds(5));

    // Millis -> verify through to_nanos to avoid rounding
    let d = Duration::of_seconds(0).plus_millis(123).minus_millis(23);
    assert_eq!(d.to_nanos(), 100_i128 * 1_000_000);

    // Nanos
    let d2 = Duration::of_seconds(0).plus_nanos(1_500).minus_nanos(500);
    assert_eq!(d2.to_nanos(), 1_000);
}

#[test]
fn saturating_add_sub_boundaries() {
    // Overflow on add should saturate and be idempotent
    let near_max = Duration::of_seconds(i64::MAX - 1);
    let grown = near_max.plus_days(1);
    assert!(grown >= near_max);
    let grown_again = grown.plus_days(10_000);
    assert_eq!(grown_again, grown, "addition at upper bound should saturate");

    // Underflow on sub should saturate and be idempotent
    let near_min = Duration::of_seconds(i64::MIN + 1);
    let reduced = near_min.minus_days(1);
    assert!(reduced <= near_min);
    let reduced_again = reduced.minus_days(10_000);
    assert_eq!(reduced_again, reduced, "subtraction at lower bound should saturate");
}

#[test]
fn abs_behavior() {
    let five = Duration::of_seconds(5);
    let neg_five = Duration::of_seconds(-5);
    assert_eq!(neg_five.abs(), five);
    assert_eq!(five.abs(), five);
    assert_eq!(Duration::of_seconds(0).abs(), Duration::of_seconds(0));

    // Extreme negative should produce a non-negative result (saturating behavior)
    let extreme_neg = Duration::of_seconds(i64::MIN);
    let abs_extreme = extreme_neg.abs();
    assert!(abs_extreme.is_positive() || abs_extreme.is_zero());
}

#[test]
fn into_time_duration_converts_correctly() {
    let d = Duration::of_seconds(5)
        .plus_millis(123)
        .plus_nanos(456);
    let t: time::Duration = d.into();
    assert_eq!(t, *d.inner());

    // Also test a negative case
    let d_neg = Duration::of_seconds(-2).minus_millis(250);
    let t_neg: time::Duration = d_neg.into();
    assert_eq!(t_neg, *d_neg.inner());
}
