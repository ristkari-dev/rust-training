# Lesson 02 — Variables, types, mutability — design

The second lesson of the Rust training course. Builds on Lesson 01 (which
already exposed students to the immutable-binding compile error) by
explaining `let`, `mut`, type inference, type annotations, shadowing, and
`const`.

## Audience and prerequisites

- Has completed Lesson 01 (`fn main`, `println!`, a library function, one
  compile error)
- Has the toolchain installed and `make verify` working

## Learning goals

By the end of this lesson, the student can:

1. Declare bindings with `let` and explain why they're immutable by default
2. Opt into mutation with `let mut` and explain when it's appropriate
3. Read Rust's inferred types from a binding and add explicit annotations
   when needed
4. Re-bind a name using shadowing, and explain why it's different from
   mutation
5. Declare compile-time constants with `const`

## Scope

In scope: `let`, `let mut`, shadowing, type inference, type annotations,
`const`. One brief aside on method syntax for primitives (so the exercise
can use `.powi()` and `f64::from()`); methods proper are taught in
Lesson 06.

Out of scope (deferred to later lessons): a numeric-types tour (Lesson 04
"Compound types" is the natural home), conversions deeper than the single
`as i32` cast and `f64::from(u32)` the exercise needs, `static`, lifetime
bounds on `const`.

## Slide arc (10 slides)

1. **Title — Variables, types, mutability.** Hook: *"In Rust, the default
   is no mutation. The type system makes that practical."*
2. **Recap.** Lesson 01 showed a compile error on `x = 2` for an immutable
   binding. We're going to make sense of that error and learn what to do
   about it.
3. **`let` — the default.** `let x = 5;` binds once. Reassigning fails to
   compile; show the same error students already saw in Lesson 01.
4. **`let mut` — opting into mutation.** `let mut x = 5; x = 6;` is fine.
   Frame it as a deliberate, local opt-in: the keyword tells future
   readers "yes, this changes."
5. **Type inference.** `let x = 5;` infers `i32`. `let y = 5.0;` infers
   `f64`. `let ok = true;` infers `bool`. Aside: hover-in-IDE shows the
   inferred type. Mini-aside on method syntax: *"Numbers have methods —
   `5_i32.abs()`, `3.14_f64.powi(2)`. The dot-syntax shows up in this
   lesson's exercise; we'll cover methods properly in Lesson 06."*
6. **Type annotations.** `let x: u32 = 5;` when you want to be explicit or
   when Rust can't infer (e.g., empty `Vec::new()`, which we'll see later).
   Annotations enforce: `let x: u32 = -1;` fails — show the diagnostic.
7. **Shadowing.** `let x = 5; let x = "five";` — re-binds `x` to a fresh
   value, possibly of a different type. Emphasize: this is NOT mutation.
   The old `x` is gone; a new `x` takes its name.
8. **`mut` vs shadowing.** Side-by-side table — the conceptual hinge.

   | aspect       | `let mut x = …; x = …;` | `let x = …; let x = …;` |
   |--------------|-------------------------|-------------------------|
   | binding      | same                    | new                     |
   | type         | must stay the same      | can change              |
   | old value    | overwritten             | dropped                 |
   | reader sees  | "this thing changes"    | "I'm done with that;    |
   |              |                         |  here's the next step"  |

   Common-mistake setup: writing `let mut x = 5; x = "hello";` and
   expecting it to work. (The compile-fail exercise drives this home.)
9. **`const`.** `const MAX_RETRIES: u32 = 3;` — truly immutable, **type
   required** (no inference), evaluated at compile time, lives for the
   whole program. Convention: SCREAMING_SNAKE_CASE. Show one realistic
   constant, e.g., `const HUNDRED: f64 = 100.0;` (foreshadows the main
   exercise).
10. **Wrap.** Three takeaways: immutability is the default; `mut` is a
    local opt-in; shadowing is a different tool for a different job. Next:
    Lesson 03 introduces control flow.

## Exercise spec

`lessons/02-variables/` follows the standard four-part lesson shape:

```
02-variables/
├── README.md
├── slides/
│   ├── index.html
│   └── slides.md
├── exercises/
│   ├── Cargo.toml
│   ├── src/lib.rs
│   ├── tests/exercise.rs
│   └── compile_fails/02-mut-cant-change-type.rs
└── solutions/
    ├── Cargo.toml
    ├── src/lib.rs
    └── tests/exercise.rs
```

Cargo package names follow the scaffolder's convention: `variables-exercises`
and `variables-solutions` (the lesson's "bare" name is `variables`).

### Warm-up: `fahrenheit_to_celsius`

Signature:

```rust
pub fn fahrenheit_to_celsius(f: f64) -> f64
```

The exercise stub (`#[must_use]` to satisfy workspace clippy::pedantic):

```rust
#[must_use]
pub fn fahrenheit_to_celsius(_f: f64) -> f64 {
    todo!("convert Fahrenheit to Celsius using (f - 32) * 5 / 9")
}
```

The reference solution uses shadowing once (`let f = f - 32.0;`) and one
explicit annotation (`let scaled: f64 = …`).

Tests (all use f64-exact values — `(F − 32) × 5 / 9` is exact when 5/9
appears inside a multiply-then-divide and the inputs avoid 1/9-style
denominators):

```rust
#[test] fn freezing()    { assert_eq!(fahrenheit_to_celsius(32.0), 0.0); }
#[test] fn boiling()     { assert_eq!(fahrenheit_to_celsius(212.0), 100.0); }
#[test] fn ten_celsius() { assert_eq!(fahrenheit_to_celsius(50.0), 10.0); }
```

### Main: `compound_interest`

Signature:

```rust
pub fn compound_interest(principal: f64, rate_percent: f64, years: u32) -> f64
```

The exercise stub (`#[must_use]` to satisfy workspace clippy::pedantic):

```rust
#[must_use]
pub fn compound_interest(_principal: f64, _rate_percent: f64, _years: u32) -> f64 {
    todo!("return principal * (1 + rate_percent/100)^years")
}
``` The reference solution:

- declares `const HUNDRED: f64 = 100.0;` at module scope
- uses shadowing (`let rate = rate_percent / HUNDRED;`)
- uses one explicit annotation (`let factor: f64 = 1.0 + rate;`)
- uses `f64::from(years)` (introduces method-on-primitive syntax) OR
  `years as i32` for the `powi` exponent — the spec accepts either; the
  reference solution uses both to introduce `f64::from` AND `as`
- ends with `principal * factor.powi(years as i32)`

Tests (all values dyadic so the f64 arithmetic is exact):

```rust
#[test] fn zero_principal_grows_to_zero()  { assert_eq!(compound_interest(0.0, 50.0, 10), 0.0); }
#[test] fn zero_rate_returns_principal()   { assert_eq!(compound_interest(1000.0, 0.0, 5), 1000.0); }
#[test] fn fifty_percent_two_years()       { assert_eq!(compound_interest(100.0, 50.0, 2), 225.0); }
#[test] fn twentyfive_percent_two_years()  { assert_eq!(compound_interest(200.0, 25.0, 2), 312.5); }
```

`1.5² = 2.25` and `1.25² = 1.5625` are both exactly representable in IEEE
754 binary64 (denominators are powers of 2), so `assert_eq!` is safe here
without approx-eq.

### Compile-fail: `02-mut-cant-change-type.rs`

Path: `exercises/compile_fails/02-mut-cant-change-type.rs`.

Ships broken; the student edits it until it compiles. The shipped state
fails to compile because it tries to reassign an `i32` binding to a
`&'static str`. The hint in the file's leading comment names the
misconception (`"mut keeps the same type"`) and prescribes the fix
(replace `x = "hello";` with `let x = "hello";`).

```rust
// Compile-fail exercise: this file MUST NOT compile until you fix it.
//
// `mut` lets you change a binding's VALUE, but not its TYPE. The line
// below tries to reassign `x` from an integer to a string slice. Read
// the rustc error: it will name the original type that Rust inferred
// and point at the offending assignment.
//
// Hint: the fix is NOT to add `mut` (we already have it). The fix is
// to use shadowing — replace the second line so it declares a NEW
// binding called `x` with `let`. The new `x` can have a different type
// because it's a separate binding that happens to reuse the name.

fn main() {
    let mut x = 5;
    x = "hello";
    println!("{x}");
}
```

`make test` (and CI) runs the compile-fails tool with `--expect broken`
and sees `ok` for the shipped state. `make verify LESSON=02-variables`
runs `--expect compiles` and refuses to pass until the student fixes the
file.

## README structure

`lessons/02-variables/README.md` follows Lesson 01's shape:

- **Title + one-paragraph hook**
- **Learning goals** — the five bullets above
- **Self-study notes** with these subsections (each ~3-6 sentences plus a
  small code block, mirroring the corresponding slide(s)):
  - The default is immutable
  - Opting into mutation with `mut`
  - Type inference and annotations
  - Shadowing
  - `mut` vs shadowing — when to use which (the heaviest section; the
    conceptual hinge)
  - `const`
- **Exercises** — pointer to `make verify LESSON=02-variables`
- **Solutions** — pointer to `solutions/src/lib.rs`

## Authoring conventions adopted by this lesson

Lesson 02 is the first lesson to formalize the **warm-up + main exercise**
pattern from the Go course's Phase 1. This means:

- `exercises/src/lib.rs` defines two stubs (warm-up first, then main)
- `tests/exercise.rs` groups its tests so the warm-up assertions appear
  before the main-exercise assertions
- The lesson README's `## Exercises` section names them explicitly

If this works well, `CONTRIBUTING.md` should be updated to codify the
warm-up + main convention for the remaining Phase 1 lessons (03 through
06). That update is **not** part of this lesson's spec — it's a follow-up
once the pattern is proven on at least one lesson.

## Done criteria

- `lessons/02-variables/` exists with the four-part structure
- Both crate manifests use the correct package names
  (`variables-exercises`, `variables-solutions`)
- `cargo test --package variables-solutions` → 7 tests pass (3 warm-up +
  4 main)
- `cargo test --manifest-path lessons/02-variables/exercises/Cargo.toml`
  → both stubs panic with `todo!()`, which is the intended undone state
- `cargo run --package compile-fails -- --expect broken lessons/02-variables`
  → ok
- `cargo run --package compile-fails -- --expect compiles lessons/02-variables`
  → fails (file ships broken, that's the point)
- `make ci` is green
- `make slides-build` produces `dist/lessons/02-variables/slides/index.html`
- `dist/index.html` now shows lesson 02 as a clickable lesson
  (no code change needed — the build-index tool detects it via the
  `slides/` directory presence)

## Open questions

None.
