# Lesson 07 — Ownership & moves — Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Author the first lesson of Phase 2 of the Rust training course: a warm-up + main exercise pair on ownership and moves. Warm-up is `append_excl` (ownership in + mutate + ownership out, with `mut` parameter binding); main is `swap_and_join` (two moves, build a new String). Compile-fail is the canonical use-after-move (pass String into function twice, fix with `.clone()`).

**Architecture:** Use the existing `make new-lesson` scaffolder to lay down the four-part lesson structure, then overwrite the placeholder content with lesson-specific README, slides, exercises, and solutions per the design spec.

**Tech Stack:** Rust 2024 edition, existing tools (`new-lesson`, `compile-fails`, `slides-dev`, `build-index`), reveal.js (vendored), GNU Make.

**Spec:** [`docs/superpowers/specs/2026-05-25-lesson-07-ownership-design.md`](../specs/2026-05-25-lesson-07-ownership-design.md).

**Working directory:** `/Users/ristkari/code/private/rust-training`.

---

## Task 1: Scaffold lessons/07-ownership

**Files (all created by the scaffolder):**
- `lessons/07-ownership/README.md` (placeholder, replaced in Task 4)
- `lessons/07-ownership/slides/index.html` (final — no edit needed)
- `lessons/07-ownership/slides/slides.md` (placeholder, replaced in Task 5)
- `lessons/07-ownership/exercises/Cargo.toml` (final — no edit needed)
- `lessons/07-ownership/exercises/src/lib.rs` (placeholder, replaced in Task 2)
- `lessons/07-ownership/exercises/tests/exercise.rs` (placeholder, replaced in Task 2)
- `lessons/07-ownership/solutions/Cargo.toml` (final — no edit needed)
- `lessons/07-ownership/solutions/src/lib.rs` (placeholder, replaced in Task 3)
- `lessons/07-ownership/solutions/tests/exercise.rs` (placeholder, replaced in Task 3)

- [ ] **Step 1: Run the scaffolder**

```bash
make new-lesson NAME=07-ownership
```

Expected: `scaffolded ./lessons/07-ownership`.

- [ ] **Step 2: Verify directory structure**

```bash
ls lessons/07-ownership/
ls lessons/07-ownership/slides/ lessons/07-ownership/exercises/ lessons/07-ownership/solutions/
```

Expected: top-level `README.md`, `slides/`, `exercises/`, `solutions/` populated from templates.

- [ ] **Step 3: Verify Cargo package names**

```bash
grep '^name' lessons/07-ownership/exercises/Cargo.toml lessons/07-ownership/solutions/Cargo.toml
```

Expected:
```
lessons/07-ownership/exercises/Cargo.toml:name = "ownership-exercises"
lessons/07-ownership/solutions/Cargo.toml:name = "ownership-solutions"
```

- [ ] **Step 4: Verify the workspace picks up the new crates**

```bash
cargo metadata --no-deps --format-version 1 | grep -o '"name":"ownership-[^"]*"' | sort -u
```

Expected output:
```
"name":"ownership-exercises"
"name":"ownership-solutions"
```

- [ ] **Step 5: Verify the scaffolded workspace builds clean**

```bash
cargo build --workspace
```

Expected: warning-free build.

- [ ] **Step 6: Commit the scaffold**

```bash
git add lessons/07-ownership
git commit -m "chore: scaffold lessons/07-ownership"
```

---

## Task 2: Exercise content (stubs + tests + compile-fail)

**Files:**
- Overwrite: `lessons/07-ownership/exercises/src/lib.rs`
- Overwrite: `lessons/07-ownership/exercises/tests/exercise.rs`
- Create: `lessons/07-ownership/exercises/compile_fails/07-use-after-move.rs`

- [ ] **Step 1: Overwrite `lessons/07-ownership/exercises/src/lib.rs`**

```rust
//! Lesson 07 — exercises.
//!
//! Implement `append_excl` (warm-up) and `swap_and_join` (main) so
//! that `cargo test --manifest-path
//! lessons/07-ownership/exercises/Cargo.toml` passes. The tests live
//! in `tests/exercise.rs`.

#[must_use]
pub fn append_excl(_s: String) -> String {
    todo!("take ownership of s, push '!' onto the end, return it")
}

#[must_use]
pub fn swap_and_join(_a: String, _b: String) -> String {
    todo!("return b followed by a space followed by a, e.g. swap_and_join(\"hello\", \"world\") -> \"world hello\"")
}
```

- [ ] **Step 2: Overwrite `lessons/07-ownership/exercises/tests/exercise.rs`**

```rust
use ownership_exercises::{append_excl, swap_and_join};

// Warm-up: append_excl

#[test]
fn warmup_typical() {
    assert_eq!(append_excl(String::from("hello")), "hello!");
}

#[test]
fn warmup_empty() {
    assert_eq!(append_excl(String::new()), "!");
}

#[test]
fn warmup_existing_punctuation() {
    assert_eq!(append_excl(String::from("oh no.")), "oh no.!");
}

#[test]
fn warmup_multibyte_chars() {
    assert_eq!(append_excl(String::from("café")), "café!");
}

// Main: swap_and_join

#[test]
fn main_typical() {
    assert_eq!(
        swap_and_join(String::from("hello"), String::from("world")),
        "world hello"
    );
}

#[test]
fn main_single_chars() {
    assert_eq!(swap_and_join(String::from("a"), String::from("b")), "b a");
}

#[test]
fn main_empty_first() {
    assert_eq!(swap_and_join(String::new(), String::from("hi")), "hi ");
}

#[test]
fn main_empty_second() {
    assert_eq!(swap_and_join(String::from("hi"), String::new()), " hi");
}
```

- [ ] **Step 3: Create `lessons/07-ownership/exercises/compile_fails/07-use-after-move.rs`**

```rust
// Compile-fail exercise: this file MUST NOT compile until you fix it.
//
// In Rust, passing a value to a function transfers ownership — unless
// the type is `Copy` (i32, bool, etc.). `String` is NOT Copy; it owns
// heap data. After you pass a `String` to a function, you can't use
// the original binding any more — ownership has moved.
//
// The function below calls `print_string(s)` twice on the same
// binding. The second call is a use-after-move and will fail.
//
// Hint: read the rustc error. It will say "value used here after move"
// and point to where the value moved (the first call). The simplest
// fix is to call `print_string(s.clone())` on the first call so the
// function receives a copy and the original `s` stays usable.

fn print_string(s: String) {
    println!("{s}");
}

fn main() {
    let s = String::from("hello");
    print_string(s);
    print_string(s);
}
```

- [ ] **Step 4: Verify exercise tests fail with `todo!()` panic (intentional)**

```bash
cargo test --manifest-path lessons/07-ownership/exercises/Cargo.toml
```

Expected: all 8 tests fail with `not yet implemented` panic message.

- [ ] **Step 5: Verify the exercises crate still builds cleanly**

```bash
cargo build --package ownership-exercises
```

Expected: warning-free build.

- [ ] **Step 6: Verify compile-fail ships broken**

```bash
cargo run --package compile-fails -- --expect broken lessons/07-ownership
```

Expected: `ok: lessons/07-ownership/exercises/compile_fails/07-use-after-move.rs` and exit 0.

- [ ] **Step 7: Verify compile-fail's student-mode check fires**

```bash
cargo run --package compile-fails -- --expect compiles lessons/07-ownership
```

Expected: non-zero exit with `FAIL: file did not compile, but was expected to: lessons/07-ownership/...`.

- [ ] **Step 8: Verify lint passes on the exercises crate**

```bash
cargo clippy --package ownership-exercises --all-targets -- -D warnings
cargo fmt --check --package ownership-exercises
```

Expected: both exit 0.

- [ ] **Step 9: Commit**

```bash
git add lessons/07-ownership/exercises
git commit -m "feat(lesson-07): add warm-up + main exercise stubs, tests, and compile-fail"
```

---

## Task 3: Reference solutions

**Files:**
- Overwrite: `lessons/07-ownership/solutions/src/lib.rs`
- Overwrite: `lessons/07-ownership/solutions/tests/exercise.rs`

- [ ] **Step 1: Overwrite `lessons/07-ownership/solutions/src/lib.rs`**

```rust
//! Lesson 07 — reference solutions.

#[must_use]
pub fn append_excl(mut s: String) -> String {
    s.push('!');
    s
}

#[must_use]
pub fn swap_and_join(a: String, b: String) -> String {
    let mut result = b;
    result.push(' ');
    result.push_str(&a);
    result
}
```

> Pedagogical notes baked into the solutions:
> - `append_excl` uses `mut s: String` parameter — the function-side mutability shorthand introduced on slide 8.
> - `swap_and_join` uses the move-and-mutate pattern: take both Strings by value, re-bind `b` as `mut result`, mutate it, return.
> - The `&a` in `push_str(&a)` is a borrow. Slide 9 and the README note this is L08's topic; for this lesson, students just type it where rustc asks for it.
>
> No `#[allow]` attributes should be needed. If clippy fires, fix the code rather than allow-listing.

- [ ] **Step 2: Overwrite `lessons/07-ownership/solutions/tests/exercise.rs`**

```rust
use ownership_solutions::{append_excl, swap_and_join};

// Warm-up: append_excl

#[test]
fn warmup_typical() {
    assert_eq!(append_excl(String::from("hello")), "hello!");
}

#[test]
fn warmup_empty() {
    assert_eq!(append_excl(String::new()), "!");
}

#[test]
fn warmup_existing_punctuation() {
    assert_eq!(append_excl(String::from("oh no.")), "oh no.!");
}

#[test]
fn warmup_multibyte_chars() {
    assert_eq!(append_excl(String::from("café")), "café!");
}

// Main: swap_and_join

#[test]
fn main_typical() {
    assert_eq!(
        swap_and_join(String::from("hello"), String::from("world")),
        "world hello"
    );
}

#[test]
fn main_single_chars() {
    assert_eq!(swap_and_join(String::from("a"), String::from("b")), "b a");
}

#[test]
fn main_empty_first() {
    assert_eq!(swap_and_join(String::new(), String::from("hi")), "hi ");
}

#[test]
fn main_empty_second() {
    assert_eq!(swap_and_join(String::from("hi"), String::new()), " hi");
}
```

- [ ] **Step 3: Verify solution tests pass**

```bash
cargo test --package ownership-solutions
```

Expected: 8 tests pass.

- [ ] **Step 4: Verify lint passes on the solutions crate**

```bash
cargo clippy --package ownership-solutions --all-targets -- -D warnings
cargo fmt --check --package ownership-solutions
```

Expected: both exit 0. No `#[allow]` attributes needed. If clippy fires, STOP and report.

- [ ] **Step 5: Commit**

```bash
git add lessons/07-ownership/solutions
git commit -m "feat(lesson-07): add reference solutions"
```

---

## Task 4: Lesson README

**Files:**
- Overwrite: `lessons/07-ownership/README.md`

- [ ] **Step 1: Overwrite `lessons/07-ownership/README.md`**

The complete file content (write this as the file's actual content, starting with the `# Lesson 07` heading). Code fences inside the markdown are plain triple-backticks.

```markdown
# Lesson 07 — Ownership & moves

There is exactly one owner. When the owner goes out of scope, the
value is dropped. Everything that surprises you about Rust descends
from those two sentences — and today is where you learn what they
mean. Phase 2 of the course opens here.

## Learning goals

- State the three ownership rules and apply them when reading code
- Explain that assignment of a non-`Copy` value transfers ownership,
  leaving the original binding unusable
- Recognize when a function call moves a value versus when a `Copy`
  type is bitwise duplicated
- Use `.clone()` deliberately to escape a move when keeping the
  original is genuinely needed
- Write a function that takes ownership, mutates the value, and
  returns ownership — including the `mut s: String` parameter
  shorthand

## Self-study notes

### The three ownership rules

Rust's memory model has three rules:

1. Each value has **exactly one owner**.
2. There is **only one owner at a time**.
3. When the owner **goes out of scope**, the value is **dropped** —
   its memory is freed.

These rules aren't enforced at runtime. The compiler tracks ownership
at compile time, refuses to compile programs that break the rules,
and produces zero runtime cost. The whole story is a static analysis.

### Moves on assignment and function calls

Assigning a non-`Copy` value transfers ownership. The old binding is
dead:

​```rust
let s = String::from("hello");
let t = s;            // ownership moved: s -> t
println!("{t}");      // OK, t owns the value
// println!("{s}");   // ERROR: s no longer owns anything
​```

Passing a value to a function does the same thing — the parameter is
the new owner:

​```rust
fn take(s: String) { /* ... */ }

let s = String::from("hello");
take(s);              // s moved into the function
// take(s);           // ERROR: use after move
​```

This is "move semantics." It applies to any type that owns
heap-allocated data, including `String`.

### `Copy` types and why they're different

Primitive types like `i32`, `f64`, `bool`, and `char` implement the
`Copy` marker trait. Tuples and arrays of `Copy` types are themselves
`Copy`. For these types, assignment is bitwise duplication — both
bindings are valid:

​```rust
let n: i32 = 5;
let m = n;            // i32 is Copy — n is bitwise duplicated
println!("{n} {m}");  // both still valid
​```

The reason: copying a few stack bytes is cheap and has no aliasing
concerns. The reason `String` is *not* `Copy`: it owns a heap
allocation, and bit-copying it would give you two owners of the same
buffer — a recipe for double-free bugs. Rust's rule "only one owner"
is what makes the language safe without a garbage collector.

### `.clone()` — explicit duplication

When you genuinely need to keep using a value *and* pass it
somewhere, call `.clone()`:

​```rust
let s = String::from("hello");
take(s.clone());      // hand the function a copy
println!("{s}");      // original still owns
​```

Cloning a `String` allocates new heap memory and copies the bytes.
That's why it's explicit — the compiler refuses to do it silently, so
you're always aware when it happens.

For `Copy` types, `clone()` exists too but is redundant — assignment
already duplicates.

### Returning ownership from functions

Functions can take ownership and give it back. This is the pattern
for "transform a value":

​```rust
fn append_excl(mut s: String) -> String {
    s.push('!');
    s
}

let s = String::from("hi");
let s = append_excl(s);   // s moved in, mutated, returned, re-bound
​```

The `mut s` in the parameter list is a small new bit of syntax: it
says "the local binding for `s` inside this function is mutable."
Caller-side mutability and callee-side mutability are independent —
the caller doesn't have to declare anything `mut` to call this
function.

> You'll see `&` in some of the std-lib method calls you make (like
> `push_str(&a)`). That's borrowing — it's how you give a function
> read-access to a value without transferring ownership. We cover
> borrowing properly in **Lesson 08**; for now, just type the `&`
> where the compiler asks for it.

## Exercises

### Warm-up: `append_excl`

Implement `append_excl(mut s: String) -> String` in
`exercises/src/lib.rs`. The function takes ownership of `s`, pushes
`'!'` onto the end, and returns the modified `String`.

The `mut s` parameter binding makes `s` mutable inside the function
body. The signature still says `s: String` (no `mut` is visible to
the caller) — function-side mutability is local.

### Main: `swap_and_join`

Implement `swap_and_join(a: String, b: String) -> String` returning
`b` followed by a space followed by `a`. Both arguments are moved
into the function.

The reference solution uses the move-and-mutate pattern:

​```rust
let mut result = b;
result.push(' ');
result.push_str(&a);
result
​```

You could equivalently use `format!("{b} {a}")` — both pass the
tests. The mutation version is more pedagogical for this lesson; the
`format!` version is more idiomatic in real-world Rust.

### Compile-fail

`exercises/compile_fails/07-use-after-move.rs` ships with a `main()`
that passes the same `String` to a function twice. The second call is
a use-after-move and won't compile. Read the rustc error (it points
at both the move site and the use site), then add `.clone()` to the
first call.

### Run

​```bash
make verify LESSON=07-ownership
​```

This runs your exercise tests and asserts the compile-fail file now
compiles.

## Solutions

See `solutions/src/lib.rs` for the reference implementations. Try the
exercises before peeking.
```

> **IMPORTANT for the implementer:** the code-fence escape in the markdown above uses an invisible zero-width character (shown as `​```) in front of each triple-backtick block — that's only there so this plan file can nest fenced markdown inside an outer fenced markdown block. When you write the actual `README.md`, every fence must be three PLAIN backticks `` ``` `` with NO leading invisible character. After writing, `grep -c '^```' lessons/07-ownership/README.md` should return 14 (7 code blocks × 2 fence lines).

- [ ] **Step 2: Spot-check the README**

```bash
head -1 lessons/07-ownership/README.md
grep -c '^### ' lessons/07-ownership/README.md
grep -c '^```' lessons/07-ownership/README.md
```

Expected:
- First line: `# Lesson 07 — Ownership & moves`
- `grep -c '^### '` returns 9 (five subsections under self-study + four under exercises)
- `grep -c '^```'` returns 14 (7 code blocks × 2 fence lines)

- [ ] **Step 3: Commit**

```bash
git add lessons/07-ownership/README.md
git commit -m "docs(lesson-07): write self-study notes"
```

---

## Task 5: Slide deck

**Files:**
- Overwrite: `lessons/07-ownership/slides/slides.md`

- [ ] **Step 1: Overwrite `lessons/07-ownership/slides/slides.md`**

The complete file content (write this as the file's actual content, starting with the `# Ownership & moves` heading):

````
# Ownership & moves

> There is exactly one owner. When the owner goes out of scope, the value is dropped. Everything that surprises you about Rust descends from those two sentences.

---

## Recap

Phase 1 gave you the language's syntax: values, control flow, compound types, sum types, product types.

Today opens **Phase 2** — what Rust does that other languages don't. Ownership is the first piece, and the foundation for everything that follows (borrowing, lifetimes, smart pointers).

---

## The three rules

1. Each value has **exactly one owner**.
2. There is **only one owner at a time**.
3. When the owner **goes out of scope**, the value is **dropped**.

Every ownership puzzle in Rust resolves through these three rules.

---

## Move on assignment

```rust
let s = String::from("hello");
let t = s;          // ownership moved: s -> t
println!("{t}");

// println!("{s}"); // ERROR: s no longer owns anything
```

Assigning a non-`Copy` value transfers ownership. The old binding is dead.

The compiler tracks this — there's no runtime cost. It's all static.

---

## Move on function call

Functions take ownership through their parameters:

```rust
fn take(s: String) { /* ... */ }

let s = String::from("hello");
take(s);            // s moved into the function
// take(s);         // ERROR: use after move
```

The compile-fail exercise drills this exact case.

---

## `Copy` types skip the move

```rust
let n: i32 = 5;
let m = n;          // i32 is Copy — n is bitwise duplicated
println!("{n} {m}"); // both work
```

Implement `Copy`: `i32`, `f64`, `bool`, `char`, plus tuples of `Copy` types.

`String` does NOT — it owns heap data.

---

## `.clone()` — explicit duplication

When you need to keep using a value AND pass it somewhere:

```rust
let s = String::from("hello");
take(s.clone());    // hand the function a copy
println!("{s}");    // original still owns
```

`clone` is deliberately explicit. Rust doesn't auto-clone because cloning a `String` allocates new heap memory — you should be intentional.

---

## Returning ownership

Functions can take ownership and give it back:

```rust
fn append_excl(mut s: String) -> String {
    s.push('!');
    s
}

let s = String::from("hi");
let s = append_excl(s);    // s moved in, returned, re-bound
```

The `mut s` in the parameter list says "I want to mutate my local copy after taking ownership." Caller mutability and parameter mutability are independent.

---

## Putting it together

Today's exercises:

- **Warm-up** `append_excl(mut s: String) -> String` — take ownership, push `'!'`, return.
- **Main** `swap_and_join(a: String, b: String) -> String` — take two Strings, return them joined in reverse order.

Quick note: references (`&str`, `&self`) appear in std-lib methods you'll call. Those are **borrowing** — the topic of **Lesson 08**. For today, focus on what *your* function signatures say about ownership.

---

## Wrap — Phase 2 launched

- One owner at a time
- Assignment and function calls **move** non-`Copy` values
- `Copy` types skip the move (`i32`, `bool`, `char`, ...)
- **`.clone()`** is the explicit escape hatch
- Returning ownership is the cleanest pattern when a function transforms a value

Next: **Lesson 08 — References & borrowing**.
````

The OUTER fence above (quadruple backticks) is just delimiting this content in the plan. The FILE you write should NOT contain the outer fence — only the inner triple-backtick `rust` blocks.

The file should:
- Start with `# Ownership & moves` on line 1
- Have exactly 9 `---` slide separators (between 10 slides)
- Contain 5 triple-backtick `rust` code fences

- [ ] **Step 2: Verify `make slides-build` succeeds and includes lesson 07**

```bash
make slides-build
test -f dist/lessons/07-ownership/slides/slides.md
test -f dist/lessons/07-ownership/slides/index.html
grep -c "07-ownership" dist/index.html
```

Expected: `slides.md` and `index.html` copied into dist; `grep -c "07-ownership"` returns at least 1.

- [ ] **Step 3: Spot-check slide separators**

```bash
grep -c '^---$' lessons/07-ownership/slides/slides.md
```

Expected: `9` (between 10 slides).

- [ ] **Step 4: Commit**

```bash
git add lessons/07-ownership/slides/slides.md
git commit -m "feat(lesson-07): write slide deck"
```

---

## Task 6: End-to-end verification + push

- [ ] **Step 1: `make ci` is green**

```bash
make ci
```

Expected: exit 0. Clippy clean, fmt clean, workspace builds, default-members tests pass (now includes the 8 new tests in `ownership-solutions`), compile-fail `--expect broken` passes for lesson 07.

- [ ] **Step 2: `make verify LESSON=07-ownership` fails (the exercise is undone — intentional)**

```bash
make verify LESSON=07-ownership || echo "expected: exercise tests fail with todo!() panic"
```

Expected: non-zero exit. All 8 exercise tests panic with `not yet implemented`.

- [ ] **Step 3: `make slides-build` final state**

```bash
make slides-build
ls dist/lessons/
grep -c "07-ownership" dist/index.html
```

Expected: `dist/lessons/` contains all seven lessons (01-hello-rust through 07-ownership). `grep -c "07-ownership"` ≥ 1.

- [ ] **Step 4: Push**

```bash
git push
```

Expected: push succeeds. CI runs and is green; Deploy rebuilds the static site so lesson 07 appears live.

- [ ] **Step 5: Smoke-test the deployed site**

After the push, wait for the Deploy workflow to finish (`gh run watch <id>`). Then:

```bash
curl -sS -o /dev/null -w "%{http_code}\n" https://rust.ristkari.dev/
curl -sS -o /dev/null -w "%{http_code}\n" https://rust.ristkari.dev/lessons/07-ownership/slides/
```

Expected: both return `200`.

---

## Done criteria

- `lessons/07-ownership/` exists with all four parts
- `cargo test --package ownership-solutions` → 8 passing tests
- `cargo test --manifest-path lessons/07-ownership/exercises/Cargo.toml` → 8 panicking tests (intentional)
- `cargo run --package compile-fails -- --expect broken lessons/07-ownership` → ok
- `cargo run --package compile-fails -- --expect compiles lessons/07-ownership` → fails (intentional)
- `make ci` → green
- `make slides-build` → produces `dist/lessons/07-ownership/slides/index.html`
- `dist/index.html` lists lesson 07 as a clickable link
- All changes committed and pushed
- Deployed site returns HTTP 200 for `/` and `/lessons/07-ownership/slides/`

**Phase 2 launched after this lesson lands.**
