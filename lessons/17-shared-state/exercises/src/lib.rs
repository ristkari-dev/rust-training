//! Lesson 17 — exercises.
//!
//! Implement `locked_increment` (warm-up) and `concurrent_counter`
//! (main) so that `cargo test --manifest-path
//! lessons/17-shared-state/exercises/Cargo.toml` passes. You'll need to
//! add `use` statements (e.g. `std::sync::Arc`, `std::thread`) for the
//! main exercise. The tests live in `tests/exercise.rs`.

use std::sync::Mutex;

#[must_use]
pub fn locked_increment(_m: &Mutex<i32>, _by: i32) -> i32 {
    todo!("lock the mutex, add `by` to the value, and return the new value")
}

#[must_use]
pub fn concurrent_counter(_threads: usize, _per_thread: usize) -> usize {
    todo!(
        "share an Arc<Mutex<usize>> across `threads` threads; each increments `per_thread` times; join all and return the final count"
    )
}
