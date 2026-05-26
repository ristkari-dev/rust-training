# Lesson 06 — Structs & methods — Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Author the sixth (and final Phase 1) lesson of the Rust training course: a warm-up + main exercise pair on structs and methods. Warm-up is `Counter` (exercising all three receiver kinds in one tiny struct); main is `Rectangle` (multi-field struct with `new`, `area`, `is_square`). Compile-fail teaches the `&self` vs `&mut self` distinction.

**Architecture:** Use the existing `make new-lesson` scaffolder to lay down the four-part lesson structure, then overwrite the placeholder content with lesson-specific README, slides, exercises, and solutions per the design spec. Both exercise and solution crates ship the `Counter` and `Rectangle` struct declarations + `impl` block scaffolding so students fill in just the method bodies.

**Tech Stack:** Rust 2024 edition, existing tools (`new-lesson`, `compile-fails`, `slides-dev`, `build-index`), reveal.js (vendored), GNU Make.

**Spec:** [`docs/superpowers/specs/2026-05-25-lesson-06-structs-design.md`](../specs/2026-05-25-lesson-06-structs-design.md).

**Working directory:** `/Users/ristkari/code/private/rust-training`.

---

## Task 1: Scaffold lessons/06-structs

**Files (all created by the scaffolder):**
- `lessons/06-structs/README.md` (placeholder, replaced in Task 4)
- `lessons/06-structs/slides/index.html` (final — no edit needed)
- `lessons/06-structs/slides/slides.md` (placeholder, replaced in Task 5)
- `lessons/06-structs/exercises/Cargo.toml` (final — no edit needed)
- `lessons/06-structs/exercises/src/lib.rs` (placeholder, replaced in Task 2)
- `lessons/06-structs/exercises/tests/exercise.rs` (placeholder, replaced in Task 2)
- `lessons/06-structs/solutions/Cargo.toml` (final — no edit needed)
- `lessons/06-structs/solutions/src/lib.rs` (placeholder, replaced in Task 3)
- `lessons/06-structs/solutions/tests/exercise.rs` (placeholder, replaced in Task 3)

- [ ] **Step 1: Run the scaffolder**

```bash
make new-lesson NAME=06-structs
```

Expected: `scaffolded ./lessons/06-structs`.

- [ ] **Step 2: Verify directory structure**

```bash
ls lessons/06-structs/
ls lessons/06-structs/slides/ lessons/06-structs/exercises/ lessons/06-structs/solutions/
```

Expected: top-level `README.md`, `slides/`, `exercises/`, `solutions/` populated from templates.

- [ ] **Step 3: Verify Cargo package names**

```bash
grep '^name' lessons/06-structs/exercises/Cargo.toml lessons/06-structs/solutions/Cargo.toml
```

Expected:
```
lessons/06-structs/exercises/Cargo.toml:name = "structs-exercises"
lessons/06-structs/solutions/Cargo.toml:name = "structs-solutions"
```

- [ ] **Step 4: Verify the workspace picks up the new crates**

```bash
cargo metadata --no-deps --format-version 1 | grep -o '"name":"structs-[^"]*"' | sort -u
```

Expected output:
```
"name":"structs-exercises"
"name":"structs-solutions"
```

- [ ] **Step 5: Verify the scaffolded workspace builds clean**

```bash
cargo build --workspace
```

Expected: warning-free build.

- [ ] **Step 6: Commit the scaffold**

```bash
git add lessons/06-structs
git commit -m "chore: scaffold lessons/06-structs"
```

---

## Task 2: Exercise content (structs + impl scaffolding + tests + compile-fail)

**Files:**
- Overwrite: `lessons/06-structs/exercises/src/lib.rs`
- Overwrite: `lessons/06-structs/exercises/tests/exercise.rs`
- Create: `lessons/06-structs/exercises/compile_fails/06-cannot-mutate-via-shared-ref.rs`

- [ ] **Step 1: Overwrite `lessons/06-structs/exercises/src/lib.rs`**

```rust
//! Lesson 06 — exercises.
//!
//! Fill in the `todo!()` method bodies so that
//! `cargo test --manifest-path lessons/06-structs/exercises/Cargo.toml`
//! passes. The tests live in `tests/exercise.rs`.

#[derive(Debug)]
pub struct Counter {
    count: u32,
}

impl Counter {
    #[must_use]
    #[allow(clippy::new_without_default)] // Default trait is Lesson 12
    pub fn new() -> Self {
        todo!("return a Counter with count = 0")
    }

    pub fn increment(&mut self) {
        todo!("add 1 to self.count")
    }

    #[must_use]
    pub fn value(&self) -> u32 {
        todo!("return self.count")
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Rectangle {
    pub width: u32,
    pub height: u32,
}

impl Rectangle {
    #[must_use]
    pub fn new(width: u32, height: u32) -> Self {
        todo!("build a Rectangle with the given fields")
    }

    #[must_use]
    pub fn area(&self) -> u32 {
        todo!("return width * height")
    }

    #[must_use]
    pub fn is_square(&self) -> bool {
        todo!("return whether width == height")
    }
}
```

- [ ] **Step 2: Overwrite `lessons/06-structs/exercises/tests/exercise.rs`**

```rust
use structs_exercises::{Counter, Rectangle};

// Warm-up: Counter

#[test]
fn warmup_new_starts_at_zero() {
    assert_eq!(Counter::new().value(), 0);
}

#[test]
fn warmup_increment_once() {
    let mut c = Counter::new();
    c.increment();
    assert_eq!(c.value(), 1);
}

#[test]
fn warmup_increment_thrice() {
    let mut c = Counter::new();
    c.increment();
    c.increment();
    c.increment();
    assert_eq!(c.value(), 3);
}

#[test]
fn warmup_two_counters_are_independent() {
    let mut a = Counter::new();
    let mut b = Counter::new();
    a.increment();
    a.increment();
    b.increment();
    assert_eq!(a.value(), 2);
    assert_eq!(b.value(), 1);
}

// Main: Rectangle

#[test]
fn main_new_sets_fields() {
    let r = Rectangle::new(3, 5);
    assert_eq!(r.width, 3);
    assert_eq!(r.height, 5);
}

#[test]
fn main_area() {
    assert_eq!(Rectangle::new(3, 5).area(), 15);
}

#[test]
fn main_is_square_true() {
    assert!(Rectangle::new(7, 7).is_square());
}

#[test]
fn main_is_square_false() {
    assert!(!Rectangle::new(3, 5).is_square());
}
```

- [ ] **Step 3: Create `lessons/06-structs/exercises/compile_fails/06-cannot-mutate-via-shared-ref.rs`**

```rust
// Compile-fail exercise: this file MUST NOT compile until you fix it.
//
// Method receivers come in three kinds:
//   - `&self`     — borrow for reading
//   - `&mut self` — borrow for mutation
//   - `self`      — take ownership
//
// The `increment` method below tries to modify `self.count`, but its
// receiver is `&self` — a read-only borrow. The compiler refuses.
//
// Hint: read the rustc error. It will mention "cannot assign" and
// "behind a `&` reference". The fix is to change the receiver from
// `&self` to `&mut self`.

struct Counter {
    count: u32,
}

impl Counter {
    fn new() -> Self {
        Counter { count: 0 }
    }

    fn increment(&self) {
        self.count += 1;
    }

    fn value(&self) -> u32 {
        self.count
    }
}

fn main() {
    let mut c = Counter::new();
    c.increment();
    println!("{}", c.value());
}
```

- [ ] **Step 4: Verify exercise tests fail with `todo!()` panic (intentional)**

```bash
cargo test --manifest-path lessons/06-structs/exercises/Cargo.toml
```

Expected: all 8 tests fail with `not yet implemented` panic message.

- [ ] **Step 5: Verify the exercises crate still builds cleanly**

```bash
cargo build --package structs-exercises
```

Expected: warning-free build.

- [ ] **Step 6: Verify compile-fail ships broken**

```bash
cargo run --package compile-fails -- --expect broken lessons/06-structs
```

Expected: `ok: lessons/06-structs/exercises/compile_fails/06-cannot-mutate-via-shared-ref.rs` and exit 0.

- [ ] **Step 7: Verify compile-fail's student-mode check fires**

```bash
cargo run --package compile-fails -- --expect compiles lessons/06-structs
```

Expected: non-zero exit with `FAIL: file did not compile, but was expected to: lessons/06-structs/...`.

- [ ] **Step 8: Verify lint passes on the exercises crate**

```bash
cargo clippy --package structs-exercises --all-targets -- -D warnings
cargo fmt --check --package structs-exercises
```

Expected: both exit 0.

- [ ] **Step 9: Commit**

```bash
git add lessons/06-structs/exercises
git commit -m "feat(lesson-06): add struct scaffolding, exercise stubs, tests, and compile-fail"
```

---

## Task 3: Reference solutions

**Files:**
- Overwrite: `lessons/06-structs/solutions/src/lib.rs`
- Overwrite: `lessons/06-structs/solutions/tests/exercise.rs`

- [ ] **Step 1: Overwrite `lessons/06-structs/solutions/src/lib.rs`**

```rust
//! Lesson 06 — reference solutions.

#[derive(Debug)]
pub struct Counter {
    count: u32,
}

impl Counter {
    #[must_use]
    #[allow(clippy::new_without_default)] // Default trait is Lesson 12
    pub fn new() -> Self {
        Counter { count: 0 }
    }

    pub fn increment(&mut self) {
        self.count += 1;
    }

    #[must_use]
    pub fn value(&self) -> u32 {
        self.count
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Rectangle {
    pub width: u32,
    pub height: u32,
}

impl Rectangle {
    #[must_use]
    pub fn new(width: u32, height: u32) -> Self {
        Rectangle { width, height }
    }

    #[must_use]
    pub fn area(&self) -> u32 {
        self.width * self.height
    }

    #[must_use]
    pub fn is_square(&self) -> bool {
        self.width == self.height
    }
}
```

> Pedagogical notes:
> - The `Counter` impl uses `&mut self` for `increment` and `&self` for `value`, demonstrating the read/write receiver split.
> - The `Rectangle::new` constructor uses the field-init shorthand `Rectangle { width, height }`.
> - The single `#[allow(clippy::new_without_default)]` is the same one as in the exercises crate.

- [ ] **Step 2: Overwrite `lessons/06-structs/solutions/tests/exercise.rs`**

```rust
use structs_solutions::{Counter, Rectangle};

// Warm-up: Counter

#[test]
fn warmup_new_starts_at_zero() {
    assert_eq!(Counter::new().value(), 0);
}

#[test]
fn warmup_increment_once() {
    let mut c = Counter::new();
    c.increment();
    assert_eq!(c.value(), 1);
}

#[test]
fn warmup_increment_thrice() {
    let mut c = Counter::new();
    c.increment();
    c.increment();
    c.increment();
    assert_eq!(c.value(), 3);
}

#[test]
fn warmup_two_counters_are_independent() {
    let mut a = Counter::new();
    let mut b = Counter::new();
    a.increment();
    a.increment();
    b.increment();
    assert_eq!(a.value(), 2);
    assert_eq!(b.value(), 1);
}

// Main: Rectangle

#[test]
fn main_new_sets_fields() {
    let r = Rectangle::new(3, 5);
    assert_eq!(r.width, 3);
    assert_eq!(r.height, 5);
}

#[test]
fn main_area() {
    assert_eq!(Rectangle::new(3, 5).area(), 15);
}

#[test]
fn main_is_square_true() {
    assert!(Rectangle::new(7, 7).is_square());
}

#[test]
fn main_is_square_false() {
    assert!(!Rectangle::new(3, 5).is_square());
}
```

- [ ] **Step 3: Verify solution tests pass**

```bash
cargo test --package structs-solutions
```

Expected: 8 tests pass.

- [ ] **Step 4: Verify lint passes on the solutions crate**

```bash
cargo clippy --package structs-solutions --all-targets -- -D warnings
cargo fmt --check --package structs-solutions
```

Expected: both exit 0. The single `#[allow(clippy::new_without_default)]` on `Counter::new()` is the only allow needed. If any other clippy lint fires, STOP and report.

- [ ] **Step 5: Commit**

```bash
git add lessons/06-structs/solutions
git commit -m "feat(lesson-06): add reference solutions"
```

---

## Task 4: Lesson README

**Files:**
- Overwrite: `lessons/06-structs/README.md`

- [ ] **Step 1: Overwrite `lessons/06-structs/README.md`**

The complete file content (write this as the file's actual content, starting with the `# Lesson 06` heading). Code fences inside the markdown are plain triple-backticks.

```markdown
# Lesson 06 — Structs & methods

Methods aren't magic — they're functions in an `impl` block. Once
you've seen the grammar, the dot-syntax you've been using since Lesson
02 (`.push_str()`, `.powi()`, and so on) makes complete sense. Today
you'll define your own struct, attach methods to it, and write your
own `::new(...)` constructor. By the end you'll have everything Phase
1 set out to give you.

## Learning goals

- Define a custom `struct` with named fields and the standard derives
  (`Debug`, `PartialEq`, `Eq`)
- Construct a struct via struct-literal syntax and access fields with
  the dot operator
- Write methods inside an `impl` block using the three receiver kinds:
  `&self` for reading, `&mut self` for mutating, `self` for consuming
- Write an associated function — typically `pub fn new(...) -> Self` —
  and call it via `Type::new(...)`
- Recognize that the dot-syntax used in earlier lessons is method-call
  syntax and follows the same rules

## Self-study notes

### Defining a struct

A struct is a **product type** — a fixed bundle of named fields, each
with its own type:

​```rust
#[derive(Debug, PartialEq, Eq)]
struct Rectangle {
    width: u32,
    height: u32,
}
​```

The derives are the same machinery you saw with enums in Lesson 05:
they give you printing (`Debug`) and equality comparison (`PartialEq`
/ `Eq`) for free.

By default the struct and its fields are private to the module they're
declared in. To make either visible to the rest of the world, prefix
with `pub`. The exercises in this lesson use both — `Counter`'s field
is private (encapsulation), `Rectangle`'s fields are public (plain
data).

### Constructing and reading fields

Construct a struct with a struct-literal, naming every field:

​```rust
let rect = Rectangle { width: 10, height: 20 };
let area = rect.width * rect.height;  // 200
​```

When a local variable has the same name as a field, Rust lets you
abbreviate with the **field-init shorthand**:

​```rust
let width = 10;
let height = 20;
let rect = Rectangle { width, height };  // same as { width: width, height: height }
​```

### `impl` blocks

Methods and associated functions go in an `impl` block:

​```rust
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}
​```

`impl Rectangle` says "the following items belong to `Rectangle`." A
type can have multiple `impl` blocks (useful for organizing by trait,
later) but conventionally you have one.

### Method receivers — `&self`, `&mut self`, `self`

A method is a function whose first parameter is one of three receiver
forms:

​```rust
impl Rectangle {
    // Read-only: borrows self
    fn area(&self) -> u32 {
        self.width * self.height
    }

    // Mutating: borrows self mutably
    fn double_width(&mut self) {
        self.width *= 2;
    }

    // Consuming: takes self by value
    fn into_parts(self) -> (u32, u32) {
        (self.width, self.height)
    }
}
​```

Treat the three kinds as grammar for now:

- `&self` — read the fields
- `&mut self` — modify the fields (caller must hold the struct in a
  `let mut` binding)
- `self` — consume the value (the struct is gone after the call
  returns)

The deep "why" — what `&` and `&mut` and ownership *actually mean* —
lands in **Lesson 07** (the start of Phase 2). For Lesson 06 you just
need to pick the right receiver for what your method does.

### Associated functions and the `new` convention

A function in an `impl` block that has no `self` receiver is an
**associated function**. It's called via `Type::name(...)` rather than
`value.name(...)`:

​```rust
impl Rectangle {
    pub fn new(width: u32, height: u32) -> Self {
        Rectangle { width, height }
    }
}

let rect = Rectangle::new(3, 5);
​```

`Self` (capital S) is shorthand for the surrounding type — `Self`
inside `impl Rectangle` means `Rectangle`. Using `Self` rather than the
name explicitly makes constructors easier to rename later.

The name `new` is convention, not a keyword. You've been calling
`String::new()`, `String::from(...)`, and `Some(...)` (which is
technically a tuple-struct constructor) since Lesson 04. Now you know
how to write your own.

## Exercises

### Warm-up: `Counter`

The exercises crate ships a `Counter` struct with one private field
(`count: u32`) and an `impl` block with three stub methods:

- `Counter::new()` — associated function, returns a counter with
  `count = 0`
- `counter.increment()` — takes `&mut self`, adds 1 to `count`
- `counter.value()` — takes `&self`, returns the current `count`

Fill in the three `todo!()` bodies. All three receiver kinds appear in
this tiny struct.

### Main: `Rectangle`

The exercises crate also ships a `Rectangle` struct with two public
fields (`width: u32`, `height: u32`) and three stub methods:

- `Rectangle::new(width, height)` — associated function, builds and
  returns a `Rectangle`
- `rect.area()` — takes `&self`, returns `width * height`
- `rect.is_square()` — takes `&self`, returns whether
  `width == height`

Use the field-init shorthand `Rectangle { width, height }` in `new` —
it's the idiomatic spelling when the local names match the field
names.

### Compile-fail

`exercises/compile_fails/06-cannot-mutate-via-shared-ref.rs` ships
with a method that takes `&self` but tries to mutate a field. rustc's
error names the receiver kind directly. The fix is a one-word
insertion.

### Run

​```bash
make verify LESSON=06-structs
​```

This runs your exercise tests and asserts the compile-fail file now
compiles.

## Solutions

See `solutions/src/lib.rs` for the reference implementations. Try the
exercises before peeking.
```

> **IMPORTANT for the implementer:** the code-fence escape in the markdown above uses an invisible zero-width character (shown as `​```) in front of each triple-backtick block — that's only there so this plan file can nest fenced markdown inside an outer fenced markdown block. When you write the actual `README.md`, every fence must be three PLAIN backticks `` ``` `` with NO leading invisible character. After writing, `grep -c '^```' lessons/06-structs/README.md` should return 14 (7 code blocks × 2 fence lines).

- [ ] **Step 2: Spot-check the README**

```bash
head -1 lessons/06-structs/README.md
grep -c '^### ' lessons/06-structs/README.md
grep -c '^```' lessons/06-structs/README.md
```

Expected:
- First line: `# Lesson 06 — Structs & methods`
- `grep -c '^### '` returns 9 (five subsections under self-study + four under exercises)
- `grep -c '^```'` returns 14 (7 code blocks × 2 fence lines)

- [ ] **Step 3: Commit**

```bash
git add lessons/06-structs/README.md
git commit -m "docs(lesson-06): write self-study notes"
```

---

## Task 5: Slide deck

**Files:**
- Overwrite: `lessons/06-structs/slides/slides.md`

- [ ] **Step 1: Overwrite `lessons/06-structs/slides/slides.md`**

The complete file content (write this as the file's actual content, starting with the `# Structs & methods` heading):

````
# Structs & methods

> Methods aren't magic — they're functions in an `impl` block. Once you've seen the grammar, the dot-syntax you've been using since Lesson 02 makes complete sense.

---

## Recap

Phase 1 so far: values & types (L01-02), control flow (L03), compound types (L04), sum types & match (L05).

Today: the final foundation — **product types** (named fields, fixed shape) and the methods attached to them.

---

## Defining a struct

```rust
#[derive(Debug, PartialEq, Eq)]
struct Rectangle {
    width: u32,
    height: u32,
}
```

- Named fields with explicit types
- `pub` on the struct or fields controls visibility
- Derive `Debug` to print, `PartialEq`/`Eq` to compare (same machinery as Lesson 05)

---

## Constructing and reading

```rust
let rect = Rectangle { width: 10, height: 20 };
let area = rect.width * rect.height;          // 200
```

The struct-literal must name **every** field.

When a local has the same name as the field, use the **shorthand**:

```rust
let width = 10;
let height = 20;
let rect = Rectangle { width, height };       // same as { width: width, height: height }
```

---

## `impl` blocks

```rust
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}
```

- `impl Type { ... }` is where methods (and associated functions) live
- A type can have multiple `impl` blocks, but conventionally one
- The block is attached by name — `impl Rectangle` says "these methods belong to Rectangle"

---

## `&self` methods

```rust
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn is_square(&self) -> bool {
        self.width == self.height
    }
}

let rect = Rectangle { width: 7, height: 7 };
rect.area();        // 49
rect.is_square();   // true
```

- `&self` borrows the struct for **reading**
- Called via dot-syntax: `rect.area()`
- `self.field` reaches the field through the receiver

---

## `&mut self` and `self`

```rust
impl Rectangle {
    fn double_width(&mut self) {
        self.width *= 2;        // &mut self lets us mutate
    }
}

let mut rect = Rectangle { width: 5, height: 10 };
rect.double_width();            // rect.width is now 10
```

The three receiver kinds:

- `&self`     — borrow for reading
- `&mut self` — borrow for mutating
- `self`      — take ownership (consumes the value)

The "why" behind these lands in **Lesson 07: Ownership & moves**. For now: read, modify, consume.

---

## Associated functions

A function in an `impl` block with **no `self` receiver** — called via `Type::name(...)`:

```rust
impl Rectangle {
    pub fn new(width: u32, height: u32) -> Self {
        Rectangle { width, height }
    }
}

let rect = Rectangle::new(3, 5);
```

- `Self` is shorthand for the surrounding type
- `new` is the conventional constructor name — not a language requirement
- You've been calling `String::new()`, `String::from(...)` since Lesson 04. Now you know how to write your own.

---

## Putting it together

The main exercise: implement methods on `Rectangle`:

```rust
impl Rectangle {
    pub fn new(width: u32, height: u32) -> Self {
        Rectangle { width, height }
    }

    pub fn area(&self) -> u32 {
        self.width * self.height
    }

    pub fn is_square(&self) -> bool {
        self.width == self.height
    }
}
```

The compile-fail exercise: write `increment(&self) { self.count += 1; }` — the compiler will tell you exactly why it doesn't work.

---

## Wrap — Phase 1 complete

- **Structs** are product types — named fields, fixed shape
- **`impl` blocks** hold methods and associated functions
- **`&self`** reads, **`&mut self`** mutates, **`self`** consumes
- **`Type::new(...)`** is the constructor convention
- Methods are functions; the dot-syntax is just sugar

**Phase 1 complete.** You've now built up: values & types, control flow, compound types, sum types, product types.

Next: **Phase 2** opens with **Lesson 07 — Ownership & moves**.
````

The OUTER fence above (quadruple backticks) is just delimiting this content in the plan. The FILE you write should NOT contain the outer fence — only the inner triple-backtick `rust` blocks.

The file should:
- Start with `# Structs & methods` on line 1
- Have exactly 9 `---` slide separators (between 10 slides)
- Contain 8 triple-backtick `rust` code fences

- [ ] **Step 2: Verify `make slides-build` succeeds and includes lesson 06**

```bash
make slides-build
test -f dist/lessons/06-structs/slides/slides.md
test -f dist/lessons/06-structs/slides/index.html
grep -c "06-structs" dist/index.html
```

Expected: `slides.md` and `index.html` copied into dist; `grep -c "06-structs"` returns at least 1.

- [ ] **Step 3: Spot-check slide separators**

```bash
grep -c '^---$' lessons/06-structs/slides/slides.md
```

Expected: `9` (between 10 slides).

- [ ] **Step 4: Commit**

```bash
git add lessons/06-structs/slides/slides.md
git commit -m "feat(lesson-06): write slide deck"
```

---

## Task 6: End-to-end verification + push

- [ ] **Step 1: `make ci` is green**

```bash
make ci
```

Expected: exit 0. Clippy clean, fmt clean, workspace builds, default-members tests pass (now includes the 8 new tests in `structs-solutions`), compile-fail `--expect broken` passes for lesson 06.

- [ ] **Step 2: `make verify LESSON=06-structs` fails (the exercise is undone — intentional)**

```bash
make verify LESSON=06-structs || echo "expected: exercise tests fail with todo!() panic"
```

Expected: non-zero exit. All 8 exercise tests panic with `not yet implemented`.

- [ ] **Step 3: `make slides-build` final state**

```bash
make slides-build
ls dist/lessons/
grep -c "06-structs" dist/index.html
```

Expected: `dist/lessons/` contains all six lessons (01-hello-rust through 06-structs). `grep -c "06-structs"` ≥ 1.

- [ ] **Step 4: Push**

```bash
git push
```

Expected: push succeeds. CI runs and is green; Deploy rebuilds the static site so lesson 06 appears live.

- [ ] **Step 5: Smoke-test the deployed site**

After the push, wait for the Deploy workflow to finish (`gh run watch <id>`). Then:

```bash
curl -sS -o /dev/null -w "%{http_code}\n" https://rust.ristkari.dev/
curl -sS -o /dev/null -w "%{http_code}\n" https://rust.ristkari.dev/lessons/06-structs/slides/
```

Expected: both return `200`.

---

## Done criteria

- `lessons/06-structs/` exists with all four parts
- Both `exercises/src/lib.rs` and `solutions/src/lib.rs` define `Counter` and `Rectangle` with the same signatures
- `cargo test --package structs-solutions` → 8 passing tests
- `cargo test --manifest-path lessons/06-structs/exercises/Cargo.toml` → 8 panicking tests (intentional)
- `cargo run --package compile-fails -- --expect broken lessons/06-structs` → ok
- `cargo run --package compile-fails -- --expect compiles lessons/06-structs` → fails (intentional)
- `make ci` → green
- `make slides-build` → produces `dist/lessons/06-structs/slides/index.html`
- `dist/index.html` lists lesson 06 as a clickable link
- All changes committed and pushed
- Deployed site returns HTTP 200 for `/` and `/lessons/06-structs/slides/`

**Phase 1 complete after this lesson lands.**
