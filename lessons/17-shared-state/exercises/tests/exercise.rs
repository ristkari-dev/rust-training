use shared_state_exercises::{concurrent_counter, locked_increment};
use std::sync::Mutex;

// Warm-up: locked_increment (single-threaded Mutex)

#[test]
fn warmup_increment_basic() {
    let m = Mutex::new(10);
    assert_eq!(locked_increment(&m, 5), 15);
}

#[test]
fn warmup_increment_zero() {
    let m = Mutex::new(0);
    assert_eq!(locked_increment(&m, 0), 0);
}

#[test]
fn warmup_increment_negative() {
    let m = Mutex::new(3);
    assert_eq!(locked_increment(&m, -8), -5);
}

#[test]
fn warmup_increment_twice() {
    let m = Mutex::new(0);
    assert_eq!(locked_increment(&m, 2), 2);
    assert_eq!(locked_increment(&m, 3), 5);
}

// Main: concurrent_counter (Arc<Mutex> shared counter)

#[test]
fn main_counter_zero_threads() {
    assert_eq!(concurrent_counter(0, 100), 0);
}

#[test]
fn main_counter_single_thread() {
    assert_eq!(concurrent_counter(1, 50), 50);
}

#[test]
fn main_counter_many() {
    assert_eq!(concurrent_counter(8, 1000), 8000);
}

#[test]
fn main_counter_no_increments() {
    assert_eq!(concurrent_counter(10, 0), 0);
}
