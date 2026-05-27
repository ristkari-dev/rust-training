# Lifetimes

> Every reference has a lifetime. Most of the time, Rust figures it out for you. Today we learn the syntax for when it can't — and why.

---

## Recap

Lesson 08 introduced `&T` and `&mut T`. We mentioned that every reference has a **lifetime** — and that elision usually handles them silently.

Today: we open the box.

---

## Every reference has a lifetime

Conceptually, a lifetime is the **scope during which a reference is valid**.

The compiler tracks it for every reference.

Most of the time it's implicit and you never see it. When you do — like `&'static str` for string literals — that's lifetime syntax surfacing.

---

## Elision handles the common cases

```rust
fn first_char(s: &str) -> &str { /* ... */ }
fn area(rect: &Rectangle) -> u32 { /* ... */ }
```

For one-input-reference functions and methods (`&self`), Rust deduces the relationship between input and output references.

You've been writing code like this since Lesson 04 without thinking about it.

---

## When elision fails

Two reference parameters, one returned reference — the compiler can't guess which one the output borrows from:

```rust
fn longest(a: &str, b: &str) -> &str {
    if a.len() >= b.len() { a } else { b }
}
```

rustc says: *"this function's return type contains a borrowed value, but the signature does not say whether it is borrowed from `a` or `b`."*

You have to spell out the relationship.

---

## `<'a>` syntax

```rust
fn longest<'a>(a: &'a str, b: &'a str) -> &'a str {
    if a.len() >= b.len() { a } else { b }
}
```

Read this as: "for some lifetime `'a`, both inputs and the output all share that lifetime."

The compiler picks `'a` at each call site to be the **shortest** of the two inputs' lifetimes — that's the lifetime of the returned reference.

---

## The `longest` function

Without `'a`, the signature is ambiguous and won't compile.

With `'a`, the compiler accepts the body and call sites.

At each call:

- The returned reference is valid for as long as **both** input references are valid.
- The shorter of the two lifetimes wins.

This is the warm-up exercise.

---

## Structs holding references

A struct field that's a reference forces the struct to declare a lifetime parameter:

```rust
struct Excerpt<'a> {
    text: &'a str,
}
```

This says: "the struct cannot outlive the `&str` it holds."

If the original string is dropped, any `Excerpt` referencing it is invalidated by the compiler.

The compile-fail exercise drills the canonical mistake — forgetting the `<'a>`.

---

## `'static` — the special lifetime

`&'static str` is a reference that lives for the **entire program**.

String literals are `&'static str` (slices into the compiled binary):

```rust
let greeting: &'static str = "hello";
```

Most other references have shorter, scope-limited lifetimes. You'll occasionally need `'static` in trait bounds and APIs, but it's not common in everyday function signatures.

---

## Wrap — Phase 2 progress

- Every reference has a lifetime
- Elision handles the common cases (single-input, `&self`)
- Explicit `<'a>` is needed for multi-input functions returning a reference, and for any struct with a reference field
- Lifetime parameters say "all these references share a relationship"
- `'static` lives for the whole program

Next: **Lesson 10 — Smart pointers** (`Box`, `Rc`, `Arc`, `RefCell`).
