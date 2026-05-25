// Compile-fail exercise: this file MUST NOT compile until you fix it.
//
// String literals like "hello" are NOT `String` — they're `&'static str`,
// slices into the program binary itself. The binding below declares its
// type as `String`, but the value on the right is a `&str`. rustc will
// tell you exactly that.
//
// Hint: convert the literal into an owned `String`. Two equivalent
// idioms:
//
//     let s: String = "hello".to_string();
//     let s: String = String::from("hello");
//
// Pick either.

fn greet() -> String {
    let s: String = "hello";
    s
}

fn main() {
    let g = greet();
    println!("{g}");
}
