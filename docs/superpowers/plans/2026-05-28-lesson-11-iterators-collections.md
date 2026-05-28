# Lesson 11 — Iterators & collections — Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Author the fifth and final lesson of Phase 2 of the Rust training course: iterators & collections. Iterators are the spine — *data in a collection → transform with a chain → collect a result*. Warm-up is `evens_squared` (a `filter`/`map`/`collect` chain over a slice). Main is a word counter: `word_frequencies` (the `HashMap` entry API) plus `most_frequent` (iterating the map with `max_by_key`). Compile-fail is "mutate a `Vec` while iterating it" (E0502), tying iterators back to the borrow rules from Lessons 8-9.

**Architecture:** Use the existing `make new-lesson` scaffolder. The exercise crate ships three function stubs; students implement the bodies. The compile-fail reinforces that the same borrow rules govern iterators.

**Tech Stack:** Rust 2024 edition, existing tools (`new-lesson`, `compile-fails`, `slides-dev`, `build-index`), reveal.js (vendored), GNU Make.

**Spec:** [`docs/superpowers/specs/2026-05-28-lesson-11-iterators-collections-design.md`](../specs/2026-05-28-lesson-11-iterators-collections-design.md).

**Working directory:** `/Users/ristkari/code/private/rust-training`.

---

## Task 1: Scaffold lessons/11-iterators-collections

**Files (all created by the scaffolder):**
- `lessons/11-iterators-collections/README.md` (placeholder, replaced in Task 4)
- `lessons/11-iterators-collections/slides/index.html` (final — no edit needed)
- `lessons/11-iterators-collections/slides/slides.md` (placeholder, replaced in Task 5)
- `lessons/11-iterators-collections/exercises/Cargo.toml` (final — no edit needed)
- `lessons/11-iterators-collections/exercises/src/lib.rs` (placeholder, replaced in Task 2)
- `lessons/11-iterators-collections/exercises/tests/exercise.rs` (placeholder, replaced in Task 2)
- `lessons/11-iterators-collections/solutions/Cargo.toml` (final — no edit needed)
- `lessons/11-iterators-collections/solutions/src/lib.rs` (placeholder, replaced in Task 3)
- `lessons/11-iterators-collections/solutions/tests/exercise.rs` (placeholder, replaced in Task 3)

- [ ] **Step 1: Run the scaffolder**

```bash
make new-lesson NAME=11-iterators-collections
```

Expected: `scaffolded ./lessons/11-iterators-collections`.

- [ ] **Step 2: Verify directory structure**

```bash
ls lessons/11-iterators-collections/
ls lessons/11-iterators-collections/slides/ lessons/11-iterators-collections/exercises/ lessons/11-iterators-collections/solutions/
```

Expected: top-level `README.md`, `slides/`, `exercises/`, `solutions/` populated from templates.

- [ ] **Step 3: Verify Cargo package names**

```bash
grep '^name' lessons/11-iterators-collections/exercises/Cargo.toml lessons/11-iterators-collections/solutions/Cargo.toml
```

Expected:
```
lessons/11-iterators-collections/exercises/Cargo.toml:name = "iterators-collections-exercises"
lessons/11-iterators-collections/solutions/Cargo.toml:name = "iterators-collections-solutions"
```

- [ ] **Step 4: Verify the workspace picks up the new crates**

```bash
cargo metadata --no-deps --format-version 1 | grep -o '"name":"iterators-collections-[^"]*"' | sort -u
```

Expected output:
```
"name":"iterators-collections-exercises"
"name":"iterators-collections-solutions"
```

- [ ] **Step 5: Verify the scaffolded workspace builds clean**

```bash
cargo build --workspace
```

Expected: warning-free build.

- [ ] **Step 6: Commit the scaffold**

```bash
git add lessons/11-iterators-collections
git commit -m "chore: scaffold lessons/11-iterators-collections"
```

---

## Task 2: Exercise content (stubs + tests + compile-fail)

**Files:**
- Overwrite: `lessons/11-iterators-collections/exercises/src/lib.rs`
- Overwrite: `lessons/11-iterators-collections/exercises/tests/exercise.rs`
- Create: `lessons/11-iterators-collections/exercises/compile_fails/11-mutate-while-iterating.rs`

- [ ] **Step 1: Overwrite `lessons/11-iterators-collections/exercises/src/lib.rs`**

```rust
//! Lesson 11 — exercises.
//!
//! Implement `evens_squared` (warm-up) and `word_frequencies` +
//! `most_frequent` (main) so that `cargo test --manifest-path
//! lessons/11-iterators-collections/exercises/Cargo.toml` passes. The
//! tests live in `tests/exercise.rs`.

use std::collections::HashMap;

#[must_use]
pub fn evens_squared(_nums: &[i32]) -> Vec<i32> {
    todo!("filter the even numbers, square each, and collect into a Vec")
}

#[must_use]
pub fn word_frequencies(_text: &str) -> HashMap<String, usize> {
    todo!("count occurrences of each whitespace-separated word")
}

#[must_use]
pub fn most_frequent(_text: &str) -> Option<String> {
    todo!("return the word with the highest count, or None if empty")
}
```

- [ ] **Step 2: Overwrite `lessons/11-iterators-collections/exercises/tests/exercise.rs`**

```rust
use iterators_collections_exercises::{evens_squared, most_frequent, word_frequencies};

// Warm-up: evens_squared

#[test]
fn warmup_empty() {
    assert_eq!(evens_squared(&[]), Vec::<i32>::new());
}

#[test]
fn warmup_all_odd() {
    assert_eq!(evens_squared(&[1, 3, 5]), Vec::<i32>::new());
}

#[test]
fn warmup_mixed() {
    assert_eq!(evens_squared(&[1, 2, 3, 4]), vec![4, 16]);
}

#[test]
fn warmup_all_even() {
    assert_eq!(evens_squared(&[2, 4, 6]), vec![4, 16, 36]);
}

// Main: word_frequencies + most_frequent

#[test]
fn main_frequencies_empty() {
    assert!(word_frequencies("").is_empty());
}

#[test]
fn main_frequencies_counts() {
    let counts = word_frequencies("the cat the dog the");
    assert_eq!(counts.get("the"), Some(&3));
    assert_eq!(counts.get("cat"), Some(&1));
    assert_eq!(counts.get("dog"), Some(&1));
    assert_eq!(counts.len(), 3);
}

#[test]
fn main_most_frequent_none_for_empty() {
    assert_eq!(most_frequent(""), None);
}

#[test]
fn main_most_frequent_unique_winner() {
    assert_eq!(most_frequent("apple apple banana"), Some("apple".to_string()));
}
```

- [ ] **Step 3: Create `lessons/11-iterators-collections/exercises/compile_fails/11-mutate-while-iterating.rs`**

```rust
// Compile-fail exercise: this file MUST NOT compile until you fix it.
//
// Iterating over a collection borrows it. `for x in &v` takes a shared
// (&) borrow of `v` that lasts the whole loop. While that shared borrow
// is alive you cannot ALSO take a mutable borrow — and `v.push(...)`
// needs `&mut v`. The borrow rules from Lessons 8-9 forbid a shared and
// a mutable borrow of the same value at the same time.
//
// rustc will say "cannot borrow `v` as mutable because it is also
// borrowed as immutable" (E0502).
//
// The fix: don't mutate a collection while iterating it. Collect what
// you want into a NEW Vec, then you never hold two borrows of `v` at
// once. For example:
//
//     let doubled: Vec<i32> = v.iter().map(|x| x * 2).collect();
//
// Hint: build a separate `Vec` from the chain instead of pushing into
// `v` inside the loop.

fn main() {
    let mut v = vec![1, 2, 3];
    for x in &v {
        v.push(*x);
    }
    println!("{v:?}");
}
```

- [ ] **Step 4: Verify exercise tests fail with `todo!()` panic (intentional)**

```bash
cargo test --manifest-path lessons/11-iterators-collections/exercises/Cargo.toml
```

Expected: all 8 tests fail with `not yet implemented` panic message.

- [ ] **Step 5: Verify the exercises crate still builds cleanly**

```bash
cargo build --package iterators-collections-exercises
```

Expected: warning-free build.

- [ ] **Step 6: Verify compile-fail ships broken**

```bash
cargo run --package compile-fails -- --expect broken lessons/11-iterators-collections
```

Expected: `ok: lessons/11-iterators-collections/exercises/compile_fails/11-mutate-while-iterating.rs` and exit 0.

- [ ] **Step 7: Verify compile-fail's student-mode check fires**

```bash
cargo run --package compile-fails -- --expect compiles lessons/11-iterators-collections
```

Expected: non-zero exit with `FAIL: file did not compile, but was expected to: lessons/11-iterators-collections/...`.

- [ ] **Step 8: Verify lint passes on the exercises crate**

```bash
cargo clippy --package iterators-collections-exercises --all-targets -- -D warnings
cargo fmt --check --package iterators-collections-exercises
```

Expected: both exit 0.

- [ ] **Step 9: Commit**

```bash
git add lessons/11-iterators-collections/exercises
git commit -m "feat(lesson-11): add exercise stubs, tests, and compile-fail"
```

---

## Task 3: Reference solutions

**Files:**
- Overwrite: `lessons/11-iterators-collections/solutions/src/lib.rs`
- Overwrite: `lessons/11-iterators-collections/solutions/tests/exercise.rs`

- [ ] **Step 1: Overwrite `lessons/11-iterators-collections/solutions/src/lib.rs`**

```rust
//! Lesson 11 — reference solutions.

use std::collections::HashMap;

#[must_use]
pub fn evens_squared(nums: &[i32]) -> Vec<i32> {
    nums.iter()
        .copied()
        .filter(|&n| n % 2 == 0)
        .map(|n| n * n)
        .collect()
}

#[must_use]
pub fn word_frequencies(text: &str) -> HashMap<String, usize> {
    let mut counts = HashMap::new();
    for word in text.split_whitespace() {
        *counts.entry(word.to_string()).or_insert(0) += 1;
    }
    counts
}

#[must_use]
pub fn most_frequent(text: &str) -> Option<String> {
    word_frequencies(text)
        .into_iter()
        .max_by_key(|(_word, count)| *count)
        .map(|(word, _count)| word)
}
```

> Pedagogical notes:
> - `evens_squared` is the lesson's spine. `iter()` yields `&i32`; `copied()` turns that into `i32`; `filter` hands its closure a *reference*, so `|&n|` destructures it back to a plain `i32`; `map` squares; `collect()` builds the `Vec` (the return type tells `collect` what to build).
> - `word_frequencies` uses the counting idiom `*map.entry(key).or_insert(0) += 1` — `entry().or_insert(0)` returns a `&mut usize`, creating the entry with `0` first if absent.
> - `most_frequent` consumes the map with `into_iter()` (yielding owned `(String, usize)` pairs, so the winning `String` moves out — no clone), finds the largest by count with `max_by_key`, then `map` keeps just the word.
> - No `#[allow]` attributes should be needed. If clippy fires unexpectedly, fix the code rather than adding an allow, and report the deviation.

- [ ] **Step 2: Overwrite `lessons/11-iterators-collections/solutions/tests/exercise.rs`**

```rust
use iterators_collections_solutions::{evens_squared, most_frequent, word_frequencies};

// Warm-up: evens_squared

#[test]
fn warmup_empty() {
    assert_eq!(evens_squared(&[]), Vec::<i32>::new());
}

#[test]
fn warmup_all_odd() {
    assert_eq!(evens_squared(&[1, 3, 5]), Vec::<i32>::new());
}

#[test]
fn warmup_mixed() {
    assert_eq!(evens_squared(&[1, 2, 3, 4]), vec![4, 16]);
}

#[test]
fn warmup_all_even() {
    assert_eq!(evens_squared(&[2, 4, 6]), vec![4, 16, 36]);
}

// Main: word_frequencies + most_frequent

#[test]
fn main_frequencies_empty() {
    assert!(word_frequencies("").is_empty());
}

#[test]
fn main_frequencies_counts() {
    let counts = word_frequencies("the cat the dog the");
    assert_eq!(counts.get("the"), Some(&3));
    assert_eq!(counts.get("cat"), Some(&1));
    assert_eq!(counts.get("dog"), Some(&1));
    assert_eq!(counts.len(), 3);
}

#[test]
fn main_most_frequent_none_for_empty() {
    assert_eq!(most_frequent(""), None);
}

#[test]
fn main_most_frequent_unique_winner() {
    assert_eq!(most_frequent("apple apple banana"), Some("apple".to_string()));
}
```

- [ ] **Step 3: Verify solution tests pass**

```bash
cargo test --package iterators-collections-solutions
```

Expected: 8 tests pass.

- [ ] **Step 4: Verify lint passes on the solutions crate**

```bash
cargo clippy --package iterators-collections-solutions --all-targets -- -D warnings
cargo fmt --check --package iterators-collections-solutions
```

Expected: both exit 0. No `#[allow]` attributes needed. If clippy fires on anything, fix the code (not with an allow) and report it.

- [ ] **Step 5: Commit**

```bash
git add lessons/11-iterators-collections/solutions
git commit -m "feat(lesson-11): add reference solutions"
```

---

## Task 4: Lesson README

**Files:**
- Overwrite: `lessons/11-iterators-collections/README.md`

- [ ] **Step 1: Overwrite `lessons/11-iterators-collections/README.md`**

The complete file content is below, delimited by an OUTER quadruple-backtick fence (` ```` `). That outer fence is ONLY a delimiter for this plan — do NOT write it into the file. The file must start with `# Lesson 11` on line 1 and contain only PLAIN triple-backtick (` ``` `) code fences.

````markdown
# Lesson 11 — Iterators & collections

An iterator is a lazy stream of values. You build a pipeline — filter,
map — and then a consumer like `collect` or `sum` pulls the values
through and produces a result. Combined with the two workhorse
collections, `Vec` and `HashMap`, iterator chains are the single
most-used pattern in idiomatic Rust. This lesson closes out Phase 2.

## Learning goals

- Build an iterator chain — adaptors (`filter`, `map`) feeding a
  consumer (`collect`, `sum`) — over a slice
- Explain that iterators are *lazy*: adaptors do nothing until a
  consumer drives them
- Distinguish `iter` (`&T`), `iter_mut` (`&mut T`), and `into_iter`
  (`T`, consumes the collection)
- Use a `HashMap<K, V>` with the `entry(...).or_insert(...)` API to
  accumulate counts
- Iterate a `HashMap` to find an extreme (`max_by_key`)
- Recognize why you cannot mutate a collection while iterating it

## Self-study notes

### Iterators are lazy — adaptors vs consumers

An *adaptor* transforms an iterator and returns a new one; a *consumer*
drives the iterator to produce a final value. Adaptors are lazy —
nothing happens until a consumer runs:

```rust
let nums = [1, 2, 3, 4];
let doubled: Vec<i32> = nums.iter().copied().map(|n| n * 2).collect();
// map is the adaptor; collect is the consumer that actually runs it
```

Common adaptors: `filter` (keep matching items), `map` (transform each
item). Common consumers: `collect` (build a collection), `sum`, and
`fold` (thread an accumulator). Without a consumer, an adaptor chain is
just an unused recipe.

### Three ways to iterate — `iter` / `iter_mut` / `into_iter`

How you start a chain decides what you get and what happens to the
collection:

```rust
let mut v = vec![1, 2, 3];
for x in v.iter() {}      // x: &i32      — borrow each item
for x in v.iter_mut() {}  // x: &mut i32  — borrow mutably
for x in v.into_iter() {} // x: i32       — consume v, take ownership
```

`iter` borrows, `iter_mut` borrows mutably, `into_iter` consumes the
collection. The borrow rules from Lessons 8-9 decide which one you can
use in a given spot.

### `Vec<T>` — the growable array

`Vec<T>` is the standard list: an owned, heap-backed buffer that grows
as you push:

```rust
let mut v = Vec::new();
v.push(10);
v.push(20);
println!("{}", v[0]);     // index with []
let slice: &[i32] = &v;   // borrow the whole Vec as a slice
```

Index with `[]`, and borrow it as a slice `&[T]` to pass it around
cheaply without giving up ownership.

### `HashMap<K, V>` — keys, values, and the entry API

`HashMap<K, V>` maps keys to values. The `entry` API is the idiom for
"update, inserting a default if the key is missing":

```rust
use std::collections::HashMap;

let mut counts: HashMap<String, usize> = HashMap::new();
*counts.entry("hi".to_string()).or_insert(0) += 1;
// or_insert(0) returns &mut to the value, creating it first if absent
```

This counting pattern — `*map.entry(key).or_insert(0) += 1` — is worth
memorizing.

### Iterating a map — finding the maximum

A `HashMap` iterates as key/value pairs. To find the pair with the
largest value, use `max_by_key`:

```rust
let top = counts
    .into_iter()
    .max_by_key(|(_word, count)| *count)
    .map(|(word, _count)| word);
```

`into_iter` yields owned `(String, usize)` pairs, `max_by_key` picks the
largest by count, and `map` keeps just the word. Iteration order is
otherwise unspecified — never rely on it.

## Exercises

### Warm-up: `evens_squared`

Implement `evens_squared(nums: &[i32]) -> Vec<i32>` that keeps the even
numbers, squares each, and collects the results:

```rust
pub fn evens_squared(nums: &[i32]) -> Vec<i32> {
    // nums.iter().copied().filter(...).map(...).collect()
    todo!()
}
```

This is the lesson's spine: an `iter`/`filter`/`map`/`collect` chain in
one expression. Use `copied()` so the closures work with plain `i32`.

### Main: `word_frequencies` + `most_frequent`

Implement two functions — `most_frequent` builds on `word_frequencies`:

```rust
// Count each whitespace-separated word.
pub fn word_frequencies(text: &str) -> HashMap<String, usize> { todo!() }

// The word with the highest count, or None if the text is empty.
pub fn most_frequent(text: &str) -> Option<String> { todo!() }
```

`word_frequencies` uses `split_whitespace` and the `entry` API;
`most_frequent` iterates the resulting map with `max_by_key`.

### Compile-fail

`exercises/compile_fails/11-mutate-while-iterating.rs` tries to `push`
to a `Vec` while a `for` loop is iterating it. The loop holds a shared
borrow, so the mutable borrow `push` needs is rejected (E0502). Fix it
by collecting into a new `Vec` instead of mutating during the loop.

### Run

```bash
make verify LESSON=11-iterators-collections
```

This runs your exercise tests and asserts the compile-fail file now
compiles.

## Solutions

See `solutions/src/lib.rs` for the reference implementations. Try the
exercises before peeking.
````

- [ ] **Step 2: Spot-check the README**

```bash
head -1 lessons/11-iterators-collections/README.md
grep -c '^### ' lessons/11-iterators-collections/README.md
grep -c '^```' lessons/11-iterators-collections/README.md
```

Expected:
- First line: `# Lesson 11 — Iterators & collections`
- `grep -c '^### '` returns 9 (five subsections under self-study + four under exercises)
- `grep -c '^```'` returns 16 (8 code blocks × 2 fence lines)

- [ ] **Step 3: Commit**

```bash
git add lessons/11-iterators-collections/README.md
git commit -m "docs(lesson-11): write self-study notes"
```

---

## Task 5: Slide deck

**Files:**
- Overwrite: `lessons/11-iterators-collections/slides/slides.md`

- [ ] **Step 1: Overwrite `lessons/11-iterators-collections/slides/slides.md`**

The complete file content is below, delimited by an OUTER quadruple-backtick fence (` ```` `). That outer fence is ONLY a delimiter for this plan — do NOT write it into the file. The file must start with `# Iterators & collections` on line 1 and contain only PLAIN triple-backtick (` ``` `) code fences.

````markdown
# Iterators & collections

> An iterator is a lazy stream of values. You build a pipeline — filter, map, then collect — and nothing runs until the end pulls the values through. It's the single most-used pattern in idiomatic Rust.

---

## Recap — why iterators

Phase 2 has been about ownership and borrowing.

Iterators are how you **process** owned and borrowed data without writing index-juggling loops. They're lazy, and they compile down to the same code a hand-written loop would — zero-cost.

---

## Three ways to iterate

```rust
let mut v = vec![1, 2, 3];

for x in v.iter() {}      // x: &i32      — borrow each item
for x in v.iter_mut() {}  // x: &mut i32  — borrow mutably
for x in v.into_iter() {} // x: i32       — consume v, take ownership
```

`iter` borrows, `iter_mut` borrows mutably, `into_iter` **consumes** the collection. The borrow rules from Lessons 8-9 decide which you can use.

---

## Adaptors — lazy building blocks

```rust
// copied() turns &i32 into i32 so the closures take plain values
let chain = nums.iter().copied().filter(|&n| n % 2 == 0).map(|n| n * n);
// nothing has run yet — `chain` is just a recipe
```

- `filter` keeps items matching a predicate
- `map` transforms each item

Both are **adaptors**: they return a new iterator and do no work until something consumes them.

---

## Consumers — driving the pipeline

```rust
let total: i32 = nums.iter().sum();
let doubled: Vec<i32> = nums.iter().copied().map(|n| n * 2).collect();
let folded: i32 = nums.iter().copied().fold(0, |acc, n| acc + n);
```

A **consumer** pulls values through and produces a result. `sum` adds them, `collect` builds a collection, `fold` threads an accumulator. The consumer is what makes the lazy chain actually run.

---

## `Vec<T>` — the growable array

```rust
let mut v = Vec::new();
v.push(10);
v.push(20);
println!("{}", v[0]);     // 10
let slice: &[i32] = &v;   // borrow the whole thing as a slice
```

`Vec` owns a heap buffer that grows as you `push`. Index with `[]`, borrow as a slice `&[T]` to pass it cheaply. The standard list type.

---

## `HashMap<K, V>` — keys to values

```rust
use std::collections::HashMap;

let mut counts: HashMap<String, usize> = HashMap::new();
*counts.entry("hi".to_string()).or_insert(0) += 1;
```

`HashMap` maps keys to values. The `entry` API is the idiom for "increment, inserting a default if absent": `entry(key).or_insert(0)` hands you a `&mut` to the value, creating it first if needed.

---

## Iterating a `HashMap`

```rust
let top = counts
    .iter()
    .max_by_key(|(_word, count)| **count)
    .map(|(word, _)| word.clone());
```

A map iterates as `(&K, &V)` pairs. `max_by_key` finds the pair with the largest value; order is otherwise unspecified, so don't rely on it.

---

## Putting it together

Today's exercises:

- **Warm-up** `evens_squared` — `iter`/`copied`/`filter`/`map`/`collect` into a `Vec`
- **Main** a word counter — `word_frequencies` builds a `HashMap` with the entry API, `most_frequent` iterates it with `max_by_key`

The compile-fail shows why you can't `push` to a `Vec` while iterating it.

---

## Wrap — Phase 2 done

- Iterators are **lazy** pipelines: adaptors build, consumers run
- `iter` / `iter_mut` / `into_iter` choose the borrow
- **`Vec<T>`** is the growable array
- **`HashMap<K, V>`** + the entry API accumulate keyed data
- The borrow rules still apply — no mutating while iterating

Next: **Lesson 12 — Traits & generics**, opening Phase 3 (Abstraction).
````

- [ ] **Step 2: Verify `make slides-build` succeeds and includes lesson 11**

```bash
make slides-build
test -f dist/lessons/11-iterators-collections/slides/slides.md
test -f dist/lessons/11-iterators-collections/slides/index.html
grep -c "11-iterators-collections" dist/index.html
```

Expected: `slides.md` and `index.html` copied into dist; `grep -c "11-iterators-collections"` returns at least 1.

- [ ] **Step 3: Spot-check slide separators**

```bash
grep -c '^---$' lessons/11-iterators-collections/slides/slides.md
```

Expected: `9` (between 10 slides).

- [ ] **Step 4: Commit**

```bash
git add lessons/11-iterators-collections/slides/slides.md
git commit -m "feat(lesson-11): write slide deck"
```

---

## Task 6: End-to-end verification + push

- [ ] **Step 1: `make ci` is green**

```bash
make ci
```

Expected: exit 0. Clippy clean, fmt clean, workspace builds, default-members tests pass (now includes the 8 new tests in `iterators-collections-solutions`), compile-fail `--expect broken` passes for lesson 11.

- [ ] **Step 2: `make verify LESSON=11-iterators-collections` fails (the exercise is undone — intentional)**

```bash
make verify LESSON=11-iterators-collections || echo "expected: exercise tests fail with todo!() panic"
```

Expected: non-zero exit. All 8 exercise tests panic with `not yet implemented`.

- [ ] **Step 3: `make slides-build` final state**

```bash
make slides-build
ls dist/lessons/
grep -c "11-iterators-collections" dist/index.html
```

Expected: `dist/lessons/` contains all eleven lessons. `grep -c "11-iterators-collections"` ≥ 1.

- [ ] **Step 4: Push**

```bash
git push
```

Expected: push succeeds. CI runs and is green; Deploy rebuilds the static site so lesson 11 appears live.

- [ ] **Step 5: Smoke-test the deployed site**

After the push, wait for the Deploy workflow to finish (`gh run watch <id>`). Then:

```bash
curl -sS -o /dev/null -w "%{http_code}\n" https://rust.ristkari.dev/
curl -sS -o /dev/null -w "%{http_code}\n" https://rust.ristkari.dev/lessons/11-iterators-collections/slides/
```

Expected: both return `200`.

---

## Done criteria

- `lessons/11-iterators-collections/` exists with all four parts
- Both `exercises/src/lib.rs` and `solutions/src/lib.rs` declare the same three function signatures (`evens_squared`, `word_frequencies`, `most_frequent`)
- `cargo test --package iterators-collections-solutions` → 8 passing tests
- `cargo test --manifest-path lessons/11-iterators-collections/exercises/Cargo.toml` → 8 panicking tests (intentional)
- `cargo run --package compile-fails -- --expect broken lessons/11-iterators-collections` → ok
- `cargo run --package compile-fails -- --expect compiles lessons/11-iterators-collections` → fails (intentional)
- `make ci` → green
- `make slides-build` → produces `dist/lessons/11-iterators-collections/slides/index.html`
- `dist/index.html` lists lesson 11 as a clickable link
- All changes committed and pushed
- Deployed site returns HTTP 200 for `/` and `/lessons/11-iterators-collections/slides/`
