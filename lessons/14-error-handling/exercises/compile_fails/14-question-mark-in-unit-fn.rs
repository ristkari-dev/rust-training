// Compile-fail exercise: this file MUST NOT compile until you fix it.
//
// The `?` operator only works inside a function whose return type can
// represent failure — a `Result` or an `Option`. On an `Err`, `?` needs
// to *return that error from the function*; a function returning `()`
// has nowhere to return it to.
//
// rustc reports E0277: "the `?` operator can only be used in a function
// that returns `Result` or `Option`".
//
// The fix: give the function a fallible return type. Have it return
// `Result<(), std::num::ParseIntError>`, `?`-parse, do the work, then
// return `Ok(())`.
//
// Hint: change the signature to
//     fn parse_and_double(input: &str) -> Result<(), std::num::ParseIntError>
// and end the body with `Ok(())`.

fn parse_and_double(input: &str) {
    let n: i32 = input.parse()?;
    println!("{}", n * 2);
}

fn main() {
    parse_and_double("21");
}
