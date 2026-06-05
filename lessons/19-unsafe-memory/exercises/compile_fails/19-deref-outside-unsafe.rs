// Compile-fail exercise: this file MUST NOT compile until you fix it.
//
// Creating a raw pointer is safe — `&raw const n` just makes a `*const
// i32`. But DEREFERENCING a raw pointer is one of the operations the
// compiler can't verify (the pointer might be null, dangling, or
// unaligned), so it is only allowed inside an `unsafe` block. Here the
// deref `*ptr` is in ordinary safe code, so the compiler rejects it.
//
// rustc reports E0133: "dereference of raw pointer is unsafe and
// requires unsafe function or block".
//
// The fix: wrap the dereference in an `unsafe { }` block (and, in real
// code, add a `// SAFETY:` comment explaining why the pointer is valid).
//
// Hint: change `let value = *ptr;` to `let value = unsafe { *ptr };`.

fn main() {
    let n = 42;
    let ptr = &raw const n;
    let value = *ptr;
    println!("{value}");
}
