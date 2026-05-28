# Smart pointers

> A smart pointer is a type that owns data like a normal value, but adds behavior: heap allocation, shared ownership, or interior mutability. Four of them cover almost everything.

---

## Recap

Phase 2 has been about ownership and borrowing.

Smart pointers are types that **own** their data (so the ownership rules apply) but wrap it with extra capabilities. They're regular structs in the standard library — nothing magic.

---

## `Box<T>` — heap allocation

```rust
let b = Box::new(5);
println!("{}", *b);   // deref to read — mostly automatic
```

- Puts a value on the heap, gives you a pointer-size owner
- Single owner; dropped when it goes out of scope
- Same ownership rules as any value

The simplest smart pointer.

---

## `Box<T>` for recursive types

This is where `Box` becomes **necessary**:

```rust
enum List {
    Cons(i32, Box<List>),
    Nil,
}
```

A `List` containing a `List` directly would have infinite size — the compiler can't lay it out.

`Box` is a fixed-size pointer to heap memory, breaking the cycle. The compile-fail exercise shows the error without it.

---

## `Rc<T>` — shared ownership

```rust
use std::rc::Rc;

let a = Rc::new(String::from("shared"));
let b = Rc::clone(&a);                 // both own it now
println!("{}", Rc::strong_count(&a));  // 2
```

- For one value with **multiple** owners (single-threaded)
- Reference-counted: dropped when the last `Rc` is dropped
- `Rc::clone` bumps a counter — it doesn't copy the data

---

## `RefCell<T>` — interior mutability

```rust
use std::cell::RefCell;

let cell = RefCell::new(5);
*cell.borrow_mut() += 1;          // mutate through a shared reference
println!("{}", cell.borrow());    // 6
```

- Normally you need `&mut` to mutate
- `RefCell` moves the borrow check to **runtime**
- Break the rules at runtime (two `borrow_mut`s at once) → panic

---

## `Rc<RefCell<T>>` — shared mutable state

```rust
let shared = Rc::new(RefCell::new(0));
let clone = Rc::clone(&shared);

*clone.borrow_mut() += 1;
println!("{}", shared.borrow());  // 1 — seen through the other handle
```

`Rc` gives shared ownership; `RefCell` gives the mutation. A common combo for tree/graph structures.

---

## `Arc<T>` — thread-safe `Rc`

`Arc` is `Rc` with **atomic** reference counting — safe to share across threads.

Same API:

```rust
use std::sync::Arc;

let a = Arc::new(42);
let b = Arc::clone(&a);
```

We'll use `Arc<Mutex<T>>` (the thread-safe analogue of `Rc<RefCell<T>>`) in **Lesson 16 — Concurrency**. For single-threaded code, prefer `Rc` — it's faster.

---

## Putting it together

Today's exercises:

- **Warm-up** a cons `List` with `sum` — single recursion through `Box`
- **Main** a binary `Tree` with `count_nodes` — double recursion

The compile-fail drives home why `Box` is mandatory for recursive types: without it, the type has infinite size.

---

## Wrap — Phase 2 nearly done

- **`Box<T>`** heap-allocates and enables recursive types
- **`Rc<T>`** gives shared ownership via reference counting
- **`RefCell<T>`** moves borrow-checking to runtime for interior mutability
- **`Rc<RefCell<T>>`** combines them for shared mutable state
- **`Arc<T>`** is the thread-safe `Rc`

Next: **Lesson 11 — Iterators & collections** (`Vec`, `HashMap`), which closes out Phase 2.
