//! Lesson 19 — exercises.
//!
//! Implement `read_doubled` (warm-up) and `sum_raw` (main) so that
//! `cargo test --manifest-path
//! lessons/19-unsafe-memory/exercises/Cargo.toml` passes. The tests live
//! in `tests/exercise.rs`.

#[must_use]
pub fn read_doubled(_n: i32) -> i32 {
    todo!(
        "make a raw `*const i32` to `n` with `&raw const n`, dereference it in an `unsafe` block, and double the value"
    )
}

/// Sum `len` consecutive `i32`s starting at `ptr`.
///
/// # Safety
///
/// `ptr` must be valid for reads of `len` consecutive `i32` values, and
/// that memory must stay valid for the duration of the call.
#[must_use]
pub unsafe fn sum_raw(_ptr: *const i32, _len: usize) -> i32 {
    todo!(
        "sum `len` values starting at `ptr` using pointer arithmetic (`*ptr.add(i)`) in an `unsafe` block"
    )
}
