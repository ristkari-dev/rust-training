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

```rust
enum Direction {
    North,
    South,
    East,
    West,
}

let heading = Direction::North;
```

To print an enum with `{:?}`, derive `Debug`. To compare two enums with
`==`, derive `PartialEq` (and usually `Eq` alongside):

```rust
#[derive(Debug, PartialEq, Eq)]
enum Direction {
    North,
    South,
    East,
    West,
}
```

We'll learn what derives *are* in Lesson 12; for now treat them as
"give me the obvious behavior for this type."

### The `match` expression

`match` lets you handle every variant of an enum (or value of any
other type) and is **exhaustive** — if you miss a variant, the
compiler refuses to compile.

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

Each arm is `pattern => expression`. Like `if`, `match` itself is an
expression — it produces the value of whichever arm matched.

### Patterns: wildcards, bindings, ranges, literals

Match arms don't have to be enum variants. Patterns include:

```rust
fn classify(n: i32) -> &'static str {
    match n {
        0 => "zero",          // literal
        1..=9 => "small",     // inclusive range
        _ => "other",         // wildcard — catches the rest
    }
}
```

You can also **bind** the matched value to a name:

```rust
let label = match x {
    n => format!("got {n}"),  // binds x to n
};
```

A bare identifier (here `n`) is a binding pattern — it always matches
and binds.

### `Option<T>` — the standard-library nullable

Rust's standard library defines:

```rust
enum Option<T> {
    Some(T),
    None,
}
```

`Option` is how Rust represents "a value of type T, or none." There is
no null pointer in Rust — instead, types that might be missing are
wrapped in `Option`:

```rust
let found: Option<i32> = Some(42);
let missing: Option<i32> = None;
```

You unpack `Option` with `match`:

```rust
fn double_or_zero(n: Option<i32>) -> i32 {
    match n {
        Some(x) => x * 2,
        None    => 0,
    }
}
```

The `Some(x)` pattern binds the inner value to `x` for use in that
arm. The compiler enforces that both arms are present.

### `if let` — sugar for one variant

When you only care about one variant, `if let` is shorter than
`match`:

```rust
let opt: Option<i32> = Some(5);

if let Some(x) = opt {
    println!("got {x}");
}
```

This is equivalent to:

```rust
match opt {
    Some(x) => println!("got {x}"),
    None    => (),
}
```

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

```rust
#[derive(Debug, PartialEq, Eq)]
pub enum Light {
    Red,
    Yellow,
    Green,
}
```

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

```bash
make verify LESSON=05-pattern-matching
```

This runs your exercise tests and asserts the compile-fail file now
compiles.

## Solutions

See `solutions/src/lib.rs` for the reference implementations. Try the
exercises before peeking.
