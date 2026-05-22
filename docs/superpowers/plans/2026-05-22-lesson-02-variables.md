# Lesson 02 — Variables, types, mutability — Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Author the second lesson of the Rust training course: a warm-up + main exercise pair on `let`/`mut`/shadowing/type inference/annotations/`const`, with one compile-fail exercise on the `mut`-can't-change-type misconception.

**Architecture:** Use the existing `make new-lesson` scaffolder to lay down the four-part lesson structure, then overwrite the placeholder content with lesson-specific README, slides, exercises, and solutions per the design spec. The workspace `members` glob picks the new crates up automatically.

**Tech Stack:** Rust 2024 edition, Cargo workspaces (existing tools: `new-lesson`, `compile-fails`, `slides-dev`, `build-index`), reveal.js (vendored), GNU Make.

**Spec:** [`docs/superpowers/specs/2026-05-22-lesson-02-variables-design.md`](../specs/2026-05-22-lesson-02-variables-design.md).

**Working directory:** `/Users/ristkari/code/private/rust-training`.

---

## Task 1: Scaffold lessons/02-variables

Use the existing tool to create the lesson skeleton from `templates/`.

**Files:**
- Create: `lessons/02-variables/README.md` (placeholder, replaced in Task 4)
- Create: `lessons/02-variables/slides/index.html` (final — no edit needed)
- Create: `lessons/02-variables/slides/slides.md` (placeholder, replaced in Task 5)
- Create: `lessons/02-variables/exercises/Cargo.toml` (final — no edit needed)
- Create: `lessons/02-variables/exercises/src/lib.rs` (placeholder, replaced in Task 2)
- Create: `lessons/02-variables/exercises/tests/exercise.rs` (placeholder, replaced in Task 2)
- Create: `lessons/02-variables/solutions/Cargo.toml` (final — no edit needed)
- Create: `lessons/02-variables/solutions/src/lib.rs` (placeholder, replaced in Task 3)
- Create: `lessons/02-variables/solutions/tests/exercise.rs` (placeholder, replaced in Task 3)

- [ ] **Step 1: Run the scaffolder**

```bash
make new-lesson NAME=02-variables
```

Expected: `scaffolded ./lessons/02-variables`.

- [ ] **Step 2: Verify directory structure**

```bash
ls lessons/02-variables/
ls lessons/02-variables/slides/ lessons/02-variables/exercises/ lessons/02-variables/solutions/
```

Expected: top-level `README.md`, `slides/`, `exercises/`, `solutions/`. Each subdirectory populated from templates.

- [ ] **Step 3: Verify Cargo package names**

```bash
grep '^name' lessons/02-variables/exercises/Cargo.toml lessons/02-variables/solutions/Cargo.toml
```

Expected:
```
lessons/02-variables/exercises/Cargo.toml:name = "variables-exercises"
lessons/02-variables/solutions/Cargo.toml:name = "variables-solutions"
```

- [ ] **Step 4: Verify the workspace picks up the new crates**

```bash
cargo metadata --no-deps --format-version 1 | grep -o '"name":"variables-[^"]*"' | sort -u
```

Expected output:
```
"name":"variables-exercises"
"name":"variables-solutions"
```

- [ ] **Step 5: Verify the scaffolded workspace builds clean**

```bash
cargo build --workspace
```

Expected: warning-free build (the template stub uses `todo!()` and `_a`/`_b` underscore params so it compiles).

- [ ] **Step 6: Commit the scaffold**

```bash
git add lessons/02-variables
git commit -m "chore: scaffold lessons/02-variables"
```

---

## Task 2: Exercise content (stubs + tests + compile-fail)

**Files:**
- Overwrite: `lessons/02-variables/exercises/src/lib.rs`
- Overwrite: `lessons/02-variables/exercises/tests/exercise.rs`
- Create: `lessons/02-variables/exercises/compile_fails/02-mut-cant-change-type.rs`

- [ ] **Step 1: Overwrite `lessons/02-variables/exercises/src/lib.rs`**

```rust
//! Lesson 02 — exercises.
//!
//! Implement `fahrenheit_to_celsius` (warm-up) and `compound_interest`
//! (main) so that `cargo test --manifest-path
//! lessons/02-variables/exercises/Cargo.toml` passes. The tests live in
//! `tests/exercise.rs`.

#[must_use]
pub fn fahrenheit_to_celsius(_f: f64) -> f64 {
    todo!("convert Fahrenheit to Celsius using (f - 32) * 5 / 9")
}

#[must_use]
pub fn compound_interest(_principal: f64, _rate_percent: f64, _years: u32) -> f64 {
    todo!("return principal * (1 + rate_percent/100)^years")
}
```

- [ ] **Step 2: Overwrite `lessons/02-variables/exercises/tests/exercise.rs`**

```rust
use variables_exercises::{compound_interest, fahrenheit_to_celsius};

// Warm-up: fahrenheit_to_celsius

#[test]
fn warmup_freezing() {
    assert_eq!(fahrenheit_to_celsius(32.0), 0.0);
}

#[test]
fn warmup_boiling() {
    assert_eq!(fahrenheit_to_celsius(212.0), 100.0);
}

#[test]
fn warmup_ten_celsius() {
    assert_eq!(fahrenheit_to_celsius(50.0), 10.0);
}

// Main: compound_interest

#[test]
fn main_zero_principal_grows_to_zero() {
    assert_eq!(compound_interest(0.0, 50.0, 10), 0.0);
}

#[test]
fn main_zero_rate_returns_principal() {
    assert_eq!(compound_interest(1000.0, 0.0, 5), 1000.0);
}

#[test]
fn main_fifty_percent_two_years() {
    assert_eq!(compound_interest(100.0, 50.0, 2), 225.0);
}

#[test]
fn main_twentyfive_percent_two_years() {
    assert_eq!(compound_interest(200.0, 25.0, 2), 312.5);
}
```

- [ ] **Step 3: Create `lessons/02-variables/exercises/compile_fails/02-mut-cant-change-type.rs`**

```rust
// Compile-fail exercise: this file MUST NOT compile until you fix it.
//
// `mut` lets you change a binding's VALUE, but not its TYPE. The line
// below tries to reassign `x` from an integer to a string slice. Read
// the rustc error: it will name the original type that Rust inferred
// and point at the offending assignment.
//
// Hint: the fix is NOT to add `mut` (we already have it). The fix is
// to use shadowing — replace the second line so it declares a NEW
// binding called `x` with `let`. The new `x` can have a different type
// because it's a separate binding that happens to reuse the name.

fn main() {
    let mut x = 5;
    x = "hello";
    println!("{x}");
}
```

- [ ] **Step 4: Verify exercise tests fail with `todo!()` panic (intentional)**

```bash
cargo test --manifest-path lessons/02-variables/exercises/Cargo.toml
```

Expected: all 7 tests fail with `not yet implemented` panic message. This is the intentional shipped state.

- [ ] **Step 5: Verify the exercises crate still builds cleanly**

```bash
cargo build --package variables-exercises
```

Expected: warning-free build (the `#[must_use]` attribute and the `_`-prefixed parameter names keep clippy happy).

- [ ] **Step 6: Verify compile-fail ships broken (the author/CI check)**

```bash
cargo run --package compile-fails -- --expect broken lessons/02-variables
```

Expected: `ok: lessons/02-variables/exercises/compile_fails/02-mut-cant-change-type.rs` and exit 0.

- [ ] **Step 7: Verify compile-fail's student-mode check fires (the exercise hasn't been fixed yet)**

```bash
cargo run --package compile-fails -- --expect compiles lessons/02-variables
```

Expected: non-zero exit with `FAIL: file did not compile, but was expected to: lessons/02-variables/...`.

- [ ] **Step 8: Verify lint passes on the exercises crate**

```bash
cargo clippy --package variables-exercises --all-targets -- -D warnings
cargo fmt --check --package variables-exercises
```

Expected: both exit 0.

- [ ] **Step 9: Commit**

```bash
git add lessons/02-variables/exercises
git commit -m "feat(lesson-02): add warm-up + main exercise stubs, tests, and compile-fail"
```

---

## Task 3: Reference solutions

**Files:**
- Overwrite: `lessons/02-variables/solutions/src/lib.rs`
- Overwrite: `lessons/02-variables/solutions/tests/exercise.rs`

- [ ] **Step 1: Overwrite `lessons/02-variables/solutions/src/lib.rs`**

```rust
//! Lesson 02 — reference solutions.

const HUNDRED: f64 = 100.0;

#[must_use]
#[allow(clippy::let_and_return)] // pedagogical: the let+annotation pattern is the point
pub fn fahrenheit_to_celsius(f: f64) -> f64 {
    // Shadowing the parameter binding — not mutation.
    let f = f - 32.0;
    // Annotation is redundant (inference works) but spells out the type.
    let scaled: f64 = f * 5.0 / 9.0;
    scaled
}

#[must_use]
#[allow(
    clippy::cast_possible_truncation,
    clippy::cast_possible_wrap,
    clippy::cast_sign_loss
)] // u32 -> i32 for powi: safe for realistic year counts
pub fn compound_interest(principal: f64, rate_percent: f64, years: u32) -> f64 {
    let rate = rate_percent / HUNDRED;
    let factor: f64 = 1.0 + rate;
    principal * factor.powi(years as i32)
}
```

> Two pedagogical points baked into the code, both requiring `#[allow]`
> attributes that workspace `clippy::pedantic` would otherwise refuse:
>
> 1. The double-let pattern in `fahrenheit_to_celsius`
>    (`let f = ...; let scaled: f64 = ...;`) demonstrates both shadowing AND
>    an explicit annotation. `clippy::let_and_return` would normally inline
>    the trailing `scaled`, but the named binding is the lesson.
> 2. The `years as i32` cast in `compound_interest` introduces the `as`
>    keyword. `clippy::pedantic` flags casts that *could* lose information
>    even when they don't in practice; the comment explains why it's safe
>    here.
>
> Both `#[allow]`s carry a short comment explaining the choice — this is
> the responsible-clippy-silencing pattern students will encounter in
> real-world code.

- [ ] **Step 2: Overwrite `lessons/02-variables/solutions/tests/exercise.rs`**

```rust
use variables_solutions::{compound_interest, fahrenheit_to_celsius};

// Warm-up: fahrenheit_to_celsius

#[test]
fn warmup_freezing() {
    assert_eq!(fahrenheit_to_celsius(32.0), 0.0);
}

#[test]
fn warmup_boiling() {
    assert_eq!(fahrenheit_to_celsius(212.0), 100.0);
}

#[test]
fn warmup_ten_celsius() {
    assert_eq!(fahrenheit_to_celsius(50.0), 10.0);
}

// Main: compound_interest

#[test]
fn main_zero_principal_grows_to_zero() {
    assert_eq!(compound_interest(0.0, 50.0, 10), 0.0);
}

#[test]
fn main_zero_rate_returns_principal() {
    assert_eq!(compound_interest(1000.0, 0.0, 5), 1000.0);
}

#[test]
fn main_fifty_percent_two_years() {
    assert_eq!(compound_interest(100.0, 50.0, 2), 225.0);
}

#[test]
fn main_twentyfive_percent_two_years() {
    assert_eq!(compound_interest(200.0, 25.0, 2), 312.5);
}
```

- [ ] **Step 3: Verify solution tests pass**

```bash
cargo test --package variables-solutions
```

Expected: 7 tests pass.

- [ ] **Step 4: Verify lint passes on the solutions crate**

```bash
cargo clippy --package variables-solutions --all-targets -- -D warnings
cargo fmt --check --package variables-solutions
```

Expected: both exit 0. The function-level `#[allow]` attributes (Step 1) suppress the two known pedantic firings; nothing else should fire. If you see other lint warnings, fix them rather than allow-listing — only the two clippy concerns the plan calls out are intentional.

- [ ] **Step 5: Commit**

```bash
git add lessons/02-variables/solutions
git commit -m "feat(lesson-02): add reference solutions"
```

---

## Task 4: Lesson README

**Files:**
- Overwrite: `lessons/02-variables/README.md`

- [ ] **Step 1: Overwrite `lessons/02-variables/README.md`**

````markdown
# Lesson 02 — Variables, types, mutability

In Rust the default is no mutation. The type system makes that practical.
By the end of this lesson you'll know how to declare bindings, how to opt
into mutation when you need it, how to read inferred types and add
annotations, and how shadowing differs from mutation in subtle but
important ways.

## Learning goals

- Declare bindings with `let` and explain why they're immutable by default
- Opt into mutation with `let mut` and explain when it's appropriate
- Read Rust's inferred types and add explicit annotations when needed
- Re-bind a name using shadowing, and explain why it's different from mutation
- Declare compile-time constants with `const`

## Self-study notes

### The default is immutable

`let` creates a binding from a name to a value. Once made, that binding
cannot be reassigned:

```rust
let x = 5;
x = 6; // error: cannot assign twice to immutable variable `x`
```

You met this error in Lesson 01. Rust is telling you that the binding
`x` is immutable, which is its default. Most bindings in a typical Rust
program are immutable — and that's by design. Immutability removes a
whole class of bugs and makes code easier to read: you can be confident
that a variable's value doesn't change underneath you.

### Opting into mutation with `mut`

When you genuinely need a binding to change, opt in with `mut`:

```rust
let mut counter = 0;
counter = counter + 1;
counter = counter + 1;
```

The `mut` keyword is a deliberate, local signal to future readers (often
your future self): *"yes, this thing changes."* If you don't see `mut`,
you can trust that the binding holds the same value its whole life.

### Type inference and annotations

Rust infers the type of a binding from the value:

```rust
let x = 5;       // inferred: i32 (the default integer type)
let y = 5.0;     // inferred: f64
let ok = true;   // inferred: bool
```

Sometimes you want to be explicit, or Rust can't infer (we'll see one in
a few lessons when we call `Vec::new()`). Add the type after the name:

```rust
let x: u32 = 5;
let pi: f64 = 3.14159;
```

Annotations also enforce. If you try `let x: u32 = -1;`, the compiler
refuses — `-1` isn't a valid `u32`.

> **Aside on dot-syntax.** Numbers have methods. You'll see `factor.powi(2)`
> in this lesson's main exercise — `.powi` is a method on `f64`. We'll
> cover methods properly with structs in Lesson 06.

### Shadowing

You can declare a binding with a name that already exists. The new
binding takes over; the old one is gone:

```rust
let x = 5;
let x = x + 1;   // a new `x`, value 6
let x = "five";  // a new `x`, type &str
```

This is NOT mutation. Each `let` makes a fresh binding. The compiler
treats them as separate variables that happen to share a name. The old
`x` is dropped when the new one shadows it.

### `mut` vs shadowing — when to use which

Both let you "change" what `x` refers to. They're different tools for
different jobs:

| aspect       | `let mut x = ...; x = ...;` | `let x = ...; let x = ...;` |
|--------------|-----------------------------|-----------------------------|
| binding      | same binding                | new binding                 |
| type         | must stay the same          | can change                  |
| old value    | overwritten                 | dropped                     |
| signals to reader | "this thing changes"   | "I'm done with that; here's the next step" |

A common mistake is reaching for `mut` when the situation actually
wants shadowing — typically when you're transforming a value through a
few steps:

```rust
// Awkward — needs a different type at each step, but mut forces same-type:
let mut s = "hello";
// s = s.len();  // ERROR: cannot assign integer to &str binding

// Natural — each step is a fresh binding via shadowing:
let s = "hello";
let s = s.len();
```

This lesson's compile-fail exercise drives this misconception home with
the compiler's own diagnostic.

### `const`

For values known at compile time that never change, use `const`:

```rust
const MAX_RETRIES: u32 = 3;
const HUNDRED: f64 = 100.0;
```

Three rules to remember about `const`:

1. The type **must** be annotated — `const` doesn't infer.
2. The value must be a compile-time constant expression.
3. Convention is `SCREAMING_SNAKE_CASE`.

## Exercises

### Warm-up: `fahrenheit_to_celsius`

Open `exercises/src/lib.rs` and implement `fahrenheit_to_celsius` using
the formula `(F − 32) × 5 / 9`.

### Main: `compound_interest`

In the same file, implement `compound_interest(principal, rate_percent, years)`
returning `principal × (1 + rate_percent/100)^years`. You'll need
`f64::powi`. Hints in the slide aside.

### Compile-fail

`exercises/compile_fails/02-mut-cant-change-type.rs` ships in a state
that does **not** compile. Read the comment in the file, then fix it.
The fix is one keyword change on one line.

### Run

```bash
make verify LESSON=02-variables
```

This runs your exercise tests and then asserts the compile-fail file now
compiles.

## Solutions

See `solutions/src/lib.rs` for the reference implementations. Try the
exercises before peeking.
````

- [ ] **Step 2: Spot-check the README**

```bash
head -20 lessons/02-variables/README.md
grep -c '^### ' lessons/02-variables/README.md
```

Expected: starts with `# Lesson 02 — Variables, types, mutability`; `grep -c` returns 9 (six `### ` subsections under self-study + three under exercises).

- [ ] **Step 3: Commit**

```bash
git add lessons/02-variables/README.md
git commit -m "docs(lesson-02): write self-study notes"
```

---

## Task 5: Slides

**Files:**
- Overwrite: `lessons/02-variables/slides/slides.md`

- [ ] **Step 1: Overwrite `lessons/02-variables/slides/slides.md`**

````markdown
# Variables, types, mutability

> In Rust, the default is no mutation. The type system makes that practical.

---

## Recap

Lesson 01 showed this error:

```rust
let x = 1;
x = 2; // ERROR: cannot assign twice to immutable variable `x`
```

Now let's understand why, and learn what to do about it.

---

## `let` — the default

```rust
let x = 5;
```

- `x` is bound to `5`
- `x` cannot be reassigned

This is the default for every binding.

Most variables in a typical Rust program are immutable.

---

## `let mut` — opting into mutation

```rust
let mut counter = 0;
counter = counter + 1;
counter = counter + 1;
```

- `mut` is a deliberate, local opt-in
- It signals to future readers: *"yes, this thing changes"*

Note: prefer immutable when you can. Reach for `mut` only when you need it.

---

## Type inference

Rust infers types from values:

```rust
let x = 5;       // i32
let y = 5.0;     // f64
let ok = true;   // bool
```

Hovering over a binding in your editor shows the inferred type.

Note: aside on dot-syntax — numbers have methods. You'll see `factor.powi(2)` in this lesson's main exercise. `.powi` is a method on `f64`. We'll cover methods properly with structs in Lesson 06.

---

## Type annotations

```rust
let x: u32 = 5;
let pi: f64 = 3.14159;
```

When to annotate:

- You want to be explicit for the reader
- Rust can't infer (e.g., `Vec::new()` later)

Annotations enforce:

```rust
let x: u32 = -1; // ERROR: -1 is not a u32
```

---

## Shadowing

```rust
let x = 5;
let x = x + 1;    // new x, value 6
let x = "five";   // new x, type &str
```

This is NOT mutation. Each `let` makes a fresh binding.

The old `x` is dropped when the new one shadows it.

---

## `mut` vs shadowing

|                | `let mut x = …; x = …;` | `let x = …; let x = …;` |
|----------------|-------------------------|-------------------------|
| binding        | same                    | new                     |
| type           | must stay the same      | can change              |
| old value      | overwritten             | dropped                 |
| signals reader | "this changes"          | "next step"             |

---

## Common mistake

```rust
let mut s = "hello";
s = s.len();
// ERROR: expected &str, found integer
```

`mut` does NOT let the type change. Use shadowing:

```rust
let s = "hello";
let s = s.len();   // s is now usize
```

Note: this is exactly what the lesson's compile-fail exercise drives home — try it before reading the solution.

---

## `const`

```rust
const MAX_RETRIES: u32 = 3;
const HUNDRED: f64 = 100.0;
```

Three rules:

1. Type **must** be annotated — no inference
2. Value must be a compile-time constant
3. Convention: `SCREAMING_SNAKE_CASE`

`const` is the right choice for values known at compile time that never change.

---

## Wrap

- Immutability is the default
- `mut` is a local, opt-in escape hatch
- Shadowing is a different tool for a different job
- `const` for true compile-time constants

Next: Lesson 03 — control flow & functions.
````

- [ ] **Step 2: Verify `make slides-build` succeeds and includes lesson 02**

```bash
make slides-build
test -f dist/lessons/02-variables/slides/slides.md
test -f dist/lessons/02-variables/slides/index.html
grep -c "02-variables" dist/index.html
```

Expected: `slides.md` and `index.html` copied into dist; `grep -c "02-variables"` returns at least 1 (the published lesson link).

- [ ] **Step 3: Commit**

```bash
git add lessons/02-variables/slides/slides.md
git commit -m "feat(lesson-02): write slide deck"
```

---

## Task 6: End-to-end verification

- [ ] **Step 1: `make ci` is green**

```bash
make ci
```

Expected: exit 0. Clippy clean, fmt clean, workspace builds, default-members tests pass (now includes the 7 new tests in `variables-solutions`), compile-fail `--expect broken` passes for lesson 02.

- [ ] **Step 2: `make verify LESSON=02-variables` fails (the exercise is undone — intentional)**

```bash
make verify LESSON=02-variables || echo "expected: exercise tests fail with todo!() panic"
```

Expected: non-zero exit. All 7 exercise tests panic with `not yet implemented`. The student fixing the exercise is what makes this pass.

- [ ] **Step 3: `make slides-build` final state**

```bash
make slides-build
ls dist/lessons/
grep -c "02-variables" dist/index.html
```

Expected: `dist/lessons/` contains both `01-hello-rust` and `02-variables`. `grep -c "02-variables"` ≥ 1.

- [ ] **Step 4: Push**

```bash
git push
```

Expected: push succeeds. CI workflow runs; deploy workflow runs (deploy will rebuild the static site and serve lesson 02 at `rust.ristkari.dev/lessons/02-variables/slides/`).

---

## Done criteria

- `lessons/02-variables/` exists with all four parts
- `cargo test --package variables-solutions` → 7 passing tests
- `cargo test --manifest-path lessons/02-variables/exercises/Cargo.toml` → 7 panicking tests (intentional)
- `cargo run --package compile-fails -- --expect broken lessons/02-variables` → ok
- `cargo run --package compile-fails -- --expect compiles lessons/02-variables` → fails (intentional)
- `make ci` → green
- `make slides-build` → produces `dist/lessons/02-variables/slides/index.html`
- `dist/index.html` lists lesson 02 as a clickable link
- All changes committed and pushed
