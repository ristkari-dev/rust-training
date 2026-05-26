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
