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
