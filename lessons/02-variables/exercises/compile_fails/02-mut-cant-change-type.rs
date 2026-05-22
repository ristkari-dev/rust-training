// Compile-fail exercise: this file MUST NOT compile until you fix it.
//
// `mut` lets you change a binding's VALUE, but not its TYPE. The line
// below tries to reassign `x` from an integer to a string slice. Read
// the rustc error: it will name the original type that Rust inferred
// and point at the offending assignment.
//
// Hint: the fix is NOT to add `mut` (we already have it). The fix is
// to use shadowing — replace the second line so it declares a NEW
// binding called `x` with `let`. The new `x` can have a different type
// because it's a separate binding that happens to reuse the name.

fn main() {
    let mut x = 5;
    x = "hello";
    println!("{x}");
}
