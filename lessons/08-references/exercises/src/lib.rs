//! Lesson 08 — exercises.
//!
//! Implement `wrap_in_quotes` (warm-up) and `merge_into` (main) so
//! that `cargo test --manifest-path
//! lessons/08-references/exercises/Cargo.toml` passes. The tests live
//! in `tests/exercise.rs`.

#[must_use]
pub fn wrap_in_quotes(_s: &str) -> String {
    todo!(
        "return a String containing s wrapped in double quotes, e.g. wrap_in_quotes(\"hi\") -> \"\\\"hi\\\"\""
    )
}

#[allow(clippy::ptr_arg)]
pub fn merge_into(_target: &mut String, _parts: &[&str], _separator: &str) {
    todo!("append parts joined by separator to target; do nothing if parts is empty")
}
