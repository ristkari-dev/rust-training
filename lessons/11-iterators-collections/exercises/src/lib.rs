//! Lesson 11 — exercises.
//!
//! Implement `evens_squared` (warm-up) and `word_frequencies` +
//! `most_frequent` (main) so that `cargo test --manifest-path
//! lessons/11-iterators-collections/exercises/Cargo.toml` passes. The
//! tests live in `tests/exercise.rs`.

use std::collections::HashMap;

#[must_use]
pub fn evens_squared(_nums: &[i32]) -> Vec<i32> {
    todo!("filter the even numbers, square each, and collect into a Vec")
}

#[must_use]
pub fn word_frequencies(_text: &str) -> HashMap<String, usize> {
    todo!("count occurrences of each whitespace-separated word")
}

#[must_use]
pub fn most_frequent(_text: &str) -> Option<String> {
    todo!("return the word with the highest count, or None if empty")
}
