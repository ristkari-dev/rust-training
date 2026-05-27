// Compile-fail exercise: this file MUST NOT compile until you fix it.
//
// Any struct that holds a reference must declare a lifetime parameter
// and use it on the field. Otherwise the compiler can't track how
// long the struct is valid — and refuses to compile.
//
// The struct below tries to hold a `&str` without a lifetime
// parameter. rustc will say "missing lifetime specifier" and suggest
// the fix.
//
// Hint: read the rustc error. The fix is to declare `<'a>` on the
// struct and use `&'a str` for the field. Like this:
//
//     struct Excerpt<'a> {
//         text: &'a str,
//     }

struct Excerpt {
    text: &str,
}

fn main() {
    let s = String::from("hello world");
    let e = Excerpt { text: &s };
    println!("{}", e.text);
}
