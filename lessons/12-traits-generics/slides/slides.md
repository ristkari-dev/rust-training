# Traits & generics

> A trait is a set of behavior a type promises to provide. A generic function works for any type that keeps the promise. Together they're how Rust does abstraction — with zero runtime cost.

---

## Phase 3 — why abstraction

Phases 1-2 built up the language. Now we reuse code across types.

Without traits you'd copy a function once per type. A **trait** names a capability; **generics** let one function serve every type that has it.

---

## Defining a trait

```rust
trait Priced {
    fn price(&self) -> u32;   // required: every impl must provide it
}
```

A trait is a named set of method signatures. It defines *what* a type can do without saying *how*.

---

## Implementing a trait

```rust
struct Book { cents: u32 }

impl Priced for Book {
    fn price(&self) -> u32 {
        self.cents
    }
}
```

`impl Trait for Type` provides the bodies. Now `Book` *is* `Priced` and you can call `book.price()`.

---

## Default methods

```rust
trait Priced {
    fn price(&self) -> u32;

    fn is_free(&self) -> bool {   // default — built on price()
        self.price() == 0
    }
}
```

A trait can provide a default body. Implementors get `is_free` for free and may override it. Default methods can call the required ones.

---

## Generic functions & trait bounds

```rust
fn total_price<T: Priced>(items: &[T]) -> u32 {
    items.iter().map(Priced::price).sum()
}
```

`<T: Priced>` is a *trait bound*: "`T` can be any type, as long as it implements `Priced`." Inside, you may call any `Priced` method on a `T`. One function, every priced type.

---

## Static dispatch — monomorphization

```rust
// The compiler generates a specialized copy per type you use:
total_price::<Book>(&books);     // one machine-code version
total_price::<Coffee>(&coffees); // another — chosen at compile time
```

Generics are resolved at compile time: rustc stamps out a concrete version for each type. No runtime lookup, no overhead — "zero-cost abstraction". The trade-off is bigger binaries.

---

## `impl Trait` and `where`

```rust
fn total(items: &[impl Priced]) -> u32 { /* ... */ }   // sugar for <T: Priced>

fn total2<T>(items: &[T]) -> u32
where
    T: Priced,
{ /* ... */ }
```

`impl Trait` in argument position is shorthand for a generic bound. `where` moves bounds below the signature. All three forms mean the same thing.

---

## Putting it together

Today's exercises:

- **Warm-up** implement `Priced` for `Book` and `Coffee` (the trait and structs are given)
- **Main** write the generic `total_price<T: Priced>(&[T]) -> u32`

`total_price` takes a slice of *one* type. A mixed `[Book, Coffee]` list needs **trait objects** — that's Lesson 13.

---

## Wrap — Phase 3 begins

- A trait names shared behavior
- `impl Trait for Type` provides it
- Default methods build on required ones
- `<T: Trait>` bounds a generic so it can call the trait's methods
- Generics use static dispatch (monomorphization) — zero runtime cost

Next: **Lesson 13 — Trait objects** (`dyn Trait`, dynamic dispatch) for heterogeneous collections.
