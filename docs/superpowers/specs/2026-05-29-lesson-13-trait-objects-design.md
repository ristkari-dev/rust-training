# Lesson 13 — Trait objects — design

The second lesson of Phase 3 (Abstraction). Resolves Lesson 12's
cliffhanger: a generic `total_price<T: Priced>` works on a slice of *one*
type (static dispatch), but a mixed `[Book, Coffee]` collection needs
**trait objects** — `dyn Priced` behind a pointer, dispatched
dynamically at runtime. The lesson reuses the exact `Priced`/`Book`/
`Coffee` from L12 so students watch the same code gain a new power.

## Audience and prerequisites

- Has completed Lessons 01-12
- Comfortable with traits, `impl Trait for Type`, default methods, and
  generic functions with trait bounds (L12); `Box<T>` (L10); iterators
  (L11)
- Has the toolchain installed and `make verify` working

## Learning goals

By the end of this lesson, the student can:

1. Explain what a trait object (`dyn Trait`) is: a value of some
   unknown-at-compile-time type that implements the trait, accessed
   behind a pointer
2. Use `&dyn Trait` to accept any implementor by shared reference
3. Use `Box<dyn Trait>` and `Vec<Box<dyn Trait>>` to own and store a
   *heterogeneous* collection
4. Contrast static dispatch (generics, monomorphization, compile-time)
   with dynamic dispatch (trait objects, vtable, runtime) and their
   trade-offs
5. Recognize that a trait object is unsized, so it always needs
   indirection (`&` or `Box`), and that array/slice elements must share
   one type

## Scope

In scope: trait objects via `&dyn Trait` (warm-up) and
`Box<dyn Trait>` / `Vec<Box<dyn Trait>>` (main); calling required and
default trait methods through a trait object; dynamic dispatch and the
vtable (conceptual); the static-vs-dynamic-dispatch trade-off; why a
trait object is unsized and needs a pointer; why a heterogeneous array
is rejected (the compile-fail). The exercises reuse the `Priced` trait
and `Book`/`Coffee` structs from Lesson 12 — these ship *complete*; the
exercise is purely about *using* trait objects.

Out of scope (deferred or skipped): object safety / dyn compatibility
rules in depth (mentioned briefly on one slide — the full E0038 rules
are heavy for this stage); `dyn` with multiple traits
(`dyn A + Send`); `impl Trait` return position vs `Box<dyn Trait>`
return (touched conceptually, not exercised); trait upcasting; `Rc<dyn
Trait>`/`Arc<dyn Trait>` (the pointer types come from L10/L17 — only
`Box`/`&` are exercised here); associated-type or generic-method
interactions with `dyn`; manual vtable details. Trait objects are
introduced as the tool for *heterogeneous collections and runtime
polymorphism*; the deeper object-safety theory waits.

## Slide arc (10 slides)

1. **Title — Trait objects.** Hook: *"Generics give you one function
   stamped out per type, chosen at compile time. Trait objects give you
   one pointer that can hold any implementing type, chosen at runtime —
   the key to mixing types in a single collection."*
2. **Recap — Lesson 12's limit.** `total_price<T: Priced>(&[T])` is
   generic: the compiler stamps out a copy per concrete type (static
   dispatch). That means every element of the slice must be the *same*
   `T`. A `Vec` holding both a `Book` and a `Coffee` is impossible this
   way.
3. **The problem — a mixed collection.**
   ```rust
   let items = [Book { cents: 100 }, Coffee { shots: 2 }];
   //          ^^^^^^^^^^^^^^^^^^^^  ^^^^^^^^^^^^^^^^^^^^
   //          expected `Book`, found `Coffee` — won't compile
   ```
   An array or slice demands one uniform element type. We need a way to
   say "any type that is `Priced`", erasing the concrete type.
4. **What is a trait object?** `dyn Priced` is a *trait object*: a value
   of some type that implements `Priced`, but *which* type is not known
   at compile time. The concrete type is erased; all that remains is "it
   is `Priced`." A trait object is **unsized** — the compiler doesn't
   know how big it is — so you always handle it behind a pointer.
5. **`&dyn Trait` — borrow any implementor.**
   ```rust
   fn describe_price(item: &dyn Priced) -> String {
       if item.is_free() {
           "free".to_string()
       } else {
           format!("{} cents", item.price())
       }
   }
   ```
   `&dyn Priced` is a reference to *any* `Priced` value. You call the
   trait's methods — required (`price`) and default (`is_free`) — right
   through it. Pass it a `&Book` or a `&Coffee`; both coerce to
   `&dyn Priced`.
6. **`Box<dyn Trait>` — own any implementor.**
   ```rust
   let items: Vec<Box<dyn Priced>> = vec![
       Box::new(Book { cents: 100 }),
       Box::new(Coffee { shots: 2 }),   // different types, one Vec!
   ];
   ```
   `Box<dyn Priced>` *owns* a trait object on the heap. A
   `Vec<Box<dyn Priced>>` is the heterogeneous collection L12 couldn't
   build — each element is a different concrete type, unified as
   `dyn Priced`.
7. **How it works — the vtable.** A `&dyn Priced` / `Box<dyn Priced>` is
   a *fat pointer*: one pointer to the data, one to a **vtable** (a table
   of the type's method addresses). Calling `item.price()` looks up
   `price` in the vtable and jumps there. This runtime lookup is
   *dynamic dispatch*.
8. **Static vs dynamic dispatch.**
   - **Static** (`<T: Priced>`): resolved at compile time,
     monomorphized, inlinable — fastest, but one code copy per type and
     no mixing.
   - **Dynamic** (`dyn Priced`): one code path, types mixable at
     runtime, but each call goes through the vtable (a small cost, no
     inlining).

   Reach for generics by default; reach for `dyn` when you need a
   heterogeneous collection or to choose behavior at runtime.
9. **Putting it together.** Walk through the exercises: `describe_price`
   takes a `&dyn Priced` (warm-up — one object by reference), and
   `total_price_dyn` sums a `&[Box<dyn Priced>]` (main — a mixed,
   owned collection). The compile-fail shows the mixed-array error that
   `Box<dyn Priced>` fixes. (Aside: a trait must be *object-safe* to
   become `dyn` — roughly, its methods can't be generic or return
   `Self`; `Priced` qualifies.)
10. **Wrap — abstraction toolkit complete.** Five takeaways: a trait
    object `dyn Trait` erases the concrete type behind a pointer;
    `&dyn Trait` borrows, `Box<dyn Trait>` owns; `Vec<Box<dyn Trait>>`
    is the heterogeneous collection generics can't build; dispatch goes
    through a vtable at runtime; choose static dispatch for speed,
    dynamic for flexibility. Next: **Lesson 14 — Error handling**
    (`Result`, `?`, `thiserror`/`anyhow`).

## Exercise spec

`lessons/13-trait-objects/` follows the standard four-part lesson shape:

```
13-trait-objects/
├── README.md
├── slides/
│   ├── index.html
│   └── slides.md
├── exercises/
│   ├── Cargo.toml
│   ├── src/lib.rs
│   ├── tests/exercise.rs
│   └── compile_fails/13-mixed-array.rs
└── solutions/
    ├── Cargo.toml
    ├── src/lib.rs
    └── tests/exercise.rs
```

Cargo package names: `trait-objects-exercises` and
`trait-objects-solutions` (the lesson's "bare" name is `trait-objects`;
the import idents are `trait_objects_exercises` /
`trait_objects_solutions`). This matches the build-index master registry
slug `trait-objects`, so the landing page links it without any change.

### Exercise stub (`exercises/src/lib.rs`)

The `Priced` trait (with its `is_free` default method), the `Book`/
`Coffee` structs, and both `impl Priced` blocks ship **complete** —
those were Lesson 12's exercise. Lesson 13's exercise is the two
trait-object functions, shipped with `todo!()` bodies. Because the
trait, structs, and impls are complete, the crate and its tests
*compile* in the undone state; the tests fail at runtime with the
`todo!()` panic, like every prior lesson.

```rust
//! Lesson 13 — exercises.
//!
//! Implement `describe_price` (warm-up) and `total_price_dyn` (main) so
//! that `cargo test --manifest-path
//! lessons/13-trait-objects/exercises/Cargo.toml` passes. The trait,
//! structs, and impls are given (they were Lesson 12's exercise) — this
//! lesson is about *using* trait objects. The tests live in
//! `tests/exercise.rs`.

pub trait Priced {
    fn price(&self) -> u32;

    fn is_free(&self) -> bool {
        self.price() == 0
    }
}

pub struct Book {
    pub cents: u32,
}

pub struct Coffee {
    pub shots: u32,
}

impl Priced for Book {
    fn price(&self) -> u32 {
        self.cents
    }
}

impl Priced for Coffee {
    fn price(&self) -> u32 {
        200 + self.shots * 50
    }
}

#[must_use]
pub fn describe_price(_item: &dyn Priced) -> String {
    todo!("return \"free\" if the item is free, otherwise \"<price> cents\"")
}

#[must_use]
pub fn total_price_dyn(_items: &[Box<dyn Priced>]) -> u32 {
    todo!("sum the price of every boxed trait object in the slice")
}
```

### Warm-up: `describe_price`

Reference solution:

```rust
#[must_use]
pub fn describe_price(item: &dyn Priced) -> String {
    if item.is_free() {
        "free".to_string()
    } else {
        format!("{} cents", item.price())
    }
}
```

Pedagogical packing: `&dyn Priced` is the simplest trait object — a
borrow of any implementor. The body calls both the default method
(`is_free`) and the required method (`price`) *through* the trait
object, showing dispatch works the same as on a concrete type. A `&Book`
or `&Coffee` coerces to `&dyn Priced` at the call site.

Four tests:

```rust
#[test]
fn warmup_describe_book() {
    assert_eq!(describe_price(&Book { cents: 500 }), "500 cents");
}

#[test]
fn warmup_describe_free() {
    assert_eq!(describe_price(&Book { cents: 0 }), "free");
}

#[test]
fn warmup_describe_coffee() {
    assert_eq!(describe_price(&Coffee { shots: 2 }), "300 cents");
}

#[test]
fn warmup_describe_via_dyn_ref() {
    let item: &dyn Priced = &Coffee { shots: 1 };
    assert_eq!(describe_price(item), "250 cents");
}
```

### Main: `total_price_dyn`

Reference solution:

```rust
#[must_use]
pub fn total_price_dyn(items: &[Box<dyn Priced>]) -> u32 {
    items.iter().map(|item| item.price()).sum()
}
```

Pedagogical packing: `&[Box<dyn Priced>]` is a slice of owned trait
objects — the heterogeneous collection L12 couldn't build. The iterator
yields `&Box<dyn Priced>`; `item.price()` auto-derefs through the `Box`
and dispatches through the vtable. (The closure `|item| item.price()` is
correct here and is *not* flagged by
`clippy::redundant_closure_for_method_calls` — the `Box` deref layer
means the method-path form `Priced::price` would not type-check;
verified during design.) `iter().map(...).sum()` reuses lesson 11's
iterators.

Four tests (the main test is the *mixed* collection — the payoff):

```rust
#[test]
fn main_total_empty() {
    let items: Vec<Box<dyn Priced>> = Vec::new();
    assert_eq!(total_price_dyn(&items), 0);
}

#[test]
fn main_total_mixed() {
    let items: Vec<Box<dyn Priced>> = vec![
        Box::new(Book { cents: 100 }),
        Box::new(Coffee { shots: 2 }),
        Box::new(Book { cents: 50 }),
    ];
    assert_eq!(total_price_dyn(&items), 450);
}

#[test]
fn main_total_all_coffee() {
    let items: Vec<Box<dyn Priced>> =
        vec![Box::new(Coffee { shots: 1 }), Box::new(Coffee { shots: 3 })];
    assert_eq!(total_price_dyn(&items), 600);
}

#[test]
fn main_total_single() {
    let items: Vec<Box<dyn Priced>> = vec![Box::new(Book { cents: 999 })];
    assert_eq!(total_price_dyn(&items), 999);
}
```

**Eight tests total** (four warm-up + four main). Test arithmetic:
`Coffee { shots: 2 }` → `200 + 2*50 = 300`; `Coffee { shots: 1 }` →
`250`; `main_total_mixed` → `100 + 300 + 50 = 450`; `main_total_all_coffee`
→ `250 + 350 = 600`.

### Compile-fail: `13-mixed-array.rs`

Path: `exercises/compile_fails/13-mixed-array.rs`. A self-contained file
(it defines its own trait, structs, and impls so it stands alone) that
tries to put a `Book` and a `Coffee` in one array. Ships broken; the
student switches to a `Vec<Box<dyn Priced>>` until it compiles.

```rust
// Compile-fail exercise: this file MUST NOT compile until you fix it.
//
// Every element of an array (or slice, or Vec<T>) must have the SAME
// type T. A `Book` and a `Coffee` are different types, so they cannot
// share an array — even though both implement `Priced`. The compiler
// reports a type mismatch (E0308): it expected the array's element type
// `Book`, then found a `Coffee`.
//
// This is exactly the limitation that trait objects solve. Box each
// value as a `Box<dyn Priced>`: now every element has the SAME type
// (`Box<dyn Priced>`), and the concrete type lives behind the pointer.
//
// The fix:
//
//     let items: Vec<Box<dyn Priced>> = vec![
//         Box::new(Book { cents: 100 }),
//         Box::new(Coffee { shots: 2 }),
//     ];
//
// Hint: change the array to a `Vec<Box<dyn Priced>>` and wrap each value
// in `Box::new(...)`.

trait Priced {
    fn price(&self) -> u32;
}

struct Book {
    cents: u32,
}

struct Coffee {
    shots: u32,
}

impl Priced for Book {
    fn price(&self) -> u32 {
        self.cents
    }
}

impl Priced for Coffee {
    fn price(&self) -> u32 {
        200 + self.shots * 50
    }
}

fn main() {
    let items = [Book { cents: 100 }, Coffee { shots: 2 }];
    let total: u32 = items.iter().map(|item| item.price()).sum();
    println!("{total}");
}
```

Pass condition: the student replaces the mixed array with a
`Vec<Box<dyn Priced>>` (boxing each value) so the elements share one
type. rustc reports E0308 ("mismatched types: expected `Book`, found
`Coffee`") on the second element — verified during design.

This is the lesson's centerpiece: the error is precisely the
heterogeneous-collection problem, and `Box<dyn Trait>` is the
resolution.

## README structure

`lessons/13-trait-objects/README.md` follows the established shape:

- **Title + one-paragraph hook**
- **Learning goals** — the five bullets above
- **Self-study notes** with five subsections:
  - The problem — generics can't mix types
  - `&dyn Trait` — borrow any implementor
  - `Box<dyn Trait>` — own a heterogeneous collection
  - How it works — the vtable and dynamic dispatch
  - Static vs dynamic dispatch — when to use which
- **Exercises** — four subsections: Warm-up (`describe_price`), Main
  (`total_price_dyn`), Compile-fail, Run
- **Solutions** — pointer to `solutions/src/lib.rs`

Each `###` subsection runs ~4-6 sentences plus a small code block. The
"`Box<dyn Trait>`" and "Static vs dynamic dispatch" sections are the
heaviest — they carry the lesson's payoff and the key decision.

## Lint expectations

Lesson 13's reference solution code is clippy-clean (with `clippy::all`
+ `clippy::pedantic` denied) without `#[allow]` attributes — verified
during design:

- `describe_price` uses an `if/else` returning owned `String`s; no lint
  fires.
- `total_price_dyn` uses `items.iter().map(|item| item.price()).sum()`.
  The closure is required (not redundant) because the iterator yields
  `&Box<dyn Priced>` and the method-path form `Priced::price` does not
  type-check across the `Box` deref — so
  `clippy::redundant_closure_for_method_calls` does *not* fire (verified;
  this differs from Lesson 12, where the slice was `&[T]` and the
  method-path form was required).
- The trait/structs/impls are identical to Lesson 12's verified-clean
  solution.
- In the *exercise stub* the two function bodies are `todo!()` with
  unused `_item`/`_items` params; this compiles and lints clean
  (verified).

If clippy fires on anything unexpected, fix the code rather than adding
an allow, and report it.

## Done criteria

- `lessons/13-trait-objects/` exists with the four-part structure
- Cargo manifests use the correct package names
  (`trait-objects-exercises`, `trait-objects-solutions`)
- `exercises/src/lib.rs` and `solutions/src/lib.rs` define the same
  `Priced` trait, `Book`/`Coffee` structs and impls, and the
  `describe_price` / `total_price_dyn` signatures; the exercise ships
  `todo!()` bodies for the two functions, the solution ships real bodies
- `cargo test --package trait-objects-solutions` → 8 tests pass
- `cargo test --manifest-path lessons/13-trait-objects/exercises/Cargo.toml`
  → compiles, all 8 tests panic with `not yet implemented` (the intended
  undone state)
- `cargo run --package compile-fails -- --expect broken lessons/13-trait-objects`
  → ok
- `cargo run --package compile-fails -- --expect compiles lessons/13-trait-objects`
  → fails (file ships broken, that's the point)
- `make ci` is green
- `make slides-build` produces `dist/lessons/13-trait-objects/slides/index.html`
- `dist/index.html` lists lesson 13 as a clickable link (registry slug
  `trait-objects` already matches this directory name)
- One push to `origin/main` triggers a green CI run and a green Deploy
  run; `https://rust.ristkari.dev/lessons/13-trait-objects/slides/`
  returns 200

## Open questions

None.
