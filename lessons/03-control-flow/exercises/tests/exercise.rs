use control_flow_exercises::{classify, count_digits};

// Warm-up: classify

#[test]
fn classify_negative() {
    assert_eq!(classify(-42), "negative");
}

#[test]
fn classify_zero() {
    assert_eq!(classify(0), "zero");
}

#[test]
fn classify_positive() {
    assert_eq!(classify(7), "positive");
}

// Main: count_digits

#[test]
fn count_digits_zero() {
    assert_eq!(count_digits(0), 1);
}

#[test]
fn count_digits_single() {
    assert_eq!(count_digits(7), 1);
}

#[test]
fn count_digits_two() {
    assert_eq!(count_digits(42), 2);
}

#[test]
fn count_digits_three() {
    assert_eq!(count_digits(100), 3);
}

#[test]
fn count_digits_u32_max() {
    assert_eq!(count_digits(4_294_967_295), 10);
}
