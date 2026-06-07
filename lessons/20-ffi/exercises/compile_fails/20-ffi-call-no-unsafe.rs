// Compile-fail exercise: this file MUST NOT compile until you fix it.
//
// Calling a foreign (C) function is one of the operations the compiler
// can't verify: it has no way to check that the C side matches the
// declared signature or behaves well. So an FFI call — like dereferencing
// a raw pointer — is only allowed inside an `unsafe` block. Here the call
// to `abs` is in ordinary safe code, so the compiler rejects it.
//
// rustc reports E0133: "call to unsafe function `abs` is unsafe and
// requires unsafe block".
//
// The fix: wrap the call in an `unsafe { }` block (in real code, behind a
// safe wrapper with a `// SAFETY:` comment).
//
// Hint: change `let result = abs(-5);` to `let result = unsafe { abs(-5) };`.

use core::ffi::c_int;

unsafe extern "C" {
    fn abs(input: c_int) -> c_int;
}

fn main() {
    let result = abs(-5);
    println!("{result}");
}
