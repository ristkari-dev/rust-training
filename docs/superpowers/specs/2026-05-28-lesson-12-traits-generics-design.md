# Lesson 12 — Traits & generics — design

The first lesson of Phase 3 (Abstraction). Introduces traits (shared
behavior you define and implement) and generics (functions that work for
any type satisfying a bound). The through-line: define a trait,
implement it for concrete types, then write one generic function that
works for *any* type implementing it — static dispatch. Mixing different
types in one collection is not yet possible; that motivates Lesson 13's
trait objects.

## Audience and prerequisites

- Has completed Lessons 01-11
- Comfortable with structs + methods (L06), enums + `match` (L05),
  ownership/borrowing (L07-09), and iterators (L11)
- Has the toolchain installed and `make verify` working

## Learning goals

By the end of this lesson, the student can:

1. Define a trait with a required method and a default method
2. Implement a trait for their own types (`impl Trait for Type`)
3. Explain how a default method builds on required methods
4. Write a generic function with a trait bound (`<T: Trait>`)
5. Describe static dispatch / monomorphization at a high level (the
   compiler generates a specialized copy per concrete type — no runtime
   cost)
6. Recognize that a generic function needs a trait bound before it can
   call the trait's methods

## Scope

In scope: defining a trait with a required method and a default method;
implementing a trait for structs; generic functions bounded by a trait
(`<T: Trait>`); static dispatch / monomorphization (conceptual);
`impl Trait` in argument position and the `where` clause as alternate
syntax (slides/README only, not exercised). The exercises drill the
trait→generic-bound pairing: a `Priced` trait (warm-up) and a generic
`total_price<T: Priced>` (main).

Out of scope (deferred or skipped): trait objects, `dyn Trait`, and
dynamic dispatch (Lesson 13 — the natural next step for *mixed*
collections); associated types; generic data types (`struct Wrapper<T>`)
and `impl<T>` blocks (mentioned at most in passing); supertraits / trait
inheritance; blanket impls; operator-overloading traits (`Add`, etc.);
deriving traits beyond a one-line mention; object safety / dyn
compatibility; const generics; generic lifetime bounds. Generics are
introduced here for *functions* over a single bounded `T`; generic data
types and dynamic dispatch come later.

## Slide arc (10 slides)

1. **Title — Traits & generics.** Hook: *"A trait is a set of behavior
   a type promises to provide. A generic function works for any type
   that keeps the promise. Together they're how Rust does abstraction —
   with zero runtime cost."*
2. **Phase 3 — why abstraction.** Phases 1-2 built up the language; now
   we reuse code across types. Without traits you'd copy a function once
   per type. A trait names a capability; generics let one function serve
   every type that has it.
3. **Defining a trait.**
   ```rust
   trait Priced {
       fn price(&self) -> u32;   // required: every impl must provide it
   }
   ```
   A trait is a named set of method signatures. It defines *what* a type
   can do without saying *how*.
4. **Implementing a trait.**
   ```rust
   struct Book { cents: u32 }

   impl Priced for Book {
       fn price(&self) -> u32 {
           self.cents
       }
   }
   ```
   `impl Trait for Type` provides the bodies. Now `Book` *is* `Priced`
   and you can call `book.price()`.
5. **Default methods.**
   ```rust
   trait Priced {
       fn price(&self) -> u32;

       fn is_free(&self) -> bool {   // default — built on price()
           self.price() == 0
       }
   }
   ```
   A trait can provide a default body. Implementors get `is_free` for
   free and may override it. Default methods can call the required ones.
6. **Generic functions & trait bounds.**
   ```rust
   fn total_price<T: Priced>(items: &[T]) -> u32 {
       items.iter().map(Priced::price).sum()
   }
   ```
   `<T: Priced>` is a *trait bound*: "`T` can be any type, as long as it
   implements `Priced`." Inside, you may call any `Priced` method on a
   `T`. One function, every priced type.
7. **Static dispatch — monomorphization.**
   ```rust
   // The compiler generates a specialized copy per type you use:
   total_price::<Book>(&books);     // one machine-code version
   total_price::<Coffee>(&coffees); // another — chosen at compile time
   ```
   Generics are resolved at compile time: rustc stamps out a concrete
   version for each type. No runtime lookup, no overhead — "zero-cost
   abstraction". The trade-off is bigger binaries.
8. **`impl Trait` and `where`.**
   ```rust
   fn total(items: &[impl Priced]) -> u32 { /* ... */ }   // sugar for <T: Priced>

   fn total2<T>(items: &[T]) -> u32
   where
       T: Priced,
   { /* ... */ }
   ```
   `impl Trait` in argument position is shorthand for a generic bound.
   `where` moves bounds below the signature — handy when there are
   several. All three forms mean the same thing.
9. **Putting it together.** Walk through the exercises: implement
   `Priced` for `Book` and `Coffee` (warm-up — the trait and structs are
   given), then write the generic `total_price` (main). Note the limit:
   `total_price` takes a slice of *one* type. A mixed `[Book, Coffee]`
   list needs **trait objects** — that's Lesson 13.
10. **Wrap — Phase 3 begins.** Five takeaways: a trait names shared
    behavior; `impl Trait for Type` provides it; default methods build on
    required ones; `<T: Trait>` bounds a generic so it can call the
    trait's methods; generics use static dispatch (monomorphization),
    zero runtime cost. Next: **Lesson 13 — Trait objects** (`dyn Trait`,
    dynamic dispatch) for heterogeneous collections.

## Exercise spec

`lessons/12-traits-generics/` follows the standard four-part lesson
shape:

```
12-traits-generics/
├── README.md
├── slides/
│   ├── index.html
│   └── slides.md
├── exercises/
│   ├── Cargo.toml
│   ├── src/lib.rs
│   ├── tests/exercise.rs
│   └── compile_fails/12-missing-trait-bound.rs
└── solutions/
    ├── Cargo.toml
    ├── src/lib.rs
    └── tests/exercise.rs
```

Cargo package names: `traits-generics-exercises` and
`traits-generics-solutions` (the lesson's "bare" name is
`traits-generics`; the import idents are `traits_generics_exercises` /
`traits_generics_solutions`).

### Exercise stub (`exercises/src/lib.rs`)

The crate ships the `Priced` trait (with its `is_free` default method),
the `Book` and `Coffee` structs, and the `impl Priced` *skeletons* with
`todo!()` bodies. Students fill in the two `price` bodies (warm-up) and
the `total_price` body (main). Shipping the skeletons (rather than
asking students to write the `impl` headers from scratch) keeps the
crate and its tests *compiling* in the undone state — the tests then
fail at runtime with the `todo!()` panic, exactly like every prior
lesson. The README explains the `impl Trait for Type` mechanics.

```rust
//! Lesson 12 — exercises.
//!
//! Fill in the `price` bodies for `Book` and `Coffee` (warm-up) and the
//! `total_price` body (main) so that `cargo test --manifest-path
//! lessons/12-traits-generics/exercises/Cargo.toml` passes. The tests
//! live in `tests/exercise.rs`.

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
        todo!("return this book's price in cents")
    }
}

impl Priced for Coffee {
    fn price(&self) -> u32 {
        todo!("compute this coffee's price: 200 plus 50 per shot")
    }
}

#[must_use]
pub fn total_price<T: Priced>(_items: &[T]) -> u32 {
    todo!("sum the price of every item in the slice")
}
```

### Warm-up: implement `Priced`

Reference solution (the two `price` bodies):

```rust
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
```

Pedagogical packing: two different implementations of the same trait —
`Book` returns a field directly, `Coffee` computes from `shots`. The
`is_free` default method (given in the trait) then works automatically
on top of either `price`. This shows that a default method builds on the
required method without the implementor writing it.

Four tests:

```rust
#[test]
fn warmup_book_price() {
    assert_eq!(Book { cents: 1299 }.price(), 1299);
}

#[test]
fn warmup_coffee_price() {
    assert_eq!(Coffee { shots: 2 }.price(), 300);
}

#[test]
fn warmup_is_free_true() {
    assert!(Book { cents: 0 }.is_free());
}

#[test]
fn warmup_is_free_false() {
    assert!(!Coffee { shots: 1 }.is_free());
}
```

### Main: `total_price`

Reference solution:

```rust
#[must_use]
pub fn total_price<T: Priced>(items: &[T]) -> u32 {
    items.iter().map(Priced::price).sum()
}
```

Pedagogical packing: a generic function bounded by `T: Priced`. The
bound is what lets the body call `.price()` on each item. `iter().map(
Priced::price).sum()` reinforces lesson 11's iterators; `Priced::price`
is the trait method used as a function value (the closure form
`|item| item.price()` is rejected by `clippy::redundant_closure_for_method_calls`,
so the method-path form is both cleaner and lint-clean). The same
function serves a slice of `Book` or a slice of `Coffee` — but not a
mixed slice (that needs L13).

Four tests:

```rust
#[test]
fn main_total_empty() {
    let books: [Book; 0] = [];
    assert_eq!(total_price(&books), 0);
}

#[test]
fn main_total_books() {
    let books = [Book { cents: 100 }, Book { cents: 250 }, Book { cents: 50 }];
    assert_eq!(total_price(&books), 400);
}

#[test]
fn main_total_coffees() {
    let coffees = [Coffee { shots: 1 }, Coffee { shots: 3 }];
    assert_eq!(total_price(&coffees), 600);
}

#[test]
fn main_total_single() {
    let books = [Book { cents: 999 }];
    assert_eq!(total_price(&books), 999);
}
```

**Eight tests total** (four warm-up + four main). Test arithmetic:
`Coffee { shots: 2 }` → `200 + 2*50 = 300`; `Coffee { shots: 1 }` →
`250` (not free); `main_total_coffees` → `250 + 350 = 600`.

### Compile-fail: `12-missing-trait-bound.rs`

Path: `exercises/compile_fails/12-missing-trait-bound.rs`. A
self-contained file (it defines its own trait, struct, and impl so it
stands alone) whose generic function is missing the `T: Priced` bound.
Ships broken; the student adds the bound until it compiles.

```rust
// Compile-fail exercise: this file MUST NOT compile until you fix it.
//
// A generic type parameter `T` with no bounds could be ANY type. The
// compiler therefore has no idea whether a `T` has a `.price()` method
// — most types don't — so it rejects the call.
//
// rustc will say "no method named `price` found" for `T` (E0599) and
// suggests restricting the type parameter with a trait bound.
//
// The fix: tell the compiler that `T` must implement `Priced`, so every
// `T` is guaranteed to have `.price()`. Change `<T>` to `<T: Priced>`.
//
// Hint: add the bound `T: Priced` to the generic function.

trait Priced {
    fn price(&self) -> u32;
}

struct Book {
    cents: u32,
}

impl Priced for Book {
    fn price(&self) -> u32 {
        self.cents
    }
}

fn total_price<T>(items: &[T]) -> u32 {
    let mut total = 0;
    for item in items {
        total += item.price();
    }
    total
}

fn main() {
    let books = [Book { cents: 100 }, Book { cents: 200 }];
    let _ = total_price(&books);
}
```

Pass condition: the student changes `fn total_price<T>` to
`fn total_price<T: Priced>`. rustc reports E0599 ("no method named
`price` found ... for type parameter `T`") and suggests adding the
bound. After the fix the file compiles.

This is the lesson's centerpiece for generics: a bound is not
bureaucracy — it is the *promise* that makes the method call possible.
Without it, the compiler cannot know `T` has the method.

## README structure

`lessons/12-traits-generics/README.md` follows the established shape:

- **Title + one-paragraph hook**
- **Learning goals** — the six bullets above
- **Self-study notes** with five subsections:
  - Defining a trait
  - Implementing a trait
  - Default methods
  - Generic functions & trait bounds
  - Static dispatch — and `impl Trait` / `where`
- **Exercises** — four subsections: Warm-up (implement `Priced`), Main
  (`total_price`), Compile-fail, Run
- **Solutions** — pointer to `solutions/src/lib.rs`

Each `###` subsection runs ~4-6 sentences plus a small code block. The
"Generic functions & trait bounds" and "Static dispatch" sections are
the heaviest — they carry the generics half the exercises drill.

## Lint expectations

Lesson 12's reference solution code is clippy-clean (with `clippy::all`
+ `clippy::pedantic` denied) without `#[allow]` attributes — verified
during design:

- `total_price` uses `items.iter().map(Priced::price).sum()`. The
  method-path `Priced::price` (not a `|item| item.price()` closure)
  avoids `clippy::redundant_closure_for_method_calls`.
- `Book::price` / `Coffee::price` use `self`, so `clippy::unused_self`
  does not fire. (In the *exercise stub* the bodies are `todo!()` so
  `self` is unused, but `unused_self` excludes trait-impl methods, so
  the stub also lints clean — verified.)
- The trait methods (`price`, `is_free`) do not trigger
  `clippy::must_use_candidate`; only the free function `total_price`
  needs `#[must_use]`, which it has.

If clippy fires on anything unexpected, fix the code rather than adding
an allow, and report it.

## Done criteria

- `lessons/12-traits-generics/` exists with the four-part structure
- Cargo manifests use the correct package names
  (`traits-generics-exercises`, `traits-generics-solutions`)
- `exercises/src/lib.rs` and `solutions/src/lib.rs` define the same
  `Priced` trait, `Book`/`Coffee` structs, and `total_price` signature;
  the exercise ships `todo!()` bodies, the solution ships real bodies
- `cargo test --package traits-generics-solutions` → 8 tests pass
- `cargo test --manifest-path lessons/12-traits-generics/exercises/Cargo.toml`
  → compiles, all 8 tests panic with `not yet implemented` (the intended
  undone state)
- `cargo run --package compile-fails -- --expect broken lessons/12-traits-generics`
  → ok
- `cargo run --package compile-fails -- --expect compiles lessons/12-traits-generics`
  → fails (file ships broken, that's the point)
- `make ci` is green
- `make slides-build` produces `dist/lessons/12-traits-generics/slides/index.html`
- `dist/index.html` lists lesson 12 as a clickable link (the build-index
  master registry already has slug `traits-generics`, which matches this
  directory name)
- One push to `origin/main` triggers a green CI run and a green Deploy
  run; `https://rust.ristkari.dev/lessons/12-traits-generics/slides/`
  returns 200

## Open questions

None.
