//! Lesson 07 — exercises.
//!
//! Implement `append_excl` (warm-up) and `swap_and_join` (main) so
//! that `cargo test --manifest-path
//! lessons/07-ownership/exercises/Cargo.toml` passes. The tests live
//! in `tests/exercise.rs`.

#[must_use]
pub fn append_excl(_s: String) -> String {
    todo!("take ownership of s, push '!' onto the end, return it")
}

#[must_use]
pub fn swap_and_join(_a: String, _b: String) -> String {
    todo!(
        "return b followed by a space followed by a, e.g. swap_and_join(\"hello\", \"world\") -> \"world hello\""
    )
}
