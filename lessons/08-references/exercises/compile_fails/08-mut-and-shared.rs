// Compile-fail exercise: this file MUST NOT compile until you fix it.
//
// Rust's borrowing rules: at any moment, for any value, you can have
// either
//   - any number of SHARED references (`&T`), OR
//   - exactly one MUTABLE reference (`&mut T`).
//
// You CANNOT have a mutable reference and shared references to the
// same value at the same time. The borrow checker enforces this — it
// prevents data races by construction.
//
// The function below tries to take a `&mut s` while a shared `&s` is
// still in use. rustc will say "cannot borrow `s` as mutable because
// it is also borrowed as immutable."
//
// Hint: read the rustc error. The fix is to drop the shared borrow
// before taking the mutable one — for example, by moving the
// `println!` of `r1` to BEFORE the line that takes `&mut s`. Once
// `r1` is no longer used, its borrow ends, and the mutable borrow
// becomes legal.

fn main() {
    let mut s = String::from("hello");
    let r1 = &s;
    let r2 = &mut s;
    r2.push_str(" world");
    println!("{r1}");
}
