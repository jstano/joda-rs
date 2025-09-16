use joda_rs::{Duration, Instant};

#[test]
fn now_is_monotonic_and_between_non_negative() {
    let a = Instant::now();
    let b = Instant::now();
    let d1 = Duration::between(a, b);
    // Non-negative
    assert!(d1.inner().is_positive() || d1.inner().is_zero());

    // After a small busy wait, the measured duration should be > 0 and small (< 1s)
    let start = Instant::now();
    let mut x = 0u64;
    for _ in 0..100_000 { x = x.wrapping_add(1); }
    let end = Instant::now();
    let d2 = Duration::between(start, end);
    assert!(d2.inner().is_positive());
    assert!(d2 <= Duration::of_seconds(1));
    // Use x to avoid optimization
    assert!(x > 0 || x == 0);
}
