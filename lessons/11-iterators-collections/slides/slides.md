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
