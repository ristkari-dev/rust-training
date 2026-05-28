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
