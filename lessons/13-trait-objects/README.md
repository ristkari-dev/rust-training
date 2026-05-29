# Lesson 13 — Trait objects

Generics give you one function stamped out per type, chosen at compile
time — but every element of a collection must then be the same type. A
trait object (`dyn Trait`) erases the concrete type behind a pointer, so
one slot can hold *any* implementor and the right method is found at
runtime. This is how you mix types in a single collection. This lesson
picks up exactly where Lesson 12 left off.

## Learning goals

- Explain what a trait object (`dyn Trait`) is: a value of some
  unknown-at-compile-time type that implements the trait, behind a
  pointer
- Use `&dyn Trait` to accept any implementor by shared reference
- Use `Box<dyn Trait>` and `Vec<Box<dyn Trait>>` to own and store a
  heterogeneous collection
- Contrast static dispatch (generics, compile-time) with dynamic
  dispatch (trait objects, runtime) and their trade-offs
- Recognize that a trait object is unsized, so it always needs
  indirection (`&` or `Box`)

## Self-study notes

### The problem — generics can't mix types

Lesson 12's `total_price<T: Priced>(items: &[T])` is generic: the
compiler stamps out a copy for each concrete `T`. That means every
element of the slice must be the *same* type:

```rust
let items = [Book { cents: 100 }, Coffee { shots: 2 }];
//                                ^^^^^^^^^^^^^^^^^^^^ expected `Book`, found `Coffee`
```

An array, slice, or `Vec<T>` holds one uniform type. To mix `Book` and
`Coffee`, we need to erase the concrete type.

### `&dyn Trait` — borrow any implementor

`dyn Priced` is a *trait object*: "some type that implements `Priced`,"
with the concrete type erased. A trait object is unsized, so you handle
it behind a pointer — the simplest being a shared reference:

```rust
fn describe_price(item: &dyn Priced) -> String {
    if item.is_free() {
        "free".to_string()
    } else {
        format!("{} cents", item.price())
    }
}
```

You call the trait's methods — required (`price`) and default
(`is_free`) — right through the `&dyn`. A `&Book` or `&Coffee` coerces to
`&dyn Priced` automatically.

### `Box<dyn Trait>` — own a heterogeneous collection

To *own* a trait object (e.g. store it in a `Vec`), put it in a `Box`:

```rust
let items: Vec<Box<dyn Priced>> = vec![
    Box::new(Book { cents: 100 }),
    Box::new(Coffee { shots: 2 }),   // different types, one Vec!
];
```

Every element now has the same type — `Box<dyn Priced>` — while the
concrete type lives on the heap behind the pointer. This is the
heterogeneous collection generics couldn't build.

### How it works — the vtable and dynamic dispatch

A `&dyn Priced` or `Box<dyn Priced>` is a *fat pointer*: one pointer to
the data, one to a **vtable** — a table of that type's method addresses.
Calling `item.price()` looks `price` up in the vtable and jumps there.
That runtime lookup is **dynamic dispatch**. It costs a little (and
blocks inlining), but it's what lets one code path serve many types.

### Static vs dynamic dispatch — when to use which

- **Static** (`<T: Priced>`): resolved at compile time, monomorphized,
  inlinable. Fastest, but one code copy per type — and no mixing.
- **Dynamic** (`dyn Priced`): one code path, types mixable at runtime,
  each call through the vtable.

Reach for generics by default; reach for trait objects when you need a
heterogeneous collection or to pick behavior at runtime.

## Exercises

### Warm-up: `describe_price`

Implement `describe_price(item: &dyn Priced) -> String`. Return `"free"`
when the item is free (use the `is_free` default method), otherwise the
price followed by `" cents"` (e.g. `"500 cents"`):

```rust
pub fn describe_price(item: &dyn Priced) -> String {
    // if item.is_free() { ... } else { format!("{} cents", item.price()) }
    todo!()
}
```

The trait, `Book`, and `Coffee` are given — they were Lesson 12's
exercise. This lesson is about *using* the trait object.

### Main: `total_price_dyn`

Implement `total_price_dyn(items: &[Box<dyn Priced>]) -> u32` that sums
the price of every item in a *heterogeneous* slice:

```rust
pub fn total_price_dyn(items: &[Box<dyn Priced>]) -> u32 {
    // items.iter().map(|item| item.price()).sum()
    todo!()
}
```

The slice can mix `Book` and `Coffee` because each element is a
`Box<dyn Priced>`. This is exactly what `total_price<T>` from Lesson 12
could not do.

### Compile-fail

`exercises/compile_fails/13-mixed-array.rs` tries to put a `Book` and a
`Coffee` in one array, which the compiler rejects (E0308 — array
elements must share one type). Fix it by switching to a
`Vec<Box<dyn Priced>>` and wrapping each value in `Box::new(...)`.

### Run

```bash
make verify LESSON=13-trait-objects
```

This runs your exercise tests and asserts the compile-fail file now
compiles.

## Solutions

See `solutions/src/lib.rs` for the reference implementations. Try the
exercises before peeking.
