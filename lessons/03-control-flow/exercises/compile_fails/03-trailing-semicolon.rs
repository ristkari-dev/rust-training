// Compile-fail exercise: this file MUST NOT compile until you fix it.
//
// In Rust, a function's body is a block; the block's TAIL EXPRESSION
// (the last expression with no trailing semicolon) is what the function
// returns. A semicolon at the end turns the expression into a STATEMENT,
// which produces the unit type `()` instead of a value. The function
// below declares it returns `i32` but its body returns `()` — the
// compiler will tell you exactly that.
//
// Hint: read the rustc error. It will mention "expected `i32`, found `()`"
// and point at the closing brace of `double`. The fix is to remove ONE
// character.

fn double(n: i32) -> i32 {
    n * 2;
}

fn main() {
    let d = double(7);
    println!("{d}");
}
