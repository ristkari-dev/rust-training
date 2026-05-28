# Lesson 11 — Iterators & collections — design

The fifth and final lesson of Phase 2 (Ownership deep dive). Introduces
iterators as the spine of Rust data processing — *data in a collection →
transform with a chain → collect a result* — using `Vec` as the input
and `HashMap` as the output. Closes out Phase 2.

## Audience and prerequisites

- Has completed Lessons 01-10
- Comfortable with ownership, borrowing, lifetimes, `match`, structs,
  and the smart pointers from L10
- Has the toolchain installed and `make verify` working

## Learning goals

By the end of this lesson, the student can:

1. Build an iterator chain — adaptors (`filter`, `map`) feeding a
   consumer (`collect`, `sum`) — over a slice
2. Explain that iterators are *lazy*: adaptors do nothing until a
   consumer drives them
3. Distinguish the three ways to iterate — `iter` (`&T`), `iter_mut`
   (`&mut T`), `into_iter` (`T`, consumes the collection)
4. Use a `HashMap<K, V>` with the `entry(...).or_insert(...)` API to
   accumulate counts
5. Iterate a `HashMap` to find an extreme (`max_by_key`)
6. Recognize why you cannot mutate a collection while iterating it — the
   borrow rules from L8-L9 still apply

## Scope

In scope: iterator adaptors `filter` and `map`; consumers `collect`,
`sum`, and a mention of `fold`; the `iter`/`iter_mut`/`into_iter`
distinction; `Vec<T>` as the growable array (creation, `push`,
indexing, slices); `HashMap<K, V>` insert/lookup and the `entry` API;
iterating a map with `max_by_key`. The exercises drill a filter-map-collect
chain (warm-up) and a HashMap word-counter with a map-iterating helper
(main).

Out of scope (deferred or skipped): custom iterators (implementing the
`Iterator` trait — Phase 3, after traits); `BTreeMap`/`HashSet`/`VecDeque`
and the rest of `std::collections`; `flat_map`, `zip`, `enumerate`,
`take`/`skip`, `chain`, `windows`, `chunks` (mentioned at most in
passing — not exercised); `collect` into types other than `Vec`/`HashMap`;
`itertools`; parallel iterators (`rayon`). Iterators are *introduced* as
a tool here; implementing your own `Iterator` waits for traits in
Phase 3.

## Slide arc (10 slides)

1. **Title — Iterators & collections.** Hook: *"An iterator is a lazy
   stream of values. You build a pipeline — filter, map, then collect —
   and nothing runs until the end pulls the values through. It's the
   single most-used pattern in idiomatic Rust."*
2. **Recap / why iterators.** Phase 2 has been ownership and borrowing.
   Iterators are how you *process* owned and borrowed data without
   writing index-juggling loops. They're lazy and compile down to the
   same code a hand-written loop would — zero-cost.
3. **Three ways to iterate.**
   ```rust
   let v = vec![1, 2, 3];
   for x in v.iter() {}      // x: &i32   — borrow each item
   for x in v.iter_mut() {}  // x: &mut i32 — borrow mutably (needs `mut v`)
   for x in v.into_iter() {} // x: i32    — consume v, take ownership
   ```
   `iter` borrows, `iter_mut` borrows mutably, `into_iter` *consumes*
   the collection. The borrow rules from L8-L9 decide which you can use.
4. **Adaptors — lazy building blocks.**
   ```rust
   // copied() turns &i32 into i32 so the closures take plain values
   let chain = nums.iter().copied().filter(|&n| n % 2 == 0).map(|n| n * n);
   // nothing has run yet — `chain` is just a recipe
   ```
   `filter` keeps items matching a predicate; `map` transforms each
   item. Both are *adaptors*: they return a new iterator and do no work
   until something consumes them.
5. **Consumers — driving the pipeline.**
   ```rust
   let total: i32 = nums.iter().sum();
   let doubled: Vec<i32> = nums.iter().map(|n| n * 2).collect();
   let sum = nums.iter().fold(0, |acc, n| acc + n);
   ```
   A *consumer* pulls values through and produces a result. `sum` adds
   them, `collect` builds a collection, `fold` threads an accumulator.
   The consumer is what makes the lazy chain actually run.
6. **`Vec<T>` — the growable array.**
   ```rust
   let mut v = Vec::new();
   v.push(10);
   v.push(20);
   println!("{}", v[0]);     // 10
   let slice: &[i32] = &v;   // borrow the whole thing as a slice
   ```
   `Vec` owns a heap buffer that grows as you `push`. Index with `[]`,
   borrow as a slice `&[T]` to pass it cheaply. The standard list type.
7. **`HashMap<K, V>` — keys to values.**
   ```rust
   use std::collections::HashMap;

   let mut counts: HashMap<String, usize> = HashMap::new();
   *counts.entry("hi".to_string()).or_insert(0) += 1;
   ```
   `HashMap` maps keys to values. The `entry` API is the idiom for
   "increment, inserting a default if absent": `entry(key).or_insert(0)`
   hands you a `&mut` to the value, creating it first if needed.
8. **Iterating a `HashMap`.**
   ```rust
   let top = counts
       .iter()
       .max_by_key(|(_word, count)| **count)
       .map(|(word, _)| word.clone());
   ```
   A map iterates as `(&K, &V)` pairs. `max_by_key` finds the pair with
   the largest value; order is otherwise unspecified, so don't rely on
   it.
9. **Putting it together.** Walk through the exercises: `evens_squared`
   (warm-up — `iter`/`copied`/`filter`/`map`/`collect` into a `Vec`)
   and the word counter (main — `word_frequencies` builds a `HashMap`
   with the entry API, `most_frequent` iterates it with `max_by_key`).
   The compile-fail shows why you can't `push` while iterating.
10. **Wrap — Phase 2 done.** Six takeaways: iterators are lazy
    pipelines; adaptors (`filter`/`map`) build, consumers
    (`collect`/`sum`/`fold`) run; `iter`/`iter_mut`/`into_iter` choose
    the borrow; `Vec` is the growable array; `HashMap` + the entry API
    accumulate keyed data; the borrow rules still apply (no mutate
    while iterating). Next: **Lesson 12 — Traits & generics**, opening
    Phase 3 (Abstraction).

## Exercise spec

`lessons/11-iterators-collections/` follows the standard four-part
lesson shape:

```
11-iterators-collections/
├── README.md
├── slides/
│   ├── index.html
│   └── slides.md
├── exercises/
│   ├── Cargo.toml
│   ├── src/lib.rs
│   ├── tests/exercise.rs
│   └── compile_fails/11-mutate-while-iterating.rs
└── solutions/
    ├── Cargo.toml
    ├── src/lib.rs
    └── tests/exercise.rs
```

Cargo package names: `iterators-collections-exercises` and
`iterators-collections-solutions` (the lesson's "bare" name is
`iterators-collections`; the import idents are
`iterators_collections_exercises` / `iterators_collections_solutions`).

### Exercise stub (`exercises/src/lib.rs`)

Three function stubs. Students implement all three bodies. The
`HashMap` import is used by the `word_frequencies` signature, so it does
not trip an unused-import warning even while the body is `todo!()`.

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

### Warm-up: `evens_squared`

Reference solution:

```rust
#[must_use]
pub fn evens_squared(nums: &[i32]) -> Vec<i32> {
    nums.iter()
        .copied()
        .filter(|&n| n % 2 == 0)
        .map(|n| n * n)
        .collect()
}
```

Pedagogical packing: the canonical adaptor chain. `iter()` yields
`&i32`; `copied()` turns that into `i32`; `filter` still hands its
closure a reference, so `|&n|` destructures it back to `n: i32` (no
double-deref `**n`) and keeps the evens; `map` then works with plain
`i32` to square them; `collect()` builds the `Vec` (the return type
tells `collect` what to build). This is the lesson's "spine" in one
expression.

Four tests:

```rust
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
```

### Main: `word_frequencies` + `most_frequent`

Reference solutions:

```rust
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

Pedagogical packing: `word_frequencies` shows the entry-API counting
idiom (`entry(...).or_insert(0)` returns a `&mut usize` you increment).
`most_frequent` builds on it: `into_iter()` consumes the map yielding
`(String, usize)` pairs, `max_by_key` finds the pair with the largest
count, and `map` extracts just the word. `into_iter` (not `iter`) means
the `String` is moved out — no clone needed.

Four tests. The `most_frequent` cases use a **unique** maximum because
`HashMap` iteration order is unspecified and `max_by_key` breaks ties by
position — an ambiguous test would be flaky.

```rust
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

**Eight tests total** (four warm-up + four main).

### Compile-fail: `11-mutate-while-iterating.rs`

Path: `exercises/compile_fails/11-mutate-while-iterating.rs`. Ships
broken; the student rewrites the loop so it no longer mutates `v` while
iterating it.

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

Pass condition: the student replaces the mutate-while-iterating loop
with a non-aliasing form (e.g. collecting into a new `Vec`) so the file
compiles. rustc names the conflict (E0502) directly.

This is the Phase 2 capstone: iterators feel new, but the *same* borrow
rules from the references and lifetimes lessons govern them. A shared
borrow held by the loop blocks the mutable borrow `push` needs.

## README structure

`lessons/11-iterators-collections/README.md` follows the established
shape:

- **Title + one-paragraph hook**
- **Learning goals** — the six bullets above
- **Self-study notes** with five subsections:
  - Iterators are lazy — adaptors vs consumers
  - Three ways to iterate — `iter` / `iter_mut` / `into_iter`
  - `Vec<T>` — the growable array
  - `HashMap<K, V>` — keys, values, and the entry API
  - Iterating a map — finding the maximum
- **Exercises** — four subsections: Warm-up (`evens_squared`), Main
  (`word_frequencies` + `most_frequent`), Compile-fail, Run
- **Solutions** — pointer to `solutions/src/lib.rs`

Each `###` subsection runs ~4-6 sentences plus a small code block. The
"adaptors vs consumers" and "entry API" sections are the heaviest —
they cover the two ideas the exercises drill.

## Lint expectations

Lesson 11's reference solution code should be clippy-clean (with
`clippy::all` + `clippy::pedantic` denied) without `#[allow]`
attributes:

- `evens_squared` uses `copied()` and destructures the filter closure
  with `|&n|`, so both closures work with plain `i32` — this avoids the
  `**n` double-deref and the reference-arithmetic that could trip
  `clippy::op_ref`. `n % 2 == 0` and `n * n` are plain integer ops.
- `word_frequencies` uses `entry(...).or_insert(0)` — the idiomatic
  counting pattern; `clippy::map_entry` targets the `contains_key`+`insert`
  anti-pattern, not this, so it will not fire.
- `most_frequent` consumes the map with `into_iter()` so the winning
  `String` is moved out (no `.clone()`), then `max_by_key` + `map`. The
  unused tuple members are named `_word` / `_count` for readability.

If clippy fires on anything unexpected, fix the code rather than adding
allows.

## Done criteria

- `lessons/11-iterators-collections/` exists with the four-part structure
- Cargo manifests use the correct package names
  (`iterators-collections-exercises`, `iterators-collections-solutions`)
- `exercises/src/lib.rs` and `solutions/src/lib.rs` declare the same
  three function signatures
- `cargo test --package iterators-collections-solutions` → 8 tests pass
- `cargo test --manifest-path lessons/11-iterators-collections/exercises/Cargo.toml`
  → all three stubs panic with `todo!()`, the intended undone state
- `cargo run --package compile-fails -- --expect broken lessons/11-iterators-collections`
  → ok
- `cargo run --package compile-fails -- --expect compiles lessons/11-iterators-collections`
  → fails (file ships broken, that's the point)
- `make ci` is green
- `make slides-build` produces `dist/lessons/11-iterators-collections/slides/index.html`
- `dist/index.html` lists lesson 11 as a clickable lesson
- One push to `origin/main` triggers a green CI run and a green Deploy
  run; `https://rust.ristkari.dev/lessons/11-iterators-collections/slides/`
  returns 200

## Open questions

None.
