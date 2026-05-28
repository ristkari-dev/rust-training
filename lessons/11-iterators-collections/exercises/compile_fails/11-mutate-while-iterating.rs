// Compile-fail exercise: this file MUST NOT compile until you fix it.
//
// Iterating over a collection borrows it. `for x in &v` takes a shared
// (&) borrow of `v` that lasts the whole loop. While that shared borrow
// is alive you cannot ALSO take a mutable borrow — and `v.push(...)`
// needs `&mut v`. The borrow rules from Lessons 8-9 forbid a shared and
// a mutable borrow of the same value at the same time.
//
// rustc will say "cannot borrow `v` as mutable because it is also
// borrowed as immutable" (E0502).
//
// The fix: don't mutate a collection while iterating it. Collect what
// you want into a NEW Vec, then you never hold two borrows of `v` at
// once. For example:
//
//     let doubled: Vec<i32> = v.iter().map(|x| x * 2).collect();
//
// Hint: build a separate `Vec` from the chain instead of pushing into
// `v` inside the loop.

fn main() {
    let mut v = vec![1, 2, 3];
    for x in &v {
        v.push(*x);
    }
    println!("{v:?}");
}
