#![allow(clippy::float_cmp)]

use variables_exercises::{compound_interest, fahrenheit_to_celsius};

// Warm-up: fahrenheit_to_celsius

#[test]
fn warmup_freezing() {
    assert_eq!(fahrenheit_to_celsius(32.0), 0.0);
}

#[test]
fn warmup_boiling() {
    assert_eq!(fahrenheit_to_celsius(212.0), 100.0);
}

#[test]
fn warmup_ten_celsius() {
    assert_eq!(fahrenheit_to_celsius(50.0), 10.0);
}

// Main: compound_interest

#[test]
fn main_zero_principal_grows_to_zero() {
    assert_eq!(compound_interest(0.0, 50.0, 10), 0.0);
}

#[test]
fn main_zero_rate_returns_principal() {
    assert_eq!(compound_interest(1000.0, 0.0, 5), 1000.0);
}

#[test]
fn main_fifty_percent_two_years() {
    assert_eq!(compound_interest(100.0, 50.0, 2), 225.0);
}

#[test]
fn main_twentyfive_percent_two_years() {
    assert_eq!(compound_interest(200.0, 25.0, 2), 312.5);
}
