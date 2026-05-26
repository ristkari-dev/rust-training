// Compile-fail exercise: this file MUST NOT compile until you fix it.
//
// In Rust, passing a value to a function transfers ownership — unless
// the type is `Copy` (i32, bool, etc.). `String` is NOT Copy; it owns
// heap data. After you pass a `String` to a function, you can't use
// the original binding any more — ownership has moved.
//
// The function below calls `print_string(s)` twice on the same
// binding. The second call is a use-after-move and will fail.
//
// Hint: read the rustc error. It will say "value used here after move"
// and point to where the value moved (the first call). The simplest
// fix is to call `print_string(s.clone())` on the first call so the
// function receives a copy and the original `s` stays usable.

fn print_string(s: String) {
    println!("{s}");
}

fn main() {
    let s = String::from("hello");
    print_string(s);
    print_string(s);
}
