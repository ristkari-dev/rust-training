# Lesson 04 — Compound types — Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Author the fourth lesson of the Rust training course: a warm-up + main exercise pair on tuples (`divmod` returning `(u32, u32)`) and slices+`String` (`join_with_dashes` taking `&[&str]` and returning `String`), with one compile-fail exercise on the `String` vs `&str` literal mismatch.

**Architecture:** Use the existing `make new-lesson` scaffolder to lay down the four-part lesson structure, then overwrite the placeholder content with lesson-specific README, slides, exercises, and solutions per the design spec. The workspace `members` glob picks the new crates up automatically.

**Tech Stack:** Rust 2024 edition, existing tools (`new-lesson`, `compile-fails`, `slides-dev`, `build-index`), reveal.js (vendored), GNU Make.

**Spec:** [`docs/superpowers/specs/2026-05-25-lesson-04-compound-types-design.md`](../specs/2026-05-25-lesson-04-compound-types-design.md).

**Working directory:** `/Users/ristkari/code/private/rust-training`.

---

## Task 1: Scaffold lessons/04-compound-types

**Files (all created by the scaffolder):**
- `lessons/04-compound-types/README.md` (placeholder, replaced in Task 4)
- `lessons/04-compound-types/slides/index.html` (final — no edit needed)
- `lessons/04-compound-types/slides/slides.md` (placeholder, replaced in Task 5)
- `lessons/04-compound-types/exercises/Cargo.toml` (final — no edit needed)
- `lessons/04-compound-types/exercises/src/lib.rs` (placeholder, replaced in Task 2)
- `lessons/04-compound-types/exercises/tests/exercise.rs` (placeholder, replaced in Task 2)
- `lessons/04-compound-types/solutions/Cargo.toml` (final — no edit needed)
- `lessons/04-compound-types/solutions/src/lib.rs` (placeholder, replaced in Task 3)
- `lessons/04-compound-types/solutions/tests/exercise.rs` (placeholder, replaced in Task 3)

- [ ] **Step 1: Run the scaffolder**

```bash
make new-lesson NAME=04-compound-types
```

Expected: `scaffolded ./lessons/04-compound-types`.

- [ ] **Step 2: Verify directory structure**

```bash
ls lessons/04-compound-types/
ls lessons/04-compound-types/slides/ lessons/04-compound-types/exercises/ lessons/04-compound-types/solutions/
```

Expected: top-level `README.md`, `slides/`, `exercises/`, `solutions/` populated from templates.

- [ ] **Step 3: Verify Cargo package names**

```bash
grep '^name' lessons/04-compound-types/exercises/Cargo.toml lessons/04-compound-types/solutions/Cargo.toml
```

Expected:
```
lessons/04-compound-types/exercises/Cargo.toml:name = "compound-types-exercises"
lessons/04-compound-types/solutions/Cargo.toml:name = "compound-types-solutions"
```

- [ ] **Step 4: Verify the workspace picks up the new crates**

```bash
cargo metadata --no-deps --format-version 1 | grep -o '"name":"compound-types-[^"]*"' | sort -u
```

Expected output:
```
"name":"compound-types-exercises"
"name":"compound-types-solutions"
```

- [ ] **Step 5: Verify the scaffolded workspace builds clean**

```bash
cargo build --workspace
```

Expected: warning-free build.

- [ ] **Step 6: Commit the scaffold**

```bash
git add lessons/04-compound-types
git commit -m "chore: scaffold lessons/04-compound-types"
```

---

## Task 2: Exercise content (stubs + tests + compile-fail)

**Files:**
- Overwrite: `lessons/04-compound-types/exercises/src/lib.rs`
- Overwrite: `lessons/04-compound-types/exercises/tests/exercise.rs`
- Create: `lessons/04-compound-types/exercises/compile_fails/04-string-vs-str.rs`

- [ ] **Step 1: Overwrite `lessons/04-compound-types/exercises/src/lib.rs`**

```rust
//! Lesson 04 — exercises.
//!
//! Implement `divmod` (warm-up) and `join_with_dashes` (main) so that
//! `cargo test --manifest-path lessons/04-compound-types/exercises/Cargo.toml`
//! passes. The tests live in `tests/exercise.rs`.

#[must_use]
pub fn divmod(_a: u32, _b: u32) -> (u32, u32) {
    todo!("return (quotient, remainder)")
}

#[must_use]
pub fn join_with_dashes(_words: &[&str]) -> String {
    todo!("join words with '-' between them; return \"\" for empty input")
}
```

- [ ] **Step 2: Overwrite `lessons/04-compound-types/exercises/tests/exercise.rs`**

```rust
use compound_types_exercises::{divmod, join_with_dashes};

// Warm-up: divmod

#[test]
fn warmup_typical() {
    assert_eq!(divmod(10, 3), (3, 1));
}

#[test]
fn warmup_exact() {
    assert_eq!(divmod(20, 4), (5, 0));
}

#[test]
fn warmup_divide_by_one() {
    assert_eq!(divmod(7, 1), (7, 0));
}

#[test]
fn warmup_zero_dividend() {
    assert_eq!(divmod(0, 5), (0, 0));
}

// Main: join_with_dashes

#[test]
fn main_empty() {
    assert_eq!(join_with_dashes(&[]), "");
}

#[test]
fn main_single() {
    assert_eq!(join_with_dashes(&["solo"]), "solo");
}

#[test]
fn main_two() {
    assert_eq!(join_with_dashes(&["a", "b"]), "a-b");
}

#[test]
fn main_three() {
    assert_eq!(join_with_dashes(&["red", "green", "blue"]), "red-green-blue");
}
```

- [ ] **Step 3: Create `lessons/04-compound-types/exercises/compile_fails/04-string-vs-str.rs`**

```rust
// Compile-fail exercise: this file MUST NOT compile until you fix it.
//
// String literals like "hello" are NOT `String` — they're `&'static str`,
// slices into the program binary itself. The binding below declares its
// type as `String`, but the value on the right is a `&str`. rustc will
// tell you exactly that.
//
// Hint: convert the literal into an owned `String`. Two equivalent
// idioms:
//
//     let s: String = "hello".to_string();
//     let s: String = String::from("hello");
//
// Pick either.

fn greet() -> String {
    let s: String = "hello";
    s
}

fn main() {
    let g = greet();
    println!("{g}");
}
```

- [ ] **Step 4: Verify exercise tests fail with `todo!()` panic (intentional)**

```bash
cargo test --manifest-path lessons/04-compound-types/exercises/Cargo.toml
```

Expected: all 8 tests fail with `not yet implemented` panic message. This is the intentional shipped state.

- [ ] **Step 5: Verify the exercises crate still builds cleanly**

```bash
cargo build --package compound-types-exercises
```

Expected: warning-free build.

- [ ] **Step 6: Verify compile-fail ships broken (the author/CI check)**

```bash
cargo run --package compile-fails -- --expect broken lessons/04-compound-types
```

Expected: `ok: lessons/04-compound-types/exercises/compile_fails/04-string-vs-str.rs` and exit 0.

- [ ] **Step 7: Verify compile-fail's student-mode check fires (the exercise hasn't been fixed yet)**

```bash
cargo run --package compile-fails -- --expect compiles lessons/04-compound-types
```

Expected: non-zero exit with `FAIL: file did not compile, but was expected to: lessons/04-compound-types/...`.

- [ ] **Step 8: Verify lint passes on the exercises crate**

```bash
cargo clippy --package compound-types-exercises --all-targets -- -D warnings
cargo fmt --check --package compound-types-exercises
```

Expected: both exit 0.

- [ ] **Step 9: Commit**

```bash
git add lessons/04-compound-types/exercises
git commit -m "feat(lesson-04): add warm-up + main exercise stubs, tests, and compile-fail"
```

---

## Task 3: Reference solutions

**Files:**
- Overwrite: `lessons/04-compound-types/solutions/src/lib.rs`
- Overwrite: `lessons/04-compound-types/solutions/tests/exercise.rs`

- [ ] **Step 1: Overwrite `lessons/04-compound-types/solutions/src/lib.rs`**

```rust
//! Lesson 04 — reference solutions.

#[must_use]
pub fn divmod(a: u32, b: u32) -> (u32, u32) {
    (a / b, a % b)
}

#[must_use]
pub fn join_with_dashes(words: &[&str]) -> String {
    let mut out = String::new();
    let mut first = true;
    for w in words {
        if !first {
            out.push('-');
        }
        out.push_str(w);
        first = false;
    }
    out
}
```

> Pedagogical notes baked into the solutions:
>
> - `divmod` is a single tail expression building a tuple. No `let`, no `return` — the canonical "build and return" pattern.
> - `join_with_dashes` synthesizes the lesson: `&[&str]` parameter (slice of slices), `String::new()`, iterating a slice with `for w in words`, the `let mut first = true;` flag pattern (do-something-between-but-not-before-first), and tail expression returning the owned `String`.
> - `out.push_str(w)` where `w: &&str` works invisibly via deref coercion. The student does NOT need to write `*w` — the type-level cleverness is implicit.
>
> No `#[allow]` attributes should be needed. If clippy fires, fix the code rather than allow-list.

- [ ] **Step 2: Overwrite `lessons/04-compound-types/solutions/tests/exercise.rs`**

```rust
use compound_types_solutions::{divmod, join_with_dashes};

// Warm-up: divmod

#[test]
fn warmup_typical() {
    assert_eq!(divmod(10, 3), (3, 1));
}

#[test]
fn warmup_exact() {
    assert_eq!(divmod(20, 4), (5, 0));
}

#[test]
fn warmup_divide_by_one() {
    assert_eq!(divmod(7, 1), (7, 0));
}

#[test]
fn warmup_zero_dividend() {
    assert_eq!(divmod(0, 5), (0, 0));
}

// Main: join_with_dashes

#[test]
fn main_empty() {
    assert_eq!(join_with_dashes(&[]), "");
}

#[test]
fn main_single() {
    assert_eq!(join_with_dashes(&["solo"]), "solo");
}

#[test]
fn main_two() {
    assert_eq!(join_with_dashes(&["a", "b"]), "a-b");
}

#[test]
fn main_three() {
    assert_eq!(join_with_dashes(&["red", "green", "blue"]), "red-green-blue");
}
```

- [ ] **Step 3: Verify solution tests pass**

```bash
cargo test --package compound-types-solutions
```

Expected: 8 tests pass.

- [ ] **Step 4: Verify lint passes on the solutions crate**

```bash
cargo clippy --package compound-types-solutions --all-targets -- -D warnings
cargo fmt --check --package compound-types-solutions
```

Expected: both exit 0. No `#[allow]` attributes needed for this lesson. If clippy fires, STOP and report rather than allow-listing.

- [ ] **Step 5: Commit**

```bash
git add lessons/04-compound-types/solutions
git commit -m "feat(lesson-04): add reference solutions"
```

---

## Task 4: Lesson README

**Files:**
- Overwrite: `lessons/04-compound-types/README.md`

- [ ] **Step 1: Overwrite `lessons/04-compound-types/README.md`**

The complete file content (write this as the file's actual content, starting with the `# Lesson 04` heading):

```markdown
# Lesson 04 — Compound types

Rust has two ways to talk about a sequence of values: own them, or look
at them through a slice. Once you see this distinction, large swathes of
the standard library — including the `String`/`&str` split that famously
trips up beginners — fall into place.

## Learning goals

- Construct, destructure, and index tuples
- Read and use fixed-size arrays
- Take a slice `&[T]` as a parameter so functions work for any-length
  inputs
- Distinguish `String` (owned, heap) from `&str` (borrowed view) and
  choose the right one for a given API
- Convert between `&str` and `String`, and recognize the canonical
  signature pattern *take `&str`, return `String`*

## Self-study notes

### Tuples

A tuple groups a fixed number of values, possibly of different types:

​```rust
let pair: (i32, &str) = (42, "hi");
let n = pair.0;     // access by position
let s = pair.1;
let (a, b) = pair;  // or destructure into named bindings
​```

The tuple's type lists each field's type in order. Common shapes: `()`
(the empty tuple, also called the unit type), `(T,)` (a one-element
tuple — note the trailing comma), `(T, U)`, `(T, U, V)`, and so on.

### Arrays

An array is a fixed-size, same-type sequence:

​```rust
let arr: [i32; 5] = [1, 2, 3, 4, 5];
let first = arr[0];     // index access
let len = arr.len();    // 5
​```

The length is **part of the type**: `[i32; 5]` and `[i32; 6]` are
different types. You generally don't pass arrays around by value (the
size would force every caller to use the same length); you pass slices.

### Slices

A slice `&[T]` is a **borrowed view** into a sequence: a pointer + length,
but no ownership of the data. You can create one with the range syntax:

​```rust
let arr = [10, 20, 30, 40, 50];
let middle: &[i32] = &arr[1..4];   // [20, 30, 40]
​```

The big payoff is in function signatures:

​```rust
fn sum(numbers: &[i32]) -> i32 {
    let mut total = 0;
    for n in numbers {
        total += n;
    }
    total
}
​```

`sum` works for any size of array, any sub-slice of an array, and (later)
any `Vec`. One function, many callers.

### `String` vs `&str`

`&str` is a slice — specifically, a slice of bytes guaranteed to be valid
UTF-8. String literals like `"hello"` are `&'static str` (slices into
the program binary).

`String` is the owned, heap-allocated, growable counterpart:

​```rust
let mut s = String::new();
s.push_str("hello");
s.push(',');
s.push_str(" world");
// s == "hello, world"
​```

`String` is what you build at runtime. `&str` is what you pass around
when you just need to look at a string without owning it.

### Conversions and signature patterns

Going between them is direct:

​```rust
let owned: String = "hi".to_string();
let owned: String = String::from("hi");

let owned = String::from("hello");
let view: &str = &owned;        // &String auto-derefs to &str
​```

The idiomatic signature pattern is **take `&str`, return `String`**:

​```rust
fn greet(name: &str) -> String {
    let mut out = String::from("Hello, ");
    out.push_str(name);
    out.push('!');
    out
}
​```

Taking `&str` lets the caller pass either a literal or a borrowed
`String`. Returning `String` makes it clear the function owns the result
and can return new data.

## Exercises

### Warm-up: `divmod`

Implement `divmod(a: u32, b: u32) -> (u32, u32)` in `exercises/src/lib.rs`
returning `(quotient, remainder)`. The function body should be a single
tail expression — no `let`, no `return`, just build and return the tuple.

### Main: `join_with_dashes`

Implement `join_with_dashes(words: &[&str]) -> String` so it joins all
the words with a `'-'` between them and returns the result as an owned
`String`. An empty input returns `""`.

You'll likely want:

- `String::new()` to start with an empty owned string
- A `let mut first = true;` flag so you only insert `'-'` between items
- A `for w in words` loop
- `out.push('-')` for the separator and `out.push_str(w)` for each word
- A tail expression that returns `out`

### Compile-fail

`exercises/compile_fails/04-string-vs-str.rs` ships in a state that
does **not** compile. The function declares it returns `String` but
binds a `&str` literal to a `String`-typed variable. Read the rustc
error, then convert the literal with `.to_string()` or `String::from(...)`.

### Run

​```bash
make verify LESSON=04-compound-types
​```

This runs your exercise tests and asserts the compile-fail file now
compiles.

## Solutions

See `solutions/src/lib.rs` for the reference implementations. Try the
exercises before peeking.
```

> **IMPORTANT for the implementer:** the code-fence escape in the markdown above uses an invisible zero-width character (shown as `​```) in front of each triple-backtick BLOCK — that's only there so this plan file can nest fenced markdown inside an outer fenced markdown block. When you write the actual `README.md`, every fence must be three PLAIN backticks `` ``` `` with NO leading invisible character. The actual file should contain seven Rust code blocks plus one bash code block = 8 code blocks = 16 fence lines.

- [ ] **Step 2: Spot-check the README**

```bash
head -1 lessons/04-compound-types/README.md
grep -c '^### ' lessons/04-compound-types/README.md
grep -c '^```' lessons/04-compound-types/README.md
```

Expected:
- First line: `# Lesson 04 — Compound types`
- `grep -c '^### '` returns 9 (five subsections under self-study + four under exercises)
- `grep -c '^```'` returns 16 (8 code blocks × 2 fence lines)

- [ ] **Step 3: Commit**

```bash
git add lessons/04-compound-types/README.md
git commit -m "docs(lesson-04): write self-study notes"
```

---

## Task 5: Slide deck

**Files:**
- Overwrite: `lessons/04-compound-types/slides/slides.md`

- [ ] **Step 1: Overwrite `lessons/04-compound-types/slides/slides.md`**

The complete file content (write this as the file's actual content, starting with the `# Compound types` heading):

````
# Compound types

> Rust has two ways to talk about a sequence of values: own them, or look at them through a slice. Most of the standard library is built on this distinction.

---

## Recap

Lessons 01-03 covered single values, mutation, and control flow.

Today: group values into tuples and arrays, then meet the **slice** — the abstraction that makes those groups usable across function boundaries.

---

## Tuples

```rust
let pair: (i32, &str) = (42, "hi");

let n = pair.0;     // 42
let s = pair.1;     // "hi"

let (n, s) = pair;  // destructure
```

- Mix types in a single value
- Access by position with `.0`, `.1`, etc.
- Destructure with `let (a, b) = ...`

---

## Arrays

```rust
let arr: [i32; 5] = [1, 2, 3, 4, 5];
let first = arr[0];      // 1
let len = arr.len();     // 5
```

- Fixed size, same element type
- Length is part of the type: `[i32; 5]` and `[i32; 6]` are different
- Index with `arr[i]`

---

## Slices

`&[T]` is a **borrowed view** into a sequence:

```rust
let arr = [1, 2, 3, 4, 5];
let middle: &[i32] = &arr[1..4];   // [2, 3, 4]

fn sum(numbers: &[i32]) -> i32 {
    let mut total = 0;
    for n in numbers {
        total += n;
    }
    total
}
```

A function taking `&[i32]` works for **any size** of array. This is huge — it decouples your function from a specific length.

---

## `&str` is a slice too

Where `&[u8]` is a view of raw bytes, `&str` is a view of bytes **guaranteed to be valid UTF-8**.

```rust
let greeting: &str = "hello";        // a slice into the binary
let slice: &str = &greeting[0..3];   // "hel"
```

String literals like `"hello"` are `&'static str` — slices pointing into the program's compiled binary itself.

---

## `String` — owned, heap-allocated

```rust
let mut s = String::new();
s.push_str("hello");
s.push(',');
s.push_str(" world");
// s == "hello, world"

let from_literal = String::from("hi");
let from_method  = "hi".to_string();
```

- Heap-allocated and growable
- Owned (we'll see what "owned" really means in Lesson 07)
- Most string data you build at runtime is a `String`

---

## `String` vs `&str` — when to use which

Rule of thumb:

- **Take `&str` as a parameter** — works for both `String` (via `&s`) and string literals
- **Return `String`** — when you build new data

Conversions:

```rust
let s: String = "hello".to_string();
let s: String = String::from("hello");

let owned: String = String::from("hello");
let view: &str = &owned;        // auto-deref from &String to &str
```

---

## Putting it together

```rust
fn join_with_dashes(words: &[&str]) -> String {
    let mut out = String::new();
    let mut first = true;
    for w in words {
        if !first {
            out.push('-');
        }
        out.push_str(w);
        first = false;
    }
    out
}
```

- `&[&str]` — slice of borrowed string slices
- `String::new()` + `push_str` + `push` to build up
- Tail returns the owned `String`

---

## Wrap

- **Tuples** mix types: `(T, U, ...)` with `.0`/`.1` access and destructuring
- **Arrays** are fixed-size and same-type: `[T; N]`
- **Slices** `&[T]` are borrowed views that decouple functions from sizes
- **`&str`** is a slice; **`String`** is owned and heap-allocated
- The canonical signature pattern: *take `&str`, return `String`*

Next: Lesson 05 — pattern matching & enums.
````

The OUTER fence above (quadruple backticks) is just delimiting this content in the plan. The FILE you write should NOT contain the outer fence — only the inner triple-backtick `rust` blocks.

The file should:
- Start with `# Compound types` on line 1
- Have exactly 9 `---` slide separators (between 10 slides)
- Contain 7 triple-backtick `rust` code fences

- [ ] **Step 2: Verify `make slides-build` succeeds and includes lesson 04**

```bash
make slides-build
test -f dist/lessons/04-compound-types/slides/slides.md
test -f dist/lessons/04-compound-types/slides/index.html
grep -c "04-compound-types" dist/index.html
```

Expected: `slides.md` and `index.html` copied into dist; `grep -c "04-compound-types"` returns at least 1.

- [ ] **Step 3: Spot-check slide separators**

```bash
grep -c '^---$' lessons/04-compound-types/slides/slides.md
```

Expected: `9` (between 10 slides).

- [ ] **Step 4: Commit**

```bash
git add lessons/04-compound-types/slides/slides.md
git commit -m "feat(lesson-04): write slide deck"
```

---

## Task 6: End-to-end verification + push

- [ ] **Step 1: `make ci` is green**

```bash
make ci
```

Expected: exit 0. Clippy clean, fmt clean, workspace builds, default-members tests pass (now includes the 8 new tests in `compound-types-solutions`), compile-fail `--expect broken` passes for lesson 04.

- [ ] **Step 2: `make verify LESSON=04-compound-types` fails (the exercise is undone — intentional)**

```bash
make verify LESSON=04-compound-types || echo "expected: exercise tests fail with todo!() panic"
```

Expected: non-zero exit. All 8 exercise tests panic with `not yet implemented`. The student fixing the exercise is what makes this pass.

- [ ] **Step 3: `make slides-build` final state**

```bash
make slides-build
ls dist/lessons/
grep -c "04-compound-types" dist/index.html
```

Expected: `dist/lessons/` contains `01-hello-rust`, `02-variables`, `03-control-flow`, and `04-compound-types`. `grep -c "04-compound-types"` ≥ 1.

- [ ] **Step 4: Push**

```bash
git push
```

Expected: push succeeds. CI runs and is green; Deploy rebuilds the static site so lesson 04 appears live.

- [ ] **Step 5: Smoke-test the deployed site**

After the push, wait for the Deploy workflow to finish (`gh run watch <id>`). Then:

```bash
curl -sS -o /dev/null -w "%{http_code}\n" https://rust.ristkari.dev/
curl -sS -o /dev/null -w "%{http_code}\n" https://rust.ristkari.dev/lessons/04-compound-types/slides/
```

Expected: both return `200`.

---

## Done criteria

- `lessons/04-compound-types/` exists with all four parts
- `cargo test --package compound-types-solutions` → 8 passing tests
- `cargo test --manifest-path lessons/04-compound-types/exercises/Cargo.toml` → 8 panicking tests (intentional)
- `cargo run --package compile-fails -- --expect broken lessons/04-compound-types` → ok
- `cargo run --package compile-fails -- --expect compiles lessons/04-compound-types` → fails (intentional)
- `make ci` → green
- `make slides-build` → produces `dist/lessons/04-compound-types/slides/index.html`
- `dist/index.html` lists lesson 04 as a clickable link
- All changes committed and pushed
- Deployed site returns HTTP 200 for `/` and `/lessons/04-compound-types/slides/`
