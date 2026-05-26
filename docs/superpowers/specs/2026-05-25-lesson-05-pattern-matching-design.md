# Lesson 05 — Pattern matching & enums — design

The fifth lesson of the Rust training course. Introduces sum types
(`enum`), the `match` expression (with exhaustiveness checking),
patterns, the std-lib `Option<T>` type, and `if let` sugar.

## Audience and prerequisites

- Has completed Lessons 01-04
- Comfortable with control flow, expressions vs statements, slices,
  `String`/`&str`
- Has the toolchain installed and `make verify` working

## Learning goals

By the end of this lesson, the student can:

1. Define a custom `enum` with multiple variants and derive `Debug` /
   `PartialEq` for it
2. Use `match` exhaustively against an enum and read rustc's
   exhaustiveness error when a variant is missing
3. Use pattern features — wildcards `_`, binding patterns, literal
   patterns, range patterns — in match arms
4. Construct and destructure `Option<T>` to represent values that may
   be absent
5. Use `if let` as sugar for a single-variant match

## Scope

In scope: `enum` declarations with unit-only variants (e.g. `Light::Red`);
`match` as exhaustive expression; pattern features (wildcards, bindings,
literals, simple ranges); `Option<T>` construction (`Some(v)`, `None`)
and matching; `if let`; `#[derive(Debug, PartialEq, Eq)]` for enums so
tests can compare values.

Out of scope (deferred): enums with associated data (`Option<T>` itself
has data but we use it as a closed example, not a template for student
construction); guard clauses `if cond` on match arms; or-patterns
`A | B`; pattern matching on structs and tuples beyond the trivial
(Lesson 06 — Structs & methods, Lesson 14 — Error handling for
`Result<T, E>`); `match` ergonomics around references (Lessons 07-09 —
Ownership deep dive); `unwrap()` / `expect()` (mentioned in slides as
"escape hatches for when you're sure" but not exercised — Lesson 14 is
the right home for error propagation).

## Slide arc (10 slides)

1. **Title — Pattern matching & enums.** Hook: *"Rust gives you the
   ability to define your own sum types — and the compiler will not let
   you forget a case."*
2. **Recap.** Lesson 04 introduced compound types we *own*. Today we
   add **sum types** — types where a value is *one of several* shapes
   — and the `match` expression that handles them safely.
3. **Enums.** `enum Direction { North, South, East, West }` — a type
   whose value is exactly one of a fixed set of variants. Constructed
   via `Direction::North`. Compared via `==` (with `#[derive(PartialEq)]`).
4. **`match` on enums.** Exhaustive — every variant must have an arm.
   Each arm is an expression. `match` itself is an expression, so its
   value can be used directly (assigned to `let`, returned from a
   function, etc.).
5. **More patterns.** Wildcards `_`, bindings `n => ...`, literal
   patterns `0 => ...`, range patterns `1..=5 => ...`. Patterns chain
   naturally in a match.
6. **`Option<T>`.** The most important enum in the standard library:
   `enum Option<T> { Some(T), None }`. Rust's answer to null. Any
   "might be missing" value uses Option.
7. **Matching on Option.** Two arms — one for `Some(value)` (binds the
   inner value), one for `None`. The classic shape:
   `match opt { Some(x) => ..., None => ... }`.
8. **`if let` — sugar for one variant.** When you only care about one
   case, `if let Some(x) = opt { ... }` is shorter than the equivalent
   two-arm match. Trade-off: you lose exhaustiveness checking, so use
   it when there's genuinely just one case to handle.
9. **Putting it together.** Walk through the lesson's main exercise:
   define `enum Light { Red, Yellow, Green }` with `#[derive(Debug,
   PartialEq, Eq)]`, then write `fn next(light: Light) -> Light` as a
   `match` over all three variants. The compile-fail exercise drives
   the exhaustiveness point home.
10. **Wrap.** Five takeaways: enums are sum types you define yourself;
    `match` is exhaustive and is an expression; patterns let you
    destructure as you match; `Option<T>` is just an enum; `if let` is
    sugar for the single-case shape. Next: Lesson 06 — structs &
    methods.

## Exercise spec

`lessons/05-pattern-matching/` follows the standard four-part lesson
shape:

```
05-pattern-matching/
├── README.md
├── slides/
│   ├── index.html
│   └── slides.md
├── exercises/
│   ├── Cargo.toml
│   ├── src/lib.rs
│   ├── tests/exercise.rs
│   └── compile_fails/05-non-exhaustive-match.rs
└── solutions/
    ├── Cargo.toml
    ├── src/lib.rs
    └── tests/exercise.rs
```

Cargo package names: `pattern-matching-exercises` and
`pattern-matching-solutions` (the lesson's "bare" name is
`pattern-matching`).

### Shared enum definition

Both the exercises crate and the solutions crate ship the `Light` enum
already defined and derived:

```rust
#[derive(Debug, PartialEq, Eq)]
pub enum Light {
    Red,
    Yellow,
    Green,
}
```

Tests can `assert_eq!` `Light` values directly thanks to `PartialEq`
and `Debug`. Students don't have to remember the derive incantation in
their first enum-author lesson.

### Exercise stub (`exercises/src/lib.rs`)

```rust
//! Lesson 05 — exercises.

#[derive(Debug, PartialEq, Eq)]
pub enum Light {
    Red,
    Yellow,
    Green,
}

#[must_use]
pub fn safe_divide(_a: i32, _b: i32) -> Option<i32> {
    todo!("return None when b == 0, otherwise Some(a / b)")
}

#[must_use]
pub fn next(_light: Light) -> Light {
    todo!("return Red->Green, Green->Yellow, Yellow->Red")
}
```

### Warm-up: `safe_divide`

Signature:

```rust
pub fn safe_divide(a: i32, b: i32) -> Option<i32>
```

Reference solution (uses `match` to model the lesson's tool, though
students may equivalently use `if`):

```rust
#[must_use]
pub fn safe_divide(a: i32, b: i32) -> Option<i32> {
    match b {
        0 => None,
        _ => Some(a / b),
    }
}
```

Four tests:

```rust
#[test] fn warmup_typical()       { assert_eq!(safe_divide(10, 2),  Some(5));  }
#[test] fn warmup_by_zero()       { assert_eq!(safe_divide(10, 0),  None);     }
#[test] fn warmup_zero_dividend() { assert_eq!(safe_divide(0, 5),   Some(0));  }
#[test] fn warmup_negative()      { assert_eq!(safe_divide(-10, 2), Some(-5)); }
```

### Main: `next` (traffic-light state machine)

Signature:

```rust
pub fn next(light: Light) -> Light
```

Reference solution:

```rust
#[must_use]
pub fn next(light: Light) -> Light {
    match light {
        Light::Red    => Light::Green,
        Light::Green  => Light::Yellow,
        Light::Yellow => Light::Red,
    }
}
```

Four tests:

```rust
#[test] fn main_red_to_green()    { assert_eq!(next(Light::Red),    Light::Green);  }
#[test] fn main_green_to_yellow() { assert_eq!(next(Light::Green),  Light::Yellow); }
#[test] fn main_yellow_to_red()   { assert_eq!(next(Light::Yellow), Light::Red);    }
#[test] fn main_cycle_closes()    { assert_eq!(next(next(next(Light::Red))), Light::Red); }
```

**Eight tests total** (four warm-up + four main).

### Compile-fail: `05-non-exhaustive-match.rs`

Path: `exercises/compile_fails/05-non-exhaustive-match.rs`. Ships
broken; the student adds the missing match arm until the file
compiles.

```rust
// Compile-fail exercise: this file MUST NOT compile until you fix it.
//
// Rust's `match` expression is **exhaustive**: every possible variant
// of the matched type must have an arm. If you forget one, the compiler
// refuses to compile. This is one of Rust's signature safety features —
// you can't accidentally not-handle a case.
//
// The function below matches on a `Direction` enum but forgets one of
// the four variants. rustc will tell you exactly which one is missing.
//
// Hint: read the rustc error. It will say "non-exhaustive patterns:
// `<Variant>` not covered". Add an arm that maps the missing variant to
// its opposite.

#[derive(Debug)]
enum Direction {
    North,
    South,
    East,
    West,
}

fn opposite(d: Direction) -> Direction {
    match d {
        Direction::North => Direction::South,
        Direction::South => Direction::North,
        Direction::East => Direction::West,
        // West is missing!
    }
}

fn main() {
    let d = opposite(Direction::West);
    println!("{d:?}");
}
```

`Direction` is deliberately a *different* enum from `Light` so the
compile-fail is self-contained — it doesn't cross-reference the
exercise crate.

`make test` (and CI) runs the compile-fails tool with `--expect broken`
and sees `ok` for the shipped state. `make verify LESSON=05-pattern-matching`
runs `--expect compiles` and refuses to pass until the student adds the
missing arm.

This compile-fail directly reinforces slide 4: rustc's diagnostic
literally names the missing variant ("non-exhaustive patterns: `West`
not covered"), putting the slide content into the compiler's voice.

## README structure

`lessons/05-pattern-matching/README.md` follows the established shape:

- **Title + one-paragraph hook**
- **Learning goals** — the five bullets above
- **Self-study notes** with these subsections:
  - Defining your own enums
  - The `match` expression
  - Patterns: wildcards, bindings, ranges, literals
  - `Option<T>` — the standard-library nullable
  - `if let` — sugar for one variant
- **Exercises** — four subsections: Warm-up (`safe_divide`),
  Main (`next` traffic light), Compile-fail, Run
- **Solutions** — pointer to `solutions/src/lib.rs`

Each `###` subsection runs ~4-6 sentences plus a small code block. The
"`match` expression" section is the heaviest since exhaustiveness is
the lesson's spine; the "`if let`" section is the lightest.

## Lint expectations

Lesson 05's reference solution code should be clippy-clean without
`#[allow]` attributes:

- `safe_divide` uses `match b { 0 => ..., _ => ... }` — straightforward.
- `next` matches all three variants explicitly — exhaustive, idiomatic.
- The enum derives `Debug, PartialEq, Eq` — standard pattern.

One lint to watch: `clippy::match_same_arms` could fire if the
implementer writes redundant arms. The reference solution's three arms
all produce different values, so the lint shouldn't fire. If it does,
fix the code rather than allow-listing.

The tests file does not need `#![allow(clippy::float_cmp)]` — there
are no f64 comparisons.

## Done criteria

- `lessons/05-pattern-matching/` exists with the four-part structure
- Cargo manifests use the correct package names
  (`pattern-matching-exercises`, `pattern-matching-solutions`)
- Both the exercise and solution crates define the `Light` enum
  identically (same variants, same derives)
- `cargo test --package pattern-matching-solutions` → 8 tests pass
- `cargo test --manifest-path lessons/05-pattern-matching/exercises/Cargo.toml`
  → both stubs panic with `todo!()`, the intended undone state
- `cargo run --package compile-fails -- --expect broken lessons/05-pattern-matching`
  → ok
- `cargo run --package compile-fails -- --expect compiles lessons/05-pattern-matching`
  → fails (file ships broken, that's the point)
- `make ci` is green
- `make slides-build` produces `dist/lessons/05-pattern-matching/slides/index.html`
- `dist/index.html` lists lesson 05 as a clickable lesson
- One push to `origin/main` triggers a green CI run and a green Deploy
  run; `https://rust.ristkari.dev/lessons/05-pattern-matching/slides/`
  returns 200

## Open questions

None.
