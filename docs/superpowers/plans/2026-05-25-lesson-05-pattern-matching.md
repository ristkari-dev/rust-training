# Lesson 05 — Pattern matching & enums — Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Author the fifth lesson of the Rust training course: a warm-up + main exercise pair on `Option<T>` (`safe_divide` returning `Option<i32>`) and a custom enum + exhaustive `match` (`next` cycling a `Light` traffic-light state machine), with one compile-fail exercise on Rust's non-exhaustive-match diagnostic.

**Architecture:** Use the existing `make new-lesson` scaffolder to lay down the four-part lesson structure, then overwrite the placeholder content with lesson-specific README, slides, exercises, and solutions per the design spec. Both exercise and solution crates ship the `Light` enum already defined and derived so tests can `assert_eq!` directly.

**Tech Stack:** Rust 2024 edition, existing tools (`new-lesson`, `compile-fails`, `slides-dev`, `build-index`), reveal.js (vendored), GNU Make.

**Spec:** [`docs/superpowers/specs/2026-05-25-lesson-05-pattern-matching-design.md`](../specs/2026-05-25-lesson-05-pattern-matching-design.md).

**Working directory:** `/Users/ristkari/code/private/rust-training`.

---

## Task 1: Scaffold lessons/05-pattern-matching

**Files (all created by the scaffolder):**
- `lessons/05-pattern-matching/README.md` (placeholder, replaced in Task 4)
- `lessons/05-pattern-matching/slides/index.html` (final — no edit needed)
- `lessons/05-pattern-matching/slides/slides.md` (placeholder, replaced in Task 5)
- `lessons/05-pattern-matching/exercises/Cargo.toml` (final — no edit needed)
- `lessons/05-pattern-matching/exercises/src/lib.rs` (placeholder, replaced in Task 2)
- `lessons/05-pattern-matching/exercises/tests/exercise.rs` (placeholder, replaced in Task 2)
- `lessons/05-pattern-matching/solutions/Cargo.toml` (final — no edit needed)
- `lessons/05-pattern-matching/solutions/src/lib.rs` (placeholder, replaced in Task 3)
- `lessons/05-pattern-matching/solutions/tests/exercise.rs` (placeholder, replaced in Task 3)

- [ ] **Step 1: Run the scaffolder**

```bash
make new-lesson NAME=05-pattern-matching
```

Expected: `scaffolded ./lessons/05-pattern-matching`.

- [ ] **Step 2: Verify directory structure**

```bash
ls lessons/05-pattern-matching/
ls lessons/05-pattern-matching/slides/ lessons/05-pattern-matching/exercises/ lessons/05-pattern-matching/solutions/
```

Expected: top-level `README.md`, `slides/`, `exercises/`, `solutions/` populated from templates.

- [ ] **Step 3: Verify Cargo package names**

```bash
grep '^name' lessons/05-pattern-matching/exercises/Cargo.toml lessons/05-pattern-matching/solutions/Cargo.toml
```

Expected:
```
lessons/05-pattern-matching/exercises/Cargo.toml:name = "pattern-matching-exercises"
lessons/05-pattern-matching/solutions/Cargo.toml:name = "pattern-matching-solutions"
```

- [ ] **Step 4: Verify the workspace picks up the new crates**

```bash
cargo metadata --no-deps --format-version 1 | grep -o '"name":"pattern-matching-[^"]*"' | sort -u
```

Expected output:
```
"name":"pattern-matching-exercises"
"name":"pattern-matching-solutions"
```

- [ ] **Step 5: Verify the scaffolded workspace builds clean**

```bash
cargo build --workspace
```

Expected: warning-free build.

- [ ] **Step 6: Commit the scaffold**

```bash
git add lessons/05-pattern-matching
git commit -m "chore: scaffold lessons/05-pattern-matching"
```

---

## Task 2: Exercise content (enum + stubs + tests + compile-fail)

**Files:**
- Overwrite: `lessons/05-pattern-matching/exercises/src/lib.rs`
- Overwrite: `lessons/05-pattern-matching/exercises/tests/exercise.rs`
- Create: `lessons/05-pattern-matching/exercises/compile_fails/05-non-exhaustive-match.rs`

- [ ] **Step 1: Overwrite `lessons/05-pattern-matching/exercises/src/lib.rs`**

The exercise crate ships the `Light` enum already defined and derived. Students only need to fill in the two function bodies.

```rust
//! Lesson 05 — exercises.
//!
//! Implement `safe_divide` (warm-up) and `next` (main) so that
//! `cargo test --manifest-path lessons/05-pattern-matching/exercises/Cargo.toml`
//! passes. The tests live in `tests/exercise.rs`.

#[derive(Debug, PartialEq, Eq)]
pub enum Light {
    Red,
    Yellow,
    Green,
}

#[must_use]
pub fn safe_divide(_a: i32, _b: i32) -> Option<i32> {
    todo!("return None when b == 0, otherwise Some(a / b)")
}

#[must_use]
pub fn next(_light: Light) -> Light {
    todo!("return Red->Green, Green->Yellow, Yellow->Red")
}
```

- [ ] **Step 2: Overwrite `lessons/05-pattern-matching/exercises/tests/exercise.rs`**

```rust
use pattern_matching_exercises::{Light, next, safe_divide};

// Warm-up: safe_divide

#[test]
fn warmup_typical() {
    assert_eq!(safe_divide(10, 2), Some(5));
}

#[test]
fn warmup_by_zero() {
    assert_eq!(safe_divide(10, 0), None);
}

#[test]
fn warmup_zero_dividend() {
    assert_eq!(safe_divide(0, 5), Some(0));
}

#[test]
fn warmup_negative() {
    assert_eq!(safe_divide(-10, 2), Some(-5));
}

// Main: next (traffic light)

#[test]
fn main_red_to_green() {
    assert_eq!(next(Light::Red), Light::Green);
}

#[test]
fn main_green_to_yellow() {
    assert_eq!(next(Light::Green), Light::Yellow);
}

#[test]
fn main_yellow_to_red() {
    assert_eq!(next(Light::Yellow), Light::Red);
}

#[test]
fn main_cycle_closes() {
    assert_eq!(next(next(next(Light::Red))), Light::Red);
}
```

- [ ] **Step 3: Create `lessons/05-pattern-matching/exercises/compile_fails/05-non-exhaustive-match.rs`**

```rust
// Compile-fail exercise: this file MUST NOT compile until you fix it.
//
// Rust's `match` expression is **exhaustive**: every possible variant
// of the matched type must have an arm. If you forget one, the compiler
// refuses to compile. This is one of Rust's signature safety features —
// you can't accidentally not-handle a case.
//
// The function below matches on a `Direction` enum but forgets one of
// the four variants. rustc will tell you exactly which one is missing.
//
// Hint: read the rustc error. It will say "non-exhaustive patterns:
// `<Variant>` not covered". Add an arm that maps the missing variant to
// its opposite.

#[derive(Debug)]
enum Direction {
    North,
    South,
    East,
    West,
}

fn opposite(d: Direction) -> Direction {
    match d {
        Direction::North => Direction::South,
        Direction::South => Direction::North,
        Direction::East => Direction::West,
        // West is missing!
    }
}

fn main() {
    let d = opposite(Direction::West);
    println!("{d:?}");
}
```

- [ ] **Step 4: Verify exercise tests fail with `todo!()` panic (intentional)**

```bash
cargo test --manifest-path lessons/05-pattern-matching/exercises/Cargo.toml
```

Expected: all 8 tests fail with `not yet implemented` panic message.

- [ ] **Step 5: Verify the exercises crate still builds cleanly**

```bash
cargo build --package pattern-matching-exercises
```

Expected: warning-free build.

- [ ] **Step 6: Verify compile-fail ships broken**

```bash
cargo run --package compile-fails -- --expect broken lessons/05-pattern-matching
```

Expected: `ok: lessons/05-pattern-matching/exercises/compile_fails/05-non-exhaustive-match.rs` and exit 0.

- [ ] **Step 7: Verify compile-fail's student-mode check fires**

```bash
cargo run --package compile-fails -- --expect compiles lessons/05-pattern-matching
```

Expected: non-zero exit with `FAIL: file did not compile, but was expected to: lessons/05-pattern-matching/...`.

- [ ] **Step 8: Verify lint passes on the exercises crate**

```bash
cargo clippy --package pattern-matching-exercises --all-targets -- -D warnings
cargo fmt --check --package pattern-matching-exercises
```

Expected: both exit 0.

- [ ] **Step 9: Commit**

```bash
git add lessons/05-pattern-matching/exercises
git commit -m "feat(lesson-05): add enum, exercise stubs, tests, and compile-fail"
```

---

## Task 3: Reference solutions

**Files:**
- Overwrite: `lessons/05-pattern-matching/solutions/src/lib.rs`
- Overwrite: `lessons/05-pattern-matching/solutions/tests/exercise.rs`

- [ ] **Step 1: Overwrite `lessons/05-pattern-matching/solutions/src/lib.rs`**

```rust
//! Lesson 05 — reference solutions.

#[derive(Debug, PartialEq, Eq)]
pub enum Light {
    Red,
    Yellow,
    Green,
}

#[must_use]
pub fn safe_divide(a: i32, b: i32) -> Option<i32> {
    match b {
        0 => None,
        _ => Some(a / b),
    }
}

#[must_use]
pub fn next(light: Light) -> Light {
    match light {
        Light::Red => Light::Green,
        Light::Green => Light::Yellow,
        Light::Yellow => Light::Red,
    }
}
```

> Pedagogical notes baked into the solutions:
> - `safe_divide` uses `match` (not `if`) to model the lesson's primary tool. Either solution would pass the tests; the reference picks `match` for pedagogical alignment.
> - `next` matches all three variants of the enum exhaustively. No wildcard `_`; every variant has its own arm to drive home the "match every variant" pattern.
> - The `Light` enum is defined identically to the exercises crate (same variants, same derives) so both crates' tests use the same type structurally.
>
> No `#[allow]` attributes should be needed. If clippy fires (especially `clippy::match_same_arms` if arms happen to collapse), STOP and report rather than allow-listing.

- [ ] **Step 2: Overwrite `lessons/05-pattern-matching/solutions/tests/exercise.rs`**

```rust
use pattern_matching_solutions::{Light, next, safe_divide};

// Warm-up: safe_divide

#[test]
fn warmup_typical() {
    assert_eq!(safe_divide(10, 2), Some(5));
}

#[test]
fn warmup_by_zero() {
    assert_eq!(safe_divide(10, 0), None);
}

#[test]
fn warmup_zero_dividend() {
    assert_eq!(safe_divide(0, 5), Some(0));
}

#[test]
fn warmup_negative() {
    assert_eq!(safe_divide(-10, 2), Some(-5));
}

// Main: next (traffic light)

#[test]
fn main_red_to_green() {
    assert_eq!(next(Light::Red), Light::Green);
}

#[test]
fn main_green_to_yellow() {
    assert_eq!(next(Light::Green), Light::Yellow);
}

#[test]
fn main_yellow_to_red() {
    assert_eq!(next(Light::Yellow), Light::Red);
}

#[test]
fn main_cycle_closes() {
    assert_eq!(next(next(next(Light::Red))), Light::Red);
}
```

- [ ] **Step 3: Verify solution tests pass**

```bash
cargo test --package pattern-matching-solutions
```

Expected: 8 tests pass.

- [ ] **Step 4: Verify lint passes on the solutions crate**

```bash
cargo clippy --package pattern-matching-solutions --all-targets -- -D warnings
cargo fmt --check --package pattern-matching-solutions
```

Expected: both exit 0. No `#[allow]` attributes needed. If clippy fires, STOP and report.

- [ ] **Step 5: Commit**

```bash
git add lessons/05-pattern-matching/solutions
git commit -m "feat(lesson-05): add reference solutions"
```

---

## Task 4: Lesson README

**Files:**
- Overwrite: `lessons/05-pattern-matching/README.md`

- [ ] **Step 1: Overwrite `lessons/05-pattern-matching/README.md`**

The complete file content (write this as the file's actual content, starting with the `# Lesson 05` heading). Code fences inside the markdown are plain triple-backticks.

```markdown
# Lesson 05 — Pattern matching & enums

Rust gives you the ability to define your own sum types — and the
compiler will not let you forget a case. Today you'll define an enum,
use `match` exhaustively against it, and meet `Option<T>` — the
standard library's answer to null. By the end you'll have written a
small state machine in three arms of a match.

## Learning goals

- Define a custom `enum` with multiple variants and derive `Debug` /
  `PartialEq` for it
- Use `match` exhaustively against an enum and read rustc's
  exhaustiveness error when a variant is missing
- Use pattern features — wildcards `_`, binding patterns, literal
  patterns, range patterns — in match arms
- Construct and destructure `Option<T>` to represent values that may
  be absent
- Use `if let` as sugar for a single-variant match

## Self-study notes

### Defining your own enums

An `enum` is a type whose value is exactly one of a fixed set of named
variants:

​```rust
enum Direction {
    North,
    South,
    East,
    West,
}

let heading = Direction::North;
​```

To print an enum with `{:?}`, derive `Debug`. To compare two enums with
`==`, derive `PartialEq` (and usually `Eq` alongside):

​```rust
#[derive(Debug, PartialEq, Eq)]
enum Direction {
    North,
    South,
    East,
    West,
}
​```

We'll learn what derives *are* in Lesson 12; for now treat them as
"give me the obvious behavior for this type."

### The `match` expression

`match` lets you handle every variant of an enum (or value of any
other type) and is **exhaustive** — if you miss a variant, the
compiler refuses to compile.

​```rust
fn opposite(d: Direction) -> Direction {
    match d {
        Direction::North => Direction::South,
        Direction::South => Direction::North,
        Direction::East  => Direction::West,
        Direction::West  => Direction::East,
    }
}
​```

Each arm is `pattern => expression`. Like `if`, `match` itself is an
expression — it produces the value of whichever arm matched.

### Patterns: wildcards, bindings, ranges, literals

Match arms don't have to be enum variants. Patterns include:

​```rust
fn classify(n: i32) -> &'static str {
    match n {
        0 => "zero",          // literal
        1..=9 => "small",     // inclusive range
        _ => "other",         // wildcard — catches the rest
    }
}
​```

You can also **bind** the matched value to a name:

​```rust
let label = match x {
    n => format!("got {n}"),  // binds x to n
};
​```

A bare identifier (here `n`) is a binding pattern — it always matches
and binds.

### `Option<T>` — the standard-library nullable

Rust's standard library defines:

​```rust
enum Option<T> {
    Some(T),
    None,
}
​```

`Option` is how Rust represents "a value of type T, or none." There is
no null pointer in Rust — instead, types that might be missing are
wrapped in `Option`:

​```rust
let found: Option<i32> = Some(42);
let missing: Option<i32> = None;
​```

You unpack `Option` with `match`:

​```rust
fn double_or_zero(n: Option<i32>) -> i32 {
    match n {
        Some(x) => x * 2,
        None    => 0,
    }
}
​```

The `Some(x)` pattern binds the inner value to `x` for use in that
arm. The compiler enforces that both arms are present.

### `if let` — sugar for one variant

When you only care about one variant, `if let` is shorter than
`match`:

​```rust
let opt: Option<i32> = Some(5);

if let Some(x) = opt {
    println!("got {x}");
}
​```

This is equivalent to:

​```rust
match opt {
    Some(x) => println!("got {x}"),
    None    => (),
}
​```

`if let` loses exhaustiveness checking — you've explicitly told Rust
"I only care about Some, do nothing for None." Use it when that
genuinely matches what you mean.

## Exercises

### Warm-up: `safe_divide`

Implement `safe_divide(a: i32, b: i32) -> Option<i32>` in
`exercises/src/lib.rs`:

- Return `None` when `b == 0`
- Return `Some(a / b)` otherwise

The reference solution uses
`match b { 0 => None, _ => Some(a / b) }`, but `if`/`else` works
equally well.

### Main: `next` (traffic light)

The exercises crate already defines:

​```rust
#[derive(Debug, PartialEq, Eq)]
pub enum Light {
    Red,
    Yellow,
    Green,
}
​```

Implement `next(light: Light) -> Light` so the cycle is:

- `Red` → `Green`
- `Green` → `Yellow`
- `Yellow` → `Red`

Use a `match` over all three variants. Each arm returns the next
`Light` directly — no wildcard.

### Compile-fail

`exercises/compile_fails/05-non-exhaustive-match.rs` ships with a
`match` that misses one variant of a `Direction` enum. Read the rustc
error (it names the missing variant), then add the missing arm.

### Run

​```bash
make verify LESSON=05-pattern-matching
​```

This runs your exercise tests and asserts the compile-fail file now
compiles.

## Solutions

See `solutions/src/lib.rs` for the reference implementations. Try the
exercises before peeking.
```

> **IMPORTANT for the implementer:** the code-fence escape in the markdown above uses an invisible zero-width character (shown as `​```) in front of each triple-backtick block — that's only there so this plan file can nest fenced markdown inside an outer fenced markdown block. When you write the actual `README.md`, every fence must be three PLAIN backticks `` ``` `` with NO leading invisible character. The actual file should contain 11 Rust code blocks plus 1 bash code block = 12 code blocks = 24 fence lines.

- [ ] **Step 2: Spot-check the README**

```bash
head -1 lessons/05-pattern-matching/README.md
grep -c '^### ' lessons/05-pattern-matching/README.md
grep -c '^```' lessons/05-pattern-matching/README.md
```

Expected:
- First line: `# Lesson 05 — Pattern matching & enums`
- `grep -c '^### '` returns 9 (five subsections under self-study + four under exercises)
- `grep -c '^```'` returns 24 (12 code blocks × 2 fence lines)

- [ ] **Step 3: Commit**

```bash
git add lessons/05-pattern-matching/README.md
git commit -m "docs(lesson-05): write self-study notes"
```

---

## Task 5: Slide deck

**Files:**
- Overwrite: `lessons/05-pattern-matching/slides/slides.md`

- [ ] **Step 1: Overwrite `lessons/05-pattern-matching/slides/slides.md`**

The complete file content (write this as the file's actual content, starting with the `# Pattern matching & enums` heading):

````
# Pattern matching & enums

> Rust gives you the ability to define your own sum types — and the compiler will not let you forget a case.

---

## Recap

Lesson 04 introduced compound types we **own**.

Today: add **sum types** — values that are *one of several* shapes — and the `match` expression that handles them safely.

---

## Enums

```rust
enum Direction {
    North,
    South,
    East,
    West,
}

let d = Direction::North;
```

- A type whose value is exactly one of a fixed set of variants
- Construct with `Direction::North`, `Direction::South`, ...
- Derive `Debug` to print, `PartialEq` to compare with `==`:

```rust
#[derive(Debug, PartialEq, Eq)]
enum Direction { /* ... */ }
```

---

## `match` on enums

```rust
fn opposite(d: Direction) -> Direction {
    match d {
        Direction::North => Direction::South,
        Direction::South => Direction::North,
        Direction::East  => Direction::West,
        Direction::West  => Direction::East,
    }
}
```

- **Exhaustive**: every variant must have an arm
- Each arm is an expression
- `match` itself is an expression — can be assigned, returned, used as a tail

---

## More patterns

```rust
fn classify(n: i32) -> &'static str {
    match n {
        0 => "zero",          // literal
        1..=9 => "small",     // inclusive range
        _ => "other",         // wildcard catches the rest
    }
}
```

- Literals: `0`, `"yes"`, etc.
- Ranges: `1..=9` (inclusive)
- Bindings: `n => ...` (catches and binds the value)
- Wildcard: `_ => ...` (catches the rest without binding)

---

## `Option<T>`

The most important enum in the standard library:

```rust
enum Option<T> {
    Some(T),
    None,
}
```

Rust's answer to null. Any value that might be missing is `Option<T>` instead of "null T".

```rust
let found: Option<i32> = Some(42);
let missing: Option<i32> = None;
```

---

## Matching on `Option`

```rust
fn double_or_zero(n: Option<i32>) -> i32 {
    match n {
        Some(x) => x * 2,
        None    => 0,
    }
}
```

- `Some(x)` binds the inner value to `x`
- `None` matches the absent case
- The compiler enforces both arms — you can't forget the missing case

---

## `if let` — one-variant sugar

When you only care about one variant:

```rust
let opt: Option<i32> = Some(5);

if let Some(x) = opt {
    println!("got {x}");
}
```

Equivalent to:

```rust
match opt {
    Some(x) => println!("got {x}"),
    None    => (),
}
```

Trade-off: you lose exhaustiveness. Use when there's genuinely just one case to handle.

---

## Putting it together

Define your own enum:

```rust
#[derive(Debug, PartialEq, Eq)]
pub enum Light {
    Red,
    Yellow,
    Green,
}
```

Then match all variants:

```rust
pub fn next(light: Light) -> Light {
    match light {
        Light::Red    => Light::Green,
        Light::Green  => Light::Yellow,
        Light::Yellow => Light::Red,
    }
}
```

If you forget a variant, the compiler refuses to compile — try it in the compile-fail exercise.

---

## Wrap

- **Enums** define your own sum types
- **`match`** is exhaustive and is an expression
- Patterns: literals, ranges, bindings, wildcards
- **`Option<T>`** is just an enum — Rust's nullable
- **`if let`** is sugar for the single-variant case

Next: Lesson 06 — structs & methods.
````

The OUTER fence above (quadruple backticks) is just delimiting this content in the plan. The FILE you write should NOT contain the outer fence — only the inner triple-backtick `rust` blocks.

The file should:
- Start with `# Pattern matching & enums` on line 1
- Have exactly 9 `---` slide separators (between 10 slides)
- Contain 11 triple-backtick `rust` code fences

- [ ] **Step 2: Verify `make slides-build` succeeds and includes lesson 05**

```bash
make slides-build
test -f dist/lessons/05-pattern-matching/slides/slides.md
test -f dist/lessons/05-pattern-matching/slides/index.html
grep -c "05-pattern-matching" dist/index.html
```

Expected: `slides.md` and `index.html` copied into dist; `grep -c "05-pattern-matching"` returns at least 1.

- [ ] **Step 3: Spot-check slide separators**

```bash
grep -c '^---$' lessons/05-pattern-matching/slides/slides.md
```

Expected: `9` (between 10 slides).

- [ ] **Step 4: Commit**

```bash
git add lessons/05-pattern-matching/slides/slides.md
git commit -m "feat(lesson-05): write slide deck"
```

---

## Task 6: End-to-end verification + push

- [ ] **Step 1: `make ci` is green**

```bash
make ci
```

Expected: exit 0. Clippy clean, fmt clean, workspace builds, default-members tests pass (now includes the 8 new tests in `pattern-matching-solutions`), compile-fail `--expect broken` passes for lesson 05.

- [ ] **Step 2: `make verify LESSON=05-pattern-matching` fails (the exercise is undone — intentional)**

```bash
make verify LESSON=05-pattern-matching || echo "expected: exercise tests fail with todo!() panic"
```

Expected: non-zero exit. All 8 exercise tests panic with `not yet implemented`.

- [ ] **Step 3: `make slides-build` final state**

```bash
make slides-build
ls dist/lessons/
grep -c "05-pattern-matching" dist/index.html
```

Expected: `dist/lessons/` contains `01-hello-rust`, `02-variables`, `03-control-flow`, `04-compound-types`, and `05-pattern-matching`. `grep -c` ≥ 1.

- [ ] **Step 4: Push**

```bash
git push
```

Expected: push succeeds. CI runs and is green; Deploy rebuilds the static site so lesson 05 appears live.

- [ ] **Step 5: Smoke-test the deployed site**

After the push, wait for the Deploy workflow to finish (`gh run watch <id>`). Then:

```bash
curl -sS -o /dev/null -w "%{http_code}\n" https://rust.ristkari.dev/
curl -sS -o /dev/null -w "%{http_code}\n" https://rust.ristkari.dev/lessons/05-pattern-matching/slides/
```

Expected: both return `200`.

---

## Done criteria

- `lessons/05-pattern-matching/` exists with all four parts
- Both `exercises/src/lib.rs` and `solutions/src/lib.rs` define the `Light` enum identically
- `cargo test --package pattern-matching-solutions` → 8 passing tests
- `cargo test --manifest-path lessons/05-pattern-matching/exercises/Cargo.toml` → 8 panicking tests (intentional)
- `cargo run --package compile-fails -- --expect broken lessons/05-pattern-matching` → ok
- `cargo run --package compile-fails -- --expect compiles lessons/05-pattern-matching` → fails (intentional)
- `make ci` → green
- `make slides-build` → produces `dist/lessons/05-pattern-matching/slides/index.html`
- `dist/index.html` lists lesson 05 as a clickable link
- All changes committed and pushed
- Deployed site returns HTTP 200 for `/` and `/lessons/05-pattern-matching/slides/`
