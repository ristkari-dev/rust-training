# Pattern matching & enums

> Rust gives you the ability to define your own sum types — and the compiler will not let you forget a case.

---

## Recap

Lesson 04 introduced compound types we **own**.

Today: add **sum types** — values that are *one of several* shapes — and the `match` expression that handles them safely.

---

## Enums

```rust
enum Direction {
    North,
    South,
    East,
    West,
}

let d = Direction::North;
```

- A type whose value is exactly one of a fixed set of variants
- Construct with `Direction::North`, `Direction::South`, ...
- Derive `Debug` to print, `PartialEq` to compare with `==`:

```rust
#[derive(Debug, PartialEq, Eq)]
enum Direction { /* ... */ }
```

---

## `match` on enums

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

- **Exhaustive**: every variant must have an arm
- Each arm is an expression
- `match` itself is an expression — can be assigned, returned, used as a tail

---

## More patterns

```rust
fn classify(n: i32) -> &'static str {
    match n {
        0 => "zero",          // literal
        1..=9 => "small",     // inclusive range
        _ => "other",         // wildcard catches the rest
    }
}
```

- Literals: `0`, `"yes"`, etc.
- Ranges: `1..=9` (inclusive)
- Bindings: `n => ...` (catches and binds the value)
- Wildcard: `_ => ...` (catches the rest without binding)

---

## `Option<T>`

The most important enum in the standard library:

```rust
enum Option<T> {
    Some(T),
    None,
}
```

Rust's answer to null. Any value that might be missing is `Option<T>` instead of "null T".

```rust
let found: Option<i32> = Some(42);
let missing: Option<i32> = None;
```

---

## Matching on `Option`

```rust
fn double_or_zero(n: Option<i32>) -> i32 {
    match n {
        Some(x) => x * 2,
        None    => 0,
    }
}
```

- `Some(x)` binds the inner value to `x`
- `None` matches the absent case
- The compiler enforces both arms — you can't forget the missing case

---

## `if let` — one-variant sugar

When you only care about one variant:

```rust
let opt: Option<i32> = Some(5);

if let Some(x) = opt {
    println!("got {x}");
}
```

Equivalent to:

```rust
match opt {
    Some(x) => println!("got {x}"),
    None    => (),
}
```

Trade-off: you lose exhaustiveness. Use when there's genuinely just one case to handle.

---

## Putting it together

Define your own enum:

```rust
#[derive(Debug, PartialEq, Eq)]
pub enum Light {
    Red,
    Yellow,
    Green,
}
```

Then match all variants:

```rust
pub fn next(light: Light) -> Light {
    match light {
        Light::Red    => Light::Green,
        Light::Green  => Light::Yellow,
        Light::Yellow => Light::Red,
    }
}
```

If you forget a variant, the compiler refuses to compile — try it in the compile-fail exercise.

---

## Wrap

- **Enums** define your own sum types
- **`match`** is exhaustive and is an expression
- Patterns: literals, ranges, bindings, wildcards
- **`Option<T>`** is just an enum — Rust's nullable
- **`if let`** is sugar for the single-variant case

Next: Lesson 06 — structs & methods.
