//! Lesson 02 — exercises.
//!
//! Implement `fahrenheit_to_celsius` (warm-up) and `compound_interest`
//! (main) so that `cargo test --manifest-path
//! lessons/02-variables/exercises/Cargo.toml` passes. The tests live in
//! `tests/exercise.rs`.

#[must_use]
pub fn fahrenheit_to_celsius(_f: f64) -> f64 {
    todo!("convert Fahrenheit to Celsius using (f - 32) * 5 / 9")
}

#[must_use]
pub fn compound_interest(_principal: f64, _rate_percent: f64, _years: u32) -> f64 {
    todo!("return principal * (1 + rate_percent/100)^years")
}
