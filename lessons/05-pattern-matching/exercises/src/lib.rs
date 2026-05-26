//! Lesson 05 — exercises.
//!
//! Implement `safe_divide` (warm-up) and `next` (main) so that
//! `cargo test --manifest-path lessons/05-pattern-matching/exercises/Cargo.toml`
//! passes. The tests live in `tests/exercise.rs`.

#[derive(Debug, PartialEq, Eq)]
pub enum Light {
    Red,
    Yellow,
    Green,
}

#[must_use]
pub fn safe_divide(_a: i32, _b: i32) -> Option<i32> {
    todo!("return None when b == 0, otherwise Some(a / b)")
}

#[must_use]
pub fn next(_light: Light) -> Light {
    todo!("return Red->Green, Green->Yellow, Yellow->Red")
}
