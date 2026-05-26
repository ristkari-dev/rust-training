use pattern_matching_exercises::{Light, next, safe_divide};

// Warm-up: safe_divide

#[test]
fn warmup_typical() {
    assert_eq!(safe_divide(10, 2), Some(5));
}

#[test]
fn warmup_by_zero() {
    assert_eq!(safe_divide(10, 0), None);
}

#[test]
fn warmup_zero_dividend() {
    assert_eq!(safe_divide(0, 5), Some(0));
}

#[test]
fn warmup_negative() {
    assert_eq!(safe_divide(-10, 2), Some(-5));
}

// Main: next (traffic light)

#[test]
fn main_red_to_green() {
    assert_eq!(next(Light::Red), Light::Green);
}

#[test]
fn main_green_to_yellow() {
    assert_eq!(next(Light::Green), Light::Yellow);
}

#[test]
fn main_yellow_to_red() {
    assert_eq!(next(Light::Yellow), Light::Red);
}

#[test]
fn main_cycle_closes() {
    assert_eq!(next(next(next(Light::Red))), Light::Red);
}
