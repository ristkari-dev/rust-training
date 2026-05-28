// Compile-fail exercise: this file MUST NOT compile until you fix it.
//
// A recursive type — one that contains itself — has, in principle,
// infinite size: a List contains a List contains a List... forever.
// The compiler can't lay that out in memory, so it refuses.
//
// The fix is to put the recursive part behind a pointer of known,
// fixed size: Box<T>. A Box is just a pointer to heap memory, so its
// size is the same no matter how big the thing it points to is.
//
// rustc will say "recursive type `List` has infinite size" and even
// suggest the fix: insert a `Box`.
//
// Hint: change the recursive field from `List` to `Box<List>`.

enum List {
    Cons(i32, List),
    Nil,
}

fn main() {
    let _list = List::Nil;
}
