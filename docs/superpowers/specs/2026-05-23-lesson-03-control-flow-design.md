# Lesson 03 — Control flow & functions — design

The third lesson of the Rust training course. Builds on Lesson 02 (which
covered `let`/`mut`/shadowing/type-inference/annotations/`const`) by
adding the control-flow constructs and the expression-vs-statement
insight that underpins how Rust functions return values.

## Audience and prerequisites

- Has completed Lessons 01 and 02
- Comfortable with `let`/`mut`/shadowing and reading inferred types
- Has the toolchain installed and `make verify` working

## Learning goals

By the end of this lesson, the student can:

1. Use `if` as an expression and read its result
2. Choose between `for`, `while`, and `loop` for different iteration shapes
3. Use `break` and `continue` to alter loop control
4. Explain the difference between statements (which produce `()`) and
   expressions (which produce values)
5. Return values from functions by tail expression and by explicit `return`

## Scope

In scope: `if`/`else if`/`else` as expressions; `for ... in <range>`;
`while`; `loop`; `break`; `continue`; expressions vs statements; the unit
type `()`; tail-expression returns; explicit `return`.

Out of scope (deferred): `match` and `if let` (Lesson 05 — Pattern
matching & enums); `break value;` returning a value from `loop` is
mentioned briefly on a slide but not exercised (a richer treatment lands
when we cover loops-as-expressions in a later lesson); pattern matching
in `for` bindings (e.g., `for (i, x) in …`) — too early without tuples
(Lesson 04).

## Slide arc (10 slides)

1. **Title — Control flow & functions.** Hook: *"In Rust, control flow
   doesn't just *do* things — it *evaluates* to things. Once you see
   that, the rest follows."*
2. **Recap.** Lesson 02 introduced `let mut`. Today we use it inside
   loops, and meet the deeper reason for some of Rust's syntax:
   control-flow constructs are expressions.
3. **`if` as an expression.** Two forms in one slide:
   `let max = if a > b { a } else { b };` and an `if`/`else if`/`else`
   chain returning `&'static str` (foreshadows the warm-up).
4. **`for ... in range`.** The most common Rust loop.
   `for i in 1..=n { … }`. Note: `..=` is inclusive, `..` is exclusive.
   One concrete example: sum 1..=10.
5. **`while`.** Condition-controlled iteration. Show a divide-by-10
   loop (foreshadows the main exercise).
6. **`loop`.** The unconditional loop paired with `break`. Brief aside:
   `loop { break value; }` can produce a value — useful for
   "retry-until-success" patterns, covered properly in Lesson 16.
7. **`break` and `continue`.** Early exit (`break`) and skip-to-next
   (`continue`). One small example each.
8. **Expressions vs statements.** The Rust-distinctive insight.
   Statements end with `;` and produce `()` (the unit type).
   Expressions have a value. `if`, `loop`, `{ block }`, and most
   everything else are expressions. Slide ends with: *"A trailing
   semicolon is not whitespace — it changes what your code returns."*
9. **Functions: tail returns.** `fn double(n: i32) -> i32 { n * 2 }` —
   no `return`, no semicolon, the tail expression IS the return value.
   Contrast with explicit `return` (which is a statement and needs a
   semicolon). The compile-fail exercise drives slides 8 + 9 home.
10. **Wrap.** Four takeaways: `if` is an expression, three loops cover
    different shapes, statements produce `()`, functions return their
    tail expression. Next: Lesson 04 — compound types (tuples, arrays,
    slices, String vs &str).

## Exercise spec

`lessons/03-control-flow/` follows the standard four-part lesson shape:

```
03-control-flow/
├── README.md
├── slides/
│   ├── index.html
│   └── slides.md
├── exercises/
│   ├── Cargo.toml
│   ├── src/lib.rs
│   ├── tests/exercise.rs
│   └── compile_fails/03-trailing-semicolon.rs
└── solutions/
    ├── Cargo.toml
    ├── src/lib.rs
    └── tests/exercise.rs
```

Cargo package names: `control-flow-exercises` and `control-flow-solutions`
(the lesson's "bare" name is `control-flow`).

### Warm-up: `classify`

Signature:

```rust
pub fn classify(n: i32) -> &'static str
```

The exercise stub (`#[must_use]` to satisfy workspace `clippy::pedantic`):

```rust
#[must_use]
pub fn classify(_n: i32) -> &'static str {
    todo!("return \"negative\" / \"zero\" / \"positive\" using an if-as-expression")
}
```

Reference solution (uses the if-as-expression directly as the function's
tail — no `return`, no intermediate binding):

```rust
#[must_use]
pub fn classify(n: i32) -> &'static str {
    if n < 0 {
        "negative"
    } else if n == 0 {
        "zero"
    } else {
        "positive"
    }
}
```

Three tests:

```rust
#[test] fn classify_negative() { assert_eq!(classify(-42), "negative"); }
#[test] fn classify_zero()     { assert_eq!(classify(0),   "zero"); }
#[test] fn classify_positive() { assert_eq!(classify(7),   "positive"); }
```

### Main: `count_digits`

Signature:

```rust
pub fn count_digits(n: u32) -> u32
```

The exercise stub:

```rust
#[must_use]
pub fn count_digits(_n: u32) -> u32 {
    todo!("count the number of decimal digits; treat 0 as having 1 digit")
}
```

Reference solution (uses explicit `return` for the edge case, two `let
mut` bindings, a `while` loop, and a tail-expression return — covering
all the lesson's keywords in one tight function):

```rust
#[must_use]
pub fn count_digits(n: u32) -> u32 {
    if n == 0 {
        return 1;
    }
    let mut remaining = n;
    let mut count: u32 = 0;
    while remaining > 0 {
        remaining /= 10;
        count += 1;
    }
    count
}
```

Five tests (the `u32::MAX` case proves the function handles ten-digit
inputs without overflow in the count):

```rust
#[test] fn count_digits_zero()    { assert_eq!(count_digits(0),             1); }
#[test] fn count_digits_single()  { assert_eq!(count_digits(7),             1); }
#[test] fn count_digits_two()     { assert_eq!(count_digits(42),            2); }
#[test] fn count_digits_three()   { assert_eq!(count_digits(100),           3); }
#[test] fn count_digits_u32_max() { assert_eq!(count_digits(4_294_967_295), 10); }
```

**Eight tests total** (three warm-up + five main).

### Compile-fail: `03-trailing-semicolon.rs`

Path: `exercises/compile_fails/03-trailing-semicolon.rs`. Ships broken;
the student edits it until it compiles. Compiles only after removing the
trailing semicolon in `double`'s body so the expression becomes the tail
return.

```rust
// Compile-fail exercise: this file MUST NOT compile until you fix it.
//
// In Rust, a function's body is a block; the block's TAIL EXPRESSION
// (the last expression with no trailing semicolon) is what the function
// returns. A semicolon at the end turns the expression into a STATEMENT,
// which produces the unit type `()` instead of a value. The function
// below declares it returns `i32` but its body returns `()` — the
// compiler will tell you exactly that.
//
// Hint: read the rustc error. It will mention "expected `i32`, found `()`"
// and point at the closing brace of `double`. The fix is to remove ONE
// character.

fn double(n: i32) -> i32 {
    n * 2;
}

fn main() {
    let d = double(7);
    println!("{d}");
}
```

`make test` (and CI) runs the compile-fails tool with `--expect broken`
and sees `ok` for the shipped state. `make verify LESSON=03-control-flow`
runs `--expect compiles` and refuses to pass until the student removes
the offending `;`.

This exercise is the perfect mirror to slides 8-9: rustc's diagnostic
literally says "expected `i32`, found `()`", naming the same unit type
the slide just introduced.

### Why no `for`-loop exercise

The main exercise's natural shape is `while` (divide-by-10 until zero).
Adding a `for`-based exercise would either duplicate `while` or require
new concepts (collections) the course hasn't introduced yet. `for ... in
range` is covered in the slides and demonstrated with a 4-line "sum
1..=10" example; later lessons (starting with Lesson 04) use `for` in
exercises naturally as compound types arrive.

## README structure

`lessons/03-control-flow/README.md` follows Lesson 02's shape:

- **Title + one-paragraph hook**
- **Learning goals** — the five bullets above
- **Self-study notes** with these subsections:
  - `if` as an expression
  - The three loops
  - `break` and `continue`
  - Expressions vs statements — and the unit type
  - Functions: tail returns and explicit `return`
- **Exercises** — pointer to `make verify LESSON=03-control-flow`
- **Solutions** — pointer to `solutions/src/lib.rs`

Each `###` subsection runs ~4-6 sentences plus a small code block,
mirroring the corresponding slide(s). The "Expressions vs statements"
section is the heaviest — it's the conceptual climax — and explicitly
introduces the unit type `()` since it appears in the rustc error
message students will see in the compile-fail exercise.

## Conventions adopted

This is the second lesson to use the warm-up + main exercise pattern
established in Lesson 02. If this lesson lands cleanly, the next
follow-up should be a small documentation pass to formalize the
convention in `CONTRIBUTING.md` so that Lessons 04 onward use it
consistently — but that is out of scope for *this* lesson's spec.

The `#![allow(clippy::float_cmp)]` workaround used in Lesson 02's test
files is not needed here — Lesson 03's tests compare integers and
`&'static str` slices, where `assert_eq!` is unambiguously safe.

## Done criteria

- `lessons/03-control-flow/` exists with the four-part structure
- Cargo manifests use the correct package names (`control-flow-exercises`,
  `control-flow-solutions`)
- `cargo test --package control-flow-solutions` → 8 tests pass
- `cargo test --manifest-path lessons/03-control-flow/exercises/Cargo.toml`
  → both stubs panic with `todo!()`, the intended undone state
- `cargo run --package compile-fails -- --expect broken lessons/03-control-flow`
  → ok
- `cargo run --package compile-fails -- --expect compiles lessons/03-control-flow`
  → fails (file ships broken, that's the point)
- `make ci` is green
- `make slides-build` produces `dist/lessons/03-control-flow/slides/index.html`
- `dist/index.html` now shows lesson 03 as a clickable lesson (no code
  change needed — `build-index` detects it via the `slides/` directory
  presence)
- One push to `origin/main` triggers a green CI run and a green Deploy run;
  `https://rust.ristkari.dev/lessons/03-control-flow/slides/` returns 200

## Open questions

None.
