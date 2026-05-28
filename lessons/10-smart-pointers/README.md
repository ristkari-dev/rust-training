# Lesson 10 — Smart pointers

A smart pointer is a type that owns data like a normal value, but
adds behavior: heap allocation, shared ownership, or interior
mutability. The standard library has four you'll reach for
constantly — `Box`, `Rc`, `RefCell`, and `Arc`. This lesson
introduces all four and drills the most foundational, `Box`, through
recursive data structures.

## Learning goals

- Use `Box<T>` to heap-allocate a value and to define recursive types
- Explain why a recursive type needs indirection (`Box`) — infinite
  size otherwise
- Describe `Rc<T>` as shared ownership via reference counting
- Describe `RefCell<T>` as interior mutability with a runtime borrow
  check, and recognize the `Rc<RefCell<T>>` combo
- Recognize `Arc<T>` as the thread-safe `Rc`

## Self-study notes

### `Box<T>` — heap allocation and recursive types

`Box<T>` puts a value on the heap and gives you an owner of fixed,
pointer size:

```rust
let b = Box::new(5);
println!("{}", *b);   // *b dereferences to read the value (mostly automatic)
```

A `Box` is a single owner with the normal ownership rules — it's
dropped when it goes out of scope, freeing the heap memory.

Where `Box` becomes *necessary* is recursive types. A type that
contains itself would have infinite size:

```rust
enum List {
    Cons(i32, Box<List>),   // Box gives a fixed-size pointer
    Nil,
}
```

Without the `Box`, the compiler can't compute the size of `List` (it
contains a `List`, which contains a `List`...). `Box` is a fixed-size
pointer to heap memory, so it breaks the cycle. This lesson's
exercises and compile-fail center on exactly this.

### `Rc<T>` — shared ownership

When one value needs *multiple* owners in single-threaded code, use
`Rc<T>` (reference counting):

```rust
use std::rc::Rc;

let a = Rc::new(String::from("shared"));
let b = Rc::clone(&a);                 // a second owner
println!("{}", Rc::strong_count(&a));  // 2
```

`Rc::clone` doesn't copy the data — it bumps a counter and hands you
another handle to the same data. The data is freed only when the last
`Rc` is dropped.

### `RefCell<T>` — interior mutability

Normally Rust requires `&mut` to mutate. `RefCell<T>` lets you mutate
through a shared `&` reference by moving the borrow check to
*runtime*:

```rust
use std::cell::RefCell;

let cell = RefCell::new(5);
*cell.borrow_mut() += 1;          // mutate via a shared reference
println!("{}", cell.borrow());    // 6
```

The borrowing rules still apply — but they're checked at runtime
instead of compile time. Two simultaneous `borrow_mut()`s panic.

### `Rc<RefCell<T>>` — shared mutable state

Stack them for the common "multiple owners that can all mutate"
pattern:

```rust
let shared = Rc::new(RefCell::new(0));
let clone = Rc::clone(&shared);

*clone.borrow_mut() += 1;
println!("{}", shared.borrow());  // 1 — the mutation is visible through both handles
```

`Rc` provides shared ownership; `RefCell` provides the mutation.
You'll see this combination in tree and graph structures where nodes
need shared, mutable access.

### `Arc<T>` — the thread-safe `Rc`

`Arc<T>` is `Rc<T>` with *atomic* reference counting, making it safe
to share across threads. The API is identical:

```rust
use std::sync::Arc;

let a = Arc::new(42);
let b = Arc::clone(&a);
```

The thread-safe analogue of `Rc<RefCell<T>>` is `Arc<Mutex<T>>`,
which we'll use in **Lesson 16 — Concurrency**. For single-threaded
code, prefer `Rc` — the atomic operations in `Arc` have a small cost.

## Exercises

### Warm-up: `sum`

The exercises crate ships a cons-list enum already using `Box`:

```rust
pub enum List {
    Cons(i32, Box<List>),
    Nil,
}
```

Implement `sum(list: &List) -> i32` that recursively sums all the
values. Match on the two variants: `Cons` holds a value and the rest
of the list; `Nil` is the empty base case (sum 0).

### Main: `count_nodes`

The exercises crate also ships a binary-tree enum:

```rust
pub enum Tree {
    Leaf,
    Node(i32, Box<Tree>, Box<Tree>),
}
```

Implement `count_nodes(tree: &Tree) -> usize` that counts the `Node`
variants. A `Node` contributes 1 plus the counts of its left and
right children; a `Leaf` contributes 0. This is double recursion —
one call for each child.

### Compile-fail

`exercises/compile_fails/10-infinite-size.rs` ships with a recursive
enum that does *not* use `Box`, so it has infinite size. rustc's
error literally suggests inserting a `Box`. Wrap the recursive field
in `Box<List>` to fix it.

### Run

```bash
make verify LESSON=10-smart-pointers
```

This runs your exercise tests and asserts the compile-fail file now
compiles.

## Solutions

See `solutions/src/lib.rs` for the reference implementations. Try the
exercises before peeking.
