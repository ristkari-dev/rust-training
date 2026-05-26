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

```rust
let s = String::from("hello");
let t = s;            // ownership moved: s -> t
println!("{t}");      // OK, t owns the value
// println!("{s}");   // ERROR: s no longer owns anything
```

Passing a value to a function does the same thing — the parameter is
the new owner:

```rust
fn take(s: String) { /* ... */ }

let s = String::from("hello");
take(s);              // s moved into the function
// take(s);           // ERROR: use after move
```

This is "move semantics." It applies to any type that owns
heap-allocated data, including `String`.

### `Copy` types and why they're different

Primitive types like `i32`, `f64`, `bool`, and `char` implement the
`Copy` marker trait. Tuples and arrays of `Copy` types are themselves
`Copy`. For these types, assignment is bitwise duplication — both
bindings are valid:

```rust
let n: i32 = 5;
let m = n;            // i32 is Copy — n is bitwise duplicated
println!("{n} {m}");  // both still valid
```

The reason: copying a few stack bytes is cheap and has no aliasing
concerns. The reason `String` is *not* `Copy`: it owns a heap
allocation, and bit-copying it would give you two owners of the same
buffer — a recipe for double-free bugs. Rust's rule "only one owner"
is what makes the language safe without a garbage collector.

### `.clone()` — explicit duplication

When you genuinely need to keep using a value *and* pass it
somewhere, call `.clone()`:

```rust
let s = String::from("hello");
take(s.clone());      // hand the function a copy
println!("{s}");      // original still owns
```

Cloning a `String` allocates new heap memory and copies the bytes.
That's why it's explicit — the compiler refuses to do it silently, so
you're always aware when it happens.

For `Copy` types, `clone()` exists too but is redundant — assignment
already duplicates.

### Returning ownership from functions

Functions can take ownership and give it back. This is the pattern
for "transform a value":

```rust
fn append_excl(mut s: String) -> String {
    s.push('!');
    s
}

let s = String::from("hi");
let s = append_excl(s);   // s moved in, mutated, returned, re-bound
```

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

```rust
let mut result = b;
result.push(' ');
result.push_str(&a);
result
```

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

```bash
make verify LESSON=07-ownership
```

This runs your exercise tests and asserts the compile-fail file now
compiles.

## Solutions

See `solutions/src/lib.rs` for the reference implementations. Try the
exercises before peeking.
