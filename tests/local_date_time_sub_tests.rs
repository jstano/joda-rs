use joda_rs::{LocalDateTime, Duration};

#[test]
fn subtraction_returns_duration() {
    let a = LocalDateTime::of(2020, 1, 1, 0, 0, 1);
    let b = LocalDateTime::of(2020, 1, 1, 0, 0, 0);
    let d = a - b;
    assert_eq!(d, Duration::of_seconds(1));

    let d2 = b - a;
    assert!(d2.inner().is_negative());
    // Compare absolute value by flipping sign via underlying time::Duration
    let abs_ns = d2.inner().abs();
    assert_eq!(joda_rs::Duration::from(abs_ns), Duration::of_seconds(1));

    let z = a - a;
    assert!(z.inner().is_zero());
}
