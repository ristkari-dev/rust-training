# Lesson 12 — Traits & generics

A trait is a set of behavior a type promises to provide; a generic
function works for any type that keeps the promise. Together they are how
Rust does abstraction — with zero runtime cost. This lesson opens Phase 3:
you define a trait, implement it for concrete types, and write one
generic function that serves all of them.

## Learning goals

- Define a trait with a required method and a default method
- Implement a trait for your own types (`impl Trait for Type`)
- Explain how a default method builds on required methods
- Write a generic function with a trait bound (`<T: Trait>`)
- Describe static dispatch / monomorphization at a high level
- Recognize that a generic function needs a trait bound before it can
  call the trait's methods

## Self-study notes

### Defining a trait

A trait is a named set of method signatures — it says *what* a type can
do without saying *how*:

```rust
trait Priced {
    fn price(&self) -> u32;   // required: every implementor must provide it
}
```

Any type that implements `Priced` promises to have a `price` method.

### Implementing a trait

`impl Trait for Type` provides the bodies for one specific type:

```rust
struct Book {
    cents: u32,
}

impl Priced for Book {
    fn price(&self) -> u32 {
        self.cents
    }
}
```

Now `Book` *is* `Priced` and you can call `book.price()`. You can
implement the same trait for many different types, each with its own
body.

### Default methods

A trait can supply a default body. Implementors get it for free and may
override it:

```rust
trait Priced {
    fn price(&self) -> u32;

    fn is_free(&self) -> bool {
        self.price() == 0
    }
}
```

`is_free` calls the required `price`, so it works for every implementor
without any of them writing it. Default methods are how traits ship
shared behavior.

### Generic functions & trait bounds

A generic function works for many types. A *trait bound* says which ones:

```rust
fn total_price<T: Priced>(items: &[T]) -> u32 {
    items.iter().map(Priced::price).sum()
}
```

`<T: Priced>` means "`T` can be any type, as long as it implements
`Priced`." That bound is exactly what lets the body call `.price()` on a
`T`. One function now serves a slice of `Book` *or* a slice of `Coffee`.

### Static dispatch — and `impl Trait` / `where`

Generics are resolved at compile time. The compiler stamps out a
specialized copy of `total_price` for each concrete type you use — no
runtime lookup, no overhead. This is "monomorphization" (the trade-off
is larger binaries). Two more spellings of the same bound:

```rust
fn total(items: &[impl Priced]) -> u32 { /* ... */ }   // impl Trait sugar

fn total2<T>(items: &[T]) -> u32
where
    T: Priced,
{ /* ... */ }
```

`impl Trait` in argument position is shorthand for a generic bound;
`where` moves bounds below the signature when there are several.

## Exercises

### Warm-up: implement `Priced`

The exercises crate ships the `Priced` trait (with its `is_free` default
method) and two structs:

```rust
pub struct Book {
    pub cents: u32,
}

pub struct Coffee {
    pub shots: u32,
}
```

Fill in the two `price` bodies. `Book::price` returns the `cents` field.
`Coffee::price` computes `200 + shots * 50`. Once `price` works,
`is_free` (the default method) works automatically.

### Main: `total_price`

Implement the generic function `total_price<T: Priced>(items: &[T]) ->
u32` that sums the price of every item in a slice:

```rust
pub fn total_price<T: Priced>(items: &[T]) -> u32 {
    // items.iter().map(Priced::price).sum()
    todo!()
}
```

The `T: Priced` bound lets you call `.price()` on each item. The same
function works for a slice of `Book` or a slice of `Coffee` — but not a
mixed slice. A mixed collection needs *trait objects*, which is Lesson
13.

### Compile-fail

`exercises/compile_fails/12-missing-trait-bound.rs` has a generic
function that calls `.price()` on a `T` with no trait bound, so the
compiler rejects it (E0599 — `T` could be any type, most of which have no
`price`). Fix it by changing `<T>` to `<T: Priced>`.

### Run

```bash
make verify LESSON=12-traits-generics
```

This runs your exercise tests and asserts the compile-fail file now
compiles.

## Solutions

See `solutions/src/lib.rs` for the reference implementations. Try the
exercises before peeking.
