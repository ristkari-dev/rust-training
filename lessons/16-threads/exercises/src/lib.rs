//! Lesson 16 — exercises.
//!
//! Implement `double_in_thread` (warm-up) and `parallel_sum_of_squares`
//! (main) so that `cargo test --manifest-path
//! lessons/16-threads/exercises/Cargo.toml` passes. You'll need to add
//! `use` statements (e.g. `std::thread`, `std::sync::mpsc`) as you go.
//! The tests live in `tests/exercise.rs`.

#[must_use]
pub fn double_in_thread(_n: i32) -> i32 {
    todo!("spawn a thread that doubles n, then join it and return the result")
}

#[must_use]
pub fn parallel_sum_of_squares(_values: Vec<i32>) -> i32 {
    todo!("spawn a thread per value to square it, send results over an mpsc channel, and sum them")
}
