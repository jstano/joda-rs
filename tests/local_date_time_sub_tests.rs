use joda_rs::{Duration, LocalDateTime};

#[test]
fn subtraction_returns_duration() {
    let a = LocalDateTime::of(2020, 1, 1, 0, 0, 1);
    let b = LocalDateTime::of(2020, 1, 1, 0, 0, 0);
    let d = a - b;
    assert_eq!(d, Duration::of_seconds(1));

    let d2 = b - a;
    assert!(d2.is_negative());
    // Compare absolute value by flipping sign via underlying time::Duration
    let abs_ns = d2.abs();
    assert_eq!(joda_rs::Duration::from(abs_ns), Duration::of_seconds(1));

    let z = a - a;
    assert!(z.is_zero());
}
