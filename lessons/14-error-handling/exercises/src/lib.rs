//! Lesson 14 — exercises.
//!
//! Implement `sum_fields` (warm-up) and `parse_setting` (main) so that
//! `cargo test --manifest-path
//! lessons/14-error-handling/exercises/Cargo.toml` passes. The
//! `ConfigError` enum is given as a worked example of a `thiserror`
//! custom error type. The tests live in `tests/exercise.rs`.

use thiserror::Error;

pub fn sum_fields(_a: &str, _b: &str) -> Result<i32, std::num::ParseIntError> {
    todo!("parse both fields with `?` and return their sum")
}

#[derive(Debug, Error, PartialEq)]
pub enum ConfigError {
    #[error("missing '=' in setting")]
    MissingEquals,
    #[error("key is empty")]
    EmptyKey,
    #[error("value is not a valid integer: {0}")]
    BadValue(#[from] std::num::ParseIntError),
}

pub fn parse_setting(_input: &str) -> Result<(String, i32), ConfigError> {
    todo!("split on '=', validate the key, then parse the value with `?`")
}
