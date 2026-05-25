//! Lesson 04 — exercises.
//!
//! Implement `divmod` (warm-up) and `join_with_dashes` (main) so that
//! `cargo test --manifest-path lessons/04-compound-types/exercises/Cargo.toml`
//! passes. The tests live in `tests/exercise.rs`.

#[must_use]
pub fn divmod(_a: u32, _b: u32) -> (u32, u32) {
    todo!("return (quotient, remainder)")
}

#[must_use]
pub fn join_with_dashes(_words: &[&str]) -> String {
    todo!("join words with '-' between them; return \"\" for empty input")
}
