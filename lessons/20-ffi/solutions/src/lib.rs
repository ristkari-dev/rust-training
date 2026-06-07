//! Lesson 20 — reference solutions.

use core::ffi::c_int;

unsafe extern "C" {
    fn abs(input: c_int) -> c_int;
}

#[must_use]
pub fn abs_via_c(n: i32) -> i32 {
    // SAFETY: `abs` is a pure libc function with no preconditions for any
    // `c_int` argument (except `i32::MIN`, which is not used here).
    unsafe { abs(n) }
}

#[unsafe(no_mangle)]
pub extern "C" fn add_in_rust(a: c_int, b: c_int) -> c_int {
    a + b
}
