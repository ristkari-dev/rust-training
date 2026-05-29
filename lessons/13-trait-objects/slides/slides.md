# Trait objects

> Generics give you one function stamped out per type, chosen at compile time. Trait objects give you one pointer that can hold any implementing type, chosen at runtime — the key to mixing types in a single collection.

---

## Recap — Lesson 12's limit

`total_price<T: Priced>(&[T])` is generic: the compiler stamps out a copy per concrete type (static dispatch).

That means every element of the slice must be the *same* `T`. A `Vec` holding both a `Book` and a `Coffee` is impossible this way.

---

## The problem — a mixed collection

```rust
let items = [Book { cents: 100 }, Coffee { shots: 2 }];
//          ^^^^^^^^^^^^^^^^^^^^  ^^^^^^^^^^^^^^^^^^^^
//          expected `Book`, found `Coffee` — won't compile
```

An array or slice demands one uniform element type. We need a way to say "any type that is `Priced`", erasing the concrete type.

---

## What is a trait object?

`dyn Priced` is a **trait object**: a value of some type that implements `Priced`, but *which* type is not known at compile time.

The concrete type is erased; all that remains is "it is `Priced`."

A trait object is **unsized** — the compiler doesn't know how big it is — so you always handle it behind a pointer.

---

## `&dyn Trait` — borrow any implementor

```rust
fn describe_price(item: &dyn Priced) -> String {
    if item.is_free() {
        "free".to_string()
    } else {
        format!("{} cents", item.price())
    }
}
```

`&dyn Priced` is a reference to *any* `Priced` value. Call the trait's methods — required (`price`) and default (`is_free`) — right through it. A `&Book` or `&Coffee` coerces to `&dyn Priced`.

---

## `Box<dyn Trait>` — own any implementor

```rust
let items: Vec<Box<dyn Priced>> = vec![
    Box::new(Book { cents: 100 }),
    Box::new(Coffee { shots: 2 }),   // different types, one Vec!
];
```

`Box<dyn Priced>` *owns* a trait object on the heap. A `Vec<Box<dyn Priced>>` is the heterogeneous collection Lesson 12 couldn't build.

---

## How it works — the vtable

A `&dyn Priced` / `Box<dyn Priced>` is a **fat pointer**: one pointer to the data, one to a **vtable** (a table of the type's method addresses).

Calling `item.price()` looks up `price` in the vtable and jumps there. This runtime lookup is **dynamic dispatch**.

---

## Static vs dynamic dispatch

- **Static** (`<T: Priced>`): resolved at compile time, monomorphized, inlinable — fastest, but one code copy per type and no mixing.
- **Dynamic** (`dyn Priced`): one code path, types mixable at runtime, but each call goes through the vtable (a small cost, no inlining).

Reach for generics by default; reach for `dyn` when you need a heterogeneous collection or runtime-chosen behavior.

---

## Putting it together

Today's exercises:

- **Warm-up** `describe_price(&dyn Priced)` — one object by reference
- **Main** `total_price_dyn(&[Box<dyn Priced>])` — a mixed, owned collection

The compile-fail shows the mixed-array error that `Box<dyn Priced>` fixes.

*(Aside: a trait must be object-safe to become `dyn` — roughly, its methods can't be generic or return `Self`. `Priced` qualifies.)*

---

## Wrap — abstraction toolkit complete

- A trait object `dyn Trait` erases the concrete type behind a pointer
- `&dyn Trait` borrows; `Box<dyn Trait>` owns
- `Vec<Box<dyn Trait>>` is the heterogeneous collection generics can't build
- Dispatch goes through a vtable at runtime
- Choose static dispatch for speed, dynamic for flexibility

Next: **Lesson 14 — Error handling** (`Result`, `?`, `thiserror`/`anyhow`).
