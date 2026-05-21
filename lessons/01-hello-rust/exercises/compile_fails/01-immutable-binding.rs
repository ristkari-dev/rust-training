// Compile-fail exercise: this file MUST NOT compile until you fix it.
//
// Task: make this file fail to compile because the code mutates an
// immutable binding. (When you "fix" the program, the fix is to make `x`
// mutable. But the goal of this exercise is to leave it broken and
// understand WHY it's broken. Read the error rustc gives you.)

fn main() {
    let x = 1;
    x = 2;
    println!("{x}");
}
