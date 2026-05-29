# Lesson 13 — Trait objects — Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Author the second lesson of Phase 3 of the Rust training course: trait objects. Resolves Lesson 12's cliffhanger — a generic `total_price<T: Priced>` can't hold a mixed `[Book, Coffee]`, but `dyn Priced` behind a pointer can. Warm-up: `describe_price(&dyn Priced) -> String`. Main: `total_price_dyn(&[Box<dyn Priced>]) -> u32` over a heterogeneous collection. Compile-fail: a mixed array (E0308) fixed by `Vec<Box<dyn Priced>>`.

**Architecture:** Use the existing `make new-lesson` scaffolder. The `Priced` trait, `Book`/`Coffee` structs, and their impls ship complete (they were Lesson 12's exercise) — Lesson 13's exercise is purely the two trait-object functions, shipped with `todo!()` bodies. The crate and tests compile in the undone state; tests panic at runtime, like every prior lesson. All reference code in this plan was empirically verified clippy-pedantic-clean during design.

**Tech Stack:** Rust 2024 edition, existing tools (`new-lesson`, `compile-fails`, `slides-dev`, `build-index`), reveal.js (vendored), GNU Make.

**Spec:** [`docs/superpowers/specs/2026-05-29-lesson-13-trait-objects-design.md`](../specs/2026-05-29-lesson-13-trait-objects-design.md).

**Working directory:** `/Users/ristkari/code/private/rust-training`.

---

## Task 1: Scaffold lessons/13-trait-objects

**Files (all created by the scaffolder):**
- `lessons/13-trait-objects/README.md` (placeholder, replaced in Task 4)
- `lessons/13-trait-objects/slides/index.html` (final — no edit needed)
- `lessons/13-trait-objects/slides/slides.md` (placeholder, replaced in Task 5)
- `lessons/13-trait-objects/exercises/Cargo.toml` (final — no edit needed)
- `lessons/13-trait-objects/exercises/src/lib.rs` (placeholder, replaced in Task 2)
- `lessons/13-trait-objects/exercises/tests/exercise.rs` (placeholder, replaced in Task 2)
- `lessons/13-trait-objects/solutions/Cargo.toml` (final — no edit needed)
- `lessons/13-trait-objects/solutions/src/lib.rs` (placeholder, replaced in Task 3)
- `lessons/13-trait-objects/solutions/tests/exercise.rs` (placeholder, replaced in Task 3)

- [ ] **Step 1: Run the scaffolder**

```bash
make new-lesson NAME=13-trait-objects
```

Expected: `scaffolded ./lessons/13-trait-objects`.

- [ ] **Step 2: Verify directory structure**

```bash
ls lessons/13-trait-objects/
ls lessons/13-trait-objects/slides/ lessons/13-trait-objects/exercises/ lessons/13-trait-objects/solutions/
```

Expected: top-level `README.md`, `slides/`, `exercises/`, `solutions/` populated from templates.

- [ ] **Step 3: Verify Cargo package names**

```bash
grep '^name' lessons/13-trait-objects/exercises/Cargo.toml lessons/13-trait-objects/solutions/Cargo.toml
```

Expected:
```
lessons/13-trait-objects/exercises/Cargo.toml:name = "trait-objects-exercises"
lessons/13-trait-objects/solutions/Cargo.toml:name = "trait-objects-solutions"
```

- [ ] **Step 4: Verify the workspace picks up the new crates**

```bash
cargo metadata --no-deps --format-version 1 | grep -o '"name":"trait-objects-[^"]*"' | sort -u
```

Expected output:
```
"name":"trait-objects-exercises"
"name":"trait-objects-solutions"
```

- [ ] **Step 5: Verify the scaffolded workspace builds clean**

```bash
cargo build --workspace
```

Expected: warning-free build.

- [ ] **Step 6: Commit the scaffold**

```bash
git add lessons/13-trait-objects
git commit -m "chore: scaffold lessons/13-trait-objects"
```

---

## Task 2: Exercise content (trait + structs + stubs + tests + compile-fail)

**Files:**
- Overwrite: `lessons/13-trait-objects/exercises/src/lib.rs`
- Overwrite: `lessons/13-trait-objects/exercises/tests/exercise.rs`
- Create: `lessons/13-trait-objects/exercises/compile_fails/13-mixed-array.rs`

- [ ] **Step 1: Overwrite `lessons/13-trait-objects/exercises/src/lib.rs`**

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

- [ ] **Step 2: Overwrite `lessons/13-trait-objects/exercises/tests/exercise.rs`**

```rust
use trait_objects_exercises::{Book, Coffee, Priced, describe_price, total_price_dyn};

// Warm-up: describe_price (&dyn Priced)

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

// Main: total_price_dyn (&[Box<dyn Priced>])

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

- [ ] **Step 3: Create `lessons/13-trait-objects/exercises/compile_fails/13-mixed-array.rs`**

The `compile_fails/` directory does not exist yet — create it. Write this file:

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

- [ ] **Step 4: Verify exercise tests compile and fail with `todo!()` panic (intentional)**

```bash
cargo test --manifest-path lessons/13-trait-objects/exercises/Cargo.toml
```

Expected: the crate COMPILES, then all 8 tests FAIL with `not yet implemented` panic message. (Compilation succeeding while tests panic is the correct undone state — the trait, structs, and impls are complete; only the two function bodies are `todo!()`.)

- [ ] **Step 5: Verify the exercises crate still builds cleanly**

```bash
cargo build --package trait-objects-exercises
```

Expected: warning-free build.

- [ ] **Step 6: Verify compile-fail ships broken**

```bash
cargo run --package compile-fails -- --expect broken lessons/13-trait-objects
```

Expected: prints `ok: lessons/13-trait-objects/exercises/compile_fails/13-mixed-array.rs` and exits 0. (The tool printing the rustc E0308 error text is expected — what matters is the final `ok:` line and exit 0.)

- [ ] **Step 7: Verify compile-fail's student-mode check fires**

```bash
cargo run --package compile-fails -- --expect compiles lessons/13-trait-objects
```

Expected: non-zero exit with a `FAIL: file did not compile, but was expected to: lessons/13-trait-objects/...` message. (This is correct — the file ships broken on purpose.)

- [ ] **Step 8: Verify lint passes on the exercises crate**

```bash
cargo clippy --package trait-objects-exercises --all-targets -- -D warnings
cargo fmt --check --package trait-objects-exercises
```

Expected: both exit 0. (The `todo!()` bodies with unused `_item`/`_items` params lint clean — verified during design.)

- [ ] **Step 9: Commit**

```bash
git add lessons/13-trait-objects/exercises
git commit -m "feat(lesson-13): add trait-object exercise stubs, tests, and compile-fail"
```

---

## Task 3: Reference solutions

**Files:**
- Overwrite: `lessons/13-trait-objects/solutions/src/lib.rs`
- Overwrite: `lessons/13-trait-objects/solutions/tests/exercise.rs`

- [ ] **Step 1: Overwrite `lessons/13-trait-objects/solutions/src/lib.rs`**

```rust
//! Lesson 13 — reference solutions.

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
pub fn describe_price(item: &dyn Priced) -> String {
    if item.is_free() {
        "free".to_string()
    } else {
        format!("{} cents", item.price())
    }
}

#[must_use]
pub fn total_price_dyn(items: &[Box<dyn Priced>]) -> u32 {
    items.iter().map(|item| item.price()).sum()
}
```

> Pedagogical notes:
> - `describe_price` takes a `&dyn Priced` — a borrow of any implementor. It calls both the default method (`is_free`) and the required method (`price`) through the trait object. A `&Book` or `&Coffee` coerces to `&dyn Priced` at the call site.
> - `total_price_dyn` takes a `&[Box<dyn Priced>]` — a slice of owned trait objects, the heterogeneous collection Lesson 12 couldn't build. `item.price()` auto-derefs through the `Box` and dispatches via the vtable.
> - The closure `|item| item.price()` is REQUIRED here and must NOT be rewritten as `.map(Priced::price)`: the iterator yields `&Box<dyn Priced>`, and the method-path form does not type-check across the `Box` deref. `clippy::redundant_closure_for_method_calls` does not fire on it (verified during design). This differs from Lesson 12, where the slice was `&[T]` and the method-path form was the lint-clean choice.
> - No `#[allow]` attributes should be needed. If clippy fires unexpectedly, fix the code rather than adding an allow, and report the deviation.

- [ ] **Step 2: Overwrite `lessons/13-trait-objects/solutions/tests/exercise.rs`**

```rust
use trait_objects_solutions::{Book, Coffee, Priced, describe_price, total_price_dyn};

// Warm-up: describe_price (&dyn Priced)

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

// Main: total_price_dyn (&[Box<dyn Priced>])

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

- [ ] **Step 3: Verify solution tests pass**

```bash
cargo test --package trait-objects-solutions
```

Expected: 8 tests pass.

- [ ] **Step 4: Verify lint passes on the solutions crate**

```bash
cargo clippy --package trait-objects-solutions --all-targets -- -D warnings
cargo fmt --check --package trait-objects-solutions
```

Expected: both exit 0. No `#[allow]` attributes needed. If clippy fires on anything, fix the code (not with an allow) and report it.

- [ ] **Step 5: Commit**

```bash
git add lessons/13-trait-objects/solutions
git commit -m "feat(lesson-13): add reference solutions"
```

---

## Task 4: Lesson README

**Files:**
- Overwrite: `lessons/13-trait-objects/README.md`

- [ ] **Step 1: Overwrite `lessons/13-trait-objects/README.md`**

The complete file content is below, delimited by an OUTER quadruple-backtick fence (` ```` `). That outer fence is ONLY a delimiter for this plan — do NOT write it into the file. The file must start with `# Lesson 13` on line 1 and contain only PLAIN triple-backtick (` ``` `) code fences.

````markdown
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
````

- [ ] **Step 2: Spot-check the README**

```bash
head -1 lessons/13-trait-objects/README.md
grep -c '^### ' lessons/13-trait-objects/README.md
grep -c '^```' lessons/13-trait-objects/README.md
```

Expected:
- First line: `# Lesson 13 — Trait objects`
- `grep -c '^### '` returns `9` (five subsections under self-study + four under exercises)
- `grep -c '^```'` returns `12` (6 code blocks × 2 fence lines — the "How it works" and "Static vs dynamic dispatch" self-study subsections and the "Compile-fail" exercise subsection are prose only)

If either count is wrong, the file content is off — re-check it against the content above and fix before committing.

- [ ] **Step 3: Commit**

```bash
git add lessons/13-trait-objects/README.md
git commit -m "docs(lesson-13): write self-study notes"
```

---

## Task 5: Slide deck

**Files:**
- Overwrite: `lessons/13-trait-objects/slides/slides.md`

- [ ] **Step 1: Overwrite `lessons/13-trait-objects/slides/slides.md`**

The complete file content is below, delimited by an OUTER quadruple-backtick fence (` ```` `). That outer fence is ONLY a delimiter for this plan — do NOT write it into the file. The file must start with `# Trait objects` on line 1 and contain only PLAIN triple-backtick (` ``` `) code fences.

````markdown
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
````

- [ ] **Step 2: Verify `make slides-build` succeeds and includes lesson 13**

```bash
make slides-build
test -f dist/lessons/13-trait-objects/slides/slides.md
test -f dist/lessons/13-trait-objects/slides/index.html
grep -c "13-trait-objects" dist/index.html
```

Expected: `slides.md` and `index.html` copied into dist; `grep -c "13-trait-objects"` returns at least 1. (The build-index master registry already has lesson 13 registered with slug `trait-objects`, matching this directory, so it renders as a clickable link.)

- [ ] **Step 3: Spot-check slide separators**

```bash
grep -c '^---$' lessons/13-trait-objects/slides/slides.md
```

Expected: `9` (between 10 slides).

- [ ] **Step 4: Commit**

```bash
git add lessons/13-trait-objects/slides/slides.md
git commit -m "feat(lesson-13): write slide deck"
```

---

## Task 6: End-to-end verification + push

- [ ] **Step 1: `make ci` is green**

```bash
make ci
```

Expected: exit 0. Clippy clean, fmt clean, workspace builds, default-members tests pass (now includes the 8 new tests in `trait-objects-solutions`), compile-fail `--expect broken` passes for lesson 13.

- [ ] **Step 2: `make verify LESSON=13-trait-objects` fails (the exercise is undone — intentional)**

```bash
make verify LESSON=13-trait-objects || echo "expected: exercise tests fail with todo!() panic"
```

Expected: non-zero exit. All 8 exercise tests panic with `not yet implemented`.

- [ ] **Step 3: `make slides-build` final state**

```bash
make slides-build
ls dist/lessons/
grep -c "13-trait-objects" dist/index.html
```

Expected: `dist/lessons/` contains all thirteen lessons. `grep -c "13-trait-objects"` ≥ 1.

- [ ] **Step 4: Push**

```bash
git push
```

Expected: push succeeds. CI runs and is green; Deploy rebuilds the static site so lesson 13 appears live.

- [ ] **Step 5: Smoke-test the deployed site**

After the push, wait for the Deploy workflow to finish (`gh run watch <id>`). Then:

```bash
curl -sS -o /dev/null -w "%{http_code}\n" https://rust.ristkari.dev/
curl -sS -o /dev/null -w "%{http_code}\n" https://rust.ristkari.dev/lessons/13-trait-objects/slides/
```

Expected: both return `200`.

---

## Done criteria

- `lessons/13-trait-objects/` exists with all four parts
- Both `exercises/src/lib.rs` and `solutions/src/lib.rs` define the same `Priced` trait, `Book`/`Coffee` structs and impls, and the `describe_price` / `total_price_dyn` signatures (exercise ships `todo!()` bodies for the two functions, solution ships real bodies)
- `cargo test --package trait-objects-solutions` → 8 passing tests
- `cargo test --manifest-path lessons/13-trait-objects/exercises/Cargo.toml` → compiles, 8 panicking tests (intentional)
- `cargo run --package compile-fails -- --expect broken lessons/13-trait-objects` → ok
- `cargo run --package compile-fails -- --expect compiles lessons/13-trait-objects` → fails (intentional)
- `make ci` → green
- `make slides-build` → produces `dist/lessons/13-trait-objects/slides/index.html`
- `dist/index.html` lists lesson 13 as a clickable link
- All changes committed and pushed
- Deployed site returns HTTP 200 for `/` and `/lessons/13-trait-objects/slides/`
