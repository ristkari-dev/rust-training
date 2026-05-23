//! Lesson 03 — exercises.
//!
//! Implement `classify` (warm-up) and `count_digits` (main) so that
//! `cargo test --manifest-path lessons/03-control-flow/exercises/Cargo.toml`
//! passes. The tests live in `tests/exercise.rs`.

#[must_use]
pub fn classify(_n: i32) -> &'static str {
    todo!("return \"negative\" / \"zero\" / \"positive\" using an if-as-expression")
}

#[must_use]
pub fn count_digits(_n: u32) -> u32 {
    todo!("count the number of decimal digits; treat 0 as having 1 digit")
}
