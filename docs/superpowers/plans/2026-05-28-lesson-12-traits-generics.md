# Lesson 12 — Traits & generics — Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Author the first lesson of Phase 3 of the Rust training course: traits & generics. The through-line is trait → generic bound. Warm-up: implement a `Priced` trait (required `price` + default `is_free`) for `Book` and `Coffee`. Main: a generic `total_price<T: Priced>(&[T]) -> u32`. Compile-fail: a generic function missing its `T: Priced` bound (E0599).

**Architecture:** Use the existing `make new-lesson` scaffolder. The exercise crate ships the `Priced` trait, the `Book`/`Coffee` structs, and the `impl Priced` skeletons with `todo!()` bodies — students fill in the bodies. Shipping the skeletons keeps the crate and its tests compiling in the undone state (tests panic at runtime, like every prior lesson). All reference code in this plan was empirically verified clippy-pedantic-clean during design.

**Tech Stack:** Rust 2024 edition, existing tools (`new-lesson`, `compile-fails`, `slides-dev`, `build-index`), reveal.js (vendored), GNU Make.

**Spec:** [`docs/superpowers/specs/2026-05-28-lesson-12-traits-generics-design.md`](../specs/2026-05-28-lesson-12-traits-generics-design.md).

**Working directory:** `/Users/ristkari/code/private/rust-training`.

---

## Task 1: Scaffold lessons/12-traits-generics

**Files (all created by the scaffolder):**
- `lessons/12-traits-generics/README.md` (placeholder, replaced in Task 4)
- `lessons/12-traits-generics/slides/index.html` (final — no edit needed)
- `lessons/12-traits-generics/slides/slides.md` (placeholder, replaced in Task 5)
- `lessons/12-traits-generics/exercises/Cargo.toml` (final — no edit needed)
- `lessons/12-traits-generics/exercises/src/lib.rs` (placeholder, replaced in Task 2)
- `lessons/12-traits-generics/exercises/tests/exercise.rs` (placeholder, replaced in Task 2)
- `lessons/12-traits-generics/solutions/Cargo.toml` (final — no edit needed)
- `lessons/12-traits-generics/solutions/src/lib.rs` (placeholder, replaced in Task 3)
- `lessons/12-traits-generics/solutions/tests/exercise.rs` (placeholder, replaced in Task 3)

- [ ] **Step 1: Run the scaffolder**

```bash
make new-lesson NAME=12-traits-generics
```

Expected: `scaffolded ./lessons/12-traits-generics`.

- [ ] **Step 2: Verify directory structure**

```bash
ls lessons/12-traits-generics/
ls lessons/12-traits-generics/slides/ lessons/12-traits-generics/exercises/ lessons/12-traits-generics/solutions/
```

Expected: top-level `README.md`, `slides/`, `exercises/`, `solutions/` populated from templates.

- [ ] **Step 3: Verify Cargo package names**

```bash
grep '^name' lessons/12-traits-generics/exercises/Cargo.toml lessons/12-traits-generics/solutions/Cargo.toml
```

Expected:
```
lessons/12-traits-generics/exercises/Cargo.toml:name = "traits-generics-exercises"
lessons/12-traits-generics/solutions/Cargo.toml:name = "traits-generics-solutions"
```

- [ ] **Step 4: Verify the workspace picks up the new crates**

```bash
cargo metadata --no-deps --format-version 1 | grep -o '"name":"traits-generics-[^"]*"' | sort -u
```

Expected output:
```
"name":"traits-generics-exercises"
"name":"traits-generics-solutions"
```

- [ ] **Step 5: Verify the scaffolded workspace builds clean**

```bash
cargo build --workspace
```

Expected: warning-free build.

- [ ] **Step 6: Commit the scaffold**

```bash
git add lessons/12-traits-generics
git commit -m "chore: scaffold lessons/12-traits-generics"
```

---

## Task 2: Exercise content (stubs + tests + compile-fail)

**Files:**
- Overwrite: `lessons/12-traits-generics/exercises/src/lib.rs`
- Overwrite: `lessons/12-traits-generics/exercises/tests/exercise.rs`
- Create: `lessons/12-traits-generics/exercises/compile_fails/12-missing-trait-bound.rs`

- [ ] **Step 1: Overwrite `lessons/12-traits-generics/exercises/src/lib.rs`**

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

- [ ] **Step 2: Overwrite `lessons/12-traits-generics/exercises/tests/exercise.rs`**

```rust
use traits_generics_exercises::{Book, Coffee, Priced, total_price};

// Warm-up: implement Priced for Book and Coffee

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

// Main: total_price

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

- [ ] **Step 3: Create `lessons/12-traits-generics/exercises/compile_fails/12-missing-trait-bound.rs`**

The `compile_fails/` directory does not exist yet — create it. Write this file:

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

- [ ] **Step 4: Verify exercise tests compile and fail with `todo!()` panic (intentional)**

```bash
cargo test --manifest-path lessons/12-traits-generics/exercises/Cargo.toml
```

Expected: the crate COMPILES, then all 8 tests FAIL with `not yet implemented` panic message. (Compilation succeeding while tests panic is the correct undone state — the `impl` skeletons exist so the methods resolve; only their bodies are `todo!()`.)

- [ ] **Step 5: Verify the exercises crate still builds cleanly**

```bash
cargo build --package traits-generics-exercises
```

Expected: warning-free build.

- [ ] **Step 6: Verify compile-fail ships broken**

```bash
cargo run --package compile-fails -- --expect broken lessons/12-traits-generics
```

Expected: prints `ok: lessons/12-traits-generics/exercises/compile_fails/12-missing-trait-bound.rs` and exits 0. (The tool printing the rustc E0599 error text is expected — what matters is the final `ok:` line and exit 0.)

- [ ] **Step 7: Verify compile-fail's student-mode check fires**

```bash
cargo run --package compile-fails -- --expect compiles lessons/12-traits-generics
```

Expected: non-zero exit with a `FAIL: file did not compile, but was expected to: lessons/12-traits-generics/...` message. (This is correct — the file ships broken on purpose.)

- [ ] **Step 8: Verify lint passes on the exercises crate**

```bash
cargo clippy --package traits-generics-exercises --all-targets -- -D warnings
cargo fmt --check --package traits-generics-exercises
```

Expected: both exit 0. (`clippy::unused_self` does not fire on the `todo!()` trait-impl bodies — verified during design.)

- [ ] **Step 9: Commit**

```bash
git add lessons/12-traits-generics/exercises
git commit -m "feat(lesson-12): add trait, structs, exercise stubs, tests, and compile-fail"
```

---

## Task 3: Reference solutions

**Files:**
- Overwrite: `lessons/12-traits-generics/solutions/src/lib.rs`
- Overwrite: `lessons/12-traits-generics/solutions/tests/exercise.rs`

- [ ] **Step 1: Overwrite `lessons/12-traits-generics/solutions/src/lib.rs`**

```rust
//! Lesson 12 — reference solutions.

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
pub fn total_price<T: Priced>(items: &[T]) -> u32 {
    items.iter().map(Priced::price).sum()
}
```

> Pedagogical notes:
> - `Book::price` returns a field directly; `Coffee::price` computes from `shots`. Two different implementations of the same trait. The `is_free` default method then works on top of either `price` without the implementor writing it.
> - `total_price<T: Priced>` is generic: the `T: Priced` bound is what lets the body call `.price()` on each item. `items.iter().map(Priced::price).sum()` reinforces lesson 11's iterators.
> - `Priced::price` is the trait method used as a function value. Do NOT write `.map(|item| item.price())` — that closure form trips `clippy::redundant_closure_for_method_calls` (a denied pedantic lint). The method-path form is both cleaner and lint-clean (verified during design).
> - No `#[allow]` attributes should be needed. If clippy fires unexpectedly, fix the code rather than adding an allow, and report the deviation.

- [ ] **Step 2: Overwrite `lessons/12-traits-generics/solutions/tests/exercise.rs`**

```rust
use traits_generics_solutions::{Book, Coffee, Priced, total_price};

// Warm-up: implement Priced for Book and Coffee

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

// Main: total_price

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

- [ ] **Step 3: Verify solution tests pass**

```bash
cargo test --package traits-generics-solutions
```

Expected: 8 tests pass.

- [ ] **Step 4: Verify lint passes on the solutions crate**

```bash
cargo clippy --package traits-generics-solutions --all-targets -- -D warnings
cargo fmt --check --package traits-generics-solutions
```

Expected: both exit 0. No `#[allow]` attributes needed. If clippy fires on anything, fix the code (not with an allow) and report it.

- [ ] **Step 5: Commit**

```bash
git add lessons/12-traits-generics/solutions
git commit -m "feat(lesson-12): add reference solutions"
```

---

## Task 4: Lesson README

**Files:**
- Overwrite: `lessons/12-traits-generics/README.md`

- [ ] **Step 1: Overwrite `lessons/12-traits-generics/README.md`**

The complete file content is below, delimited by an OUTER quadruple-backtick fence (` ```` `). That outer fence is ONLY a delimiter for this plan — do NOT write it into the file. The file must start with `# Lesson 12` on line 1 and contain only PLAIN triple-backtick (` ``` `) code fences.

````markdown
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
````

- [ ] **Step 2: Spot-check the README**

```bash
head -1 lessons/12-traits-generics/README.md
grep -c '^### ' lessons/12-traits-generics/README.md
grep -c '^```' lessons/12-traits-generics/README.md
```

Expected:
- First line: `# Lesson 12 — Traits & generics`
- `grep -c '^### '` returns `9` (five subsections under self-study + four under exercises)
- `grep -c '^```'` returns `16` (8 code blocks × 2 fence lines)

If either count is wrong, the file content is off — re-check it against the content above and fix before committing.

- [ ] **Step 3: Commit**

```bash
git add lessons/12-traits-generics/README.md
git commit -m "docs(lesson-12): write self-study notes"
```

---

## Task 5: Slide deck

**Files:**
- Overwrite: `lessons/12-traits-generics/slides/slides.md`

- [ ] **Step 1: Overwrite `lessons/12-traits-generics/slides/slides.md`**

The complete file content is below, delimited by an OUTER quadruple-backtick fence (` ```` `). That outer fence is ONLY a delimiter for this plan — do NOT write it into the file. The file must start with `# Traits & generics` on line 1 and contain only PLAIN triple-backtick (` ``` `) code fences.

````markdown
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
````

- [ ] **Step 2: Verify `make slides-build` succeeds and includes lesson 12**

```bash
make slides-build
test -f dist/lessons/12-traits-generics/slides/slides.md
test -f dist/lessons/12-traits-generics/slides/index.html
grep -c "12-traits-generics" dist/index.html
```

Expected: `slides.md` and `index.html` copied into dist; `grep -c "12-traits-generics"` returns at least 1. (The build-index master registry already has lesson 12 registered with slug `traits-generics`, matching this directory, so it renders as a clickable link.)

- [ ] **Step 3: Spot-check slide separators**

```bash
grep -c '^---$' lessons/12-traits-generics/slides/slides.md
```

Expected: `9` (between 10 slides).

- [ ] **Step 4: Commit**

```bash
git add lessons/12-traits-generics/slides/slides.md
git commit -m "feat(lesson-12): write slide deck"
```

---

## Task 6: End-to-end verification + push

- [ ] **Step 1: `make ci` is green**

```bash
make ci
```

Expected: exit 0. Clippy clean, fmt clean, workspace builds, default-members tests pass (now includes the 8 new tests in `traits-generics-solutions`), compile-fail `--expect broken` passes for lesson 12.

- [ ] **Step 2: `make verify LESSON=12-traits-generics` fails (the exercise is undone — intentional)**

```bash
make verify LESSON=12-traits-generics || echo "expected: exercise tests fail with todo!() panic"
```

Expected: non-zero exit. All 8 exercise tests panic with `not yet implemented`.

- [ ] **Step 3: `make slides-build` final state**

```bash
make slides-build
ls dist/lessons/
grep -c "12-traits-generics" dist/index.html
```

Expected: `dist/lessons/` contains all twelve lessons. `grep -c "12-traits-generics"` ≥ 1.

- [ ] **Step 4: Push**

```bash
git push
```

Expected: push succeeds. CI runs and is green; Deploy rebuilds the static site so lesson 12 appears live.

- [ ] **Step 5: Smoke-test the deployed site**

After the push, wait for the Deploy workflow to finish (`gh run watch <id>`). Then:

```bash
curl -sS -o /dev/null -w "%{http_code}\n" https://rust.ristkari.dev/
curl -sS -o /dev/null -w "%{http_code}\n" https://rust.ristkari.dev/lessons/12-traits-generics/slides/
```

Expected: both return `200`.

---

## Done criteria

- `lessons/12-traits-generics/` exists with all four parts
- Both `exercises/src/lib.rs` and `solutions/src/lib.rs` define the same `Priced` trait, `Book`/`Coffee` structs, and `total_price` signature (exercise ships `todo!()` bodies, solution ships real bodies)
- `cargo test --package traits-generics-solutions` → 8 passing tests
- `cargo test --manifest-path lessons/12-traits-generics/exercises/Cargo.toml` → compiles, 8 panicking tests (intentional)
- `cargo run --package compile-fails -- --expect broken lessons/12-traits-generics` → ok
- `cargo run --package compile-fails -- --expect compiles lessons/12-traits-generics` → fails (intentional)
- `make ci` → green
- `make slides-build` → produces `dist/lessons/12-traits-generics/slides/index.html`
- `dist/index.html` lists lesson 12 as a clickable link
- All changes committed and pushed
- Deployed site returns HTTP 200 for `/` and `/lessons/12-traits-generics/slides/`
