//! Lesson 20 — exercises.
//!
//! Implement `abs_via_c` (warm-up) and `add_in_rust` (main) so that
//! `cargo test --manifest-path lessons/20-ffi/exercises/Cargo.toml`
//! passes. The tests live in `tests/exercise.rs`.

use core::ffi::c_int;

#[must_use]
pub fn abs_via_c(_n: i32) -> i32 {
    todo!(
        "declare the C `abs` function in an `unsafe extern \"C\"` block, then call it in an `unsafe` block and return its result"
    )
}

#[unsafe(no_mangle)]
pub extern "C" fn add_in_rust(_a: c_int, _b: c_int) -> c_int {
    todo!("add the two C integers and return the result")
}
