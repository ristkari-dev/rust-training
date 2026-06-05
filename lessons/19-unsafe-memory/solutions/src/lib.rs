//! Lesson 19 — reference solutions.

#[must_use]
pub fn read_doubled(n: i32) -> i32 {
    let ptr = &raw const n;
    // SAFETY: `ptr` was just created from the live local `n`, so it is
    // non-null, aligned, and points to an initialized `i32`.
    let value = unsafe { *ptr };
    value * 2
}

/// Sum `len` consecutive `i32`s starting at `ptr`.
///
/// # Safety
///
/// `ptr` must be valid for reads of `len` consecutive `i32` values, and
/// that memory must stay valid for the duration of the call.
#[must_use]
pub unsafe fn sum_raw(ptr: *const i32, len: usize) -> i32 {
    let mut total = 0;
    for i in 0..len {
        // SAFETY: the caller guarantees `ptr` is valid for `len` reads,
        // so `ptr.add(i)` for `i < len` is in bounds and readable.
        total += unsafe { *ptr.add(i) };
    }
    total
}
