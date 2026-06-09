// Compile-fail exercise: this file MUST NOT compile until you fix it.
//
// The `Read` and `Write` methods (`read_to_end`, `read_to_string`,
// `write_all`, ...) are defined on TRAITS. To call a trait's methods,
// the trait must be IN SCOPE — you have to `use` it. Here `&[u8]` does
// implement `Read`, but without `use std::io::Read;` the method
// `read_to_end` isn't visible, so the call fails.
//
// rustc reports E0599 ("no method named `read_to_end` ...") and helpfully
// notes that the `Read` trait is implemented but not in scope, suggesting
// the exact import.
//
// The fix: bring the trait into scope.
//
// Hint: add `use std::io::Read;` at the top of the file.

fn main() {
    let data: &[u8] = b"hello";
    let mut buf = Vec::new();
    let n = data.read_to_end(&mut buf);
    println!("{n:?}");
}
