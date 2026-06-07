use ffi_solutions::{abs_via_c, add_in_rust};

// Warm-up: abs_via_c (declare + call libc `abs`)

#[test]
fn warmup_abs_negative() {
    assert_eq!(abs_via_c(-5), 5);
}

#[test]
fn warmup_abs_positive() {
    assert_eq!(abs_via_c(7), 7);
}

#[test]
fn warmup_abs_zero() {
    assert_eq!(abs_via_c(0), 0);
}

#[test]
fn warmup_abs_large() {
    assert_eq!(abs_via_c(-1000), 1000);
}

// Main: add_in_rust (exported with the C ABI)

#[test]
fn main_add_basic() {
    assert_eq!(add_in_rust(2, 3), 5);
}

#[test]
fn main_add_zero() {
    assert_eq!(add_in_rust(0, 0), 0);
}

#[test]
fn main_add_negative() {
    assert_eq!(add_in_rust(-4, 10), 6);
}

#[test]
fn main_add_commutes() {
    assert_eq!(add_in_rust(8, -8), 0);
}
