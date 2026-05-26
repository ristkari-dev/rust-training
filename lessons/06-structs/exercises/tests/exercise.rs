use structs_exercises::{Counter, Rectangle};

// Warm-up: Counter

#[test]
fn warmup_new_starts_at_zero() {
    assert_eq!(Counter::new().value(), 0);
}

#[test]
fn warmup_increment_once() {
    let mut c = Counter::new();
    c.increment();
    assert_eq!(c.value(), 1);
}

#[test]
fn warmup_increment_thrice() {
    let mut c = Counter::new();
    c.increment();
    c.increment();
    c.increment();
    assert_eq!(c.value(), 3);
}

#[test]
fn warmup_two_counters_are_independent() {
    let mut a = Counter::new();
    let mut b = Counter::new();
    a.increment();
    a.increment();
    b.increment();
    assert_eq!(a.value(), 2);
    assert_eq!(b.value(), 1);
}

// Main: Rectangle

#[test]
fn main_new_sets_fields() {
    let r = Rectangle::new(3, 5);
    assert_eq!(r.width, 3);
    assert_eq!(r.height, 5);
}

#[test]
fn main_area() {
    assert_eq!(Rectangle::new(3, 5).area(), 15);
}

#[test]
fn main_is_square_true() {
    assert!(Rectangle::new(7, 7).is_square());
}

#[test]
fn main_is_square_false() {
    assert!(!Rectangle::new(3, 5).is_square());
}
