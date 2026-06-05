use unsafe_memory_solutions::{read_doubled, sum_raw};

// Warm-up: read_doubled (raw pointer deref in a safe fn)

#[test]
fn warmup_basic() {
    assert_eq!(read_doubled(21), 42);
}

#[test]
fn warmup_zero() {
    assert_eq!(read_doubled(0), 0);
}

#[test]
fn warmup_negative() {
    assert_eq!(read_doubled(-5), -10);
}

#[test]
fn warmup_large() {
    assert_eq!(read_doubled(1000), 2000);
}

// Main: sum_raw (unsafe fn + pointer arithmetic)

#[test]
fn main_sum_empty() {
    let v: [i32; 0] = [];
    // SAFETY: `v.as_ptr()` is valid for `v.len()` (0) reads.
    assert_eq!(unsafe { sum_raw(v.as_ptr(), v.len()) }, 0);
}

#[test]
fn main_sum_one() {
    let v = [7];
    // SAFETY: `v.as_ptr()` is valid for `v.len()` reads.
    assert_eq!(unsafe { sum_raw(v.as_ptr(), v.len()) }, 7);
}

#[test]
fn main_sum_many() {
    let v = [1, 2, 3, 4];
    // SAFETY: `v.as_ptr()` is valid for `v.len()` reads.
    assert_eq!(unsafe { sum_raw(v.as_ptr(), v.len()) }, 10);
}

#[test]
fn main_sum_negatives() {
    let v = [-2, 5, 10];
    // SAFETY: `v.as_ptr()` is valid for `v.len()` reads.
    assert_eq!(unsafe { sum_raw(v.as_ptr(), v.len()) }, 13);
}
