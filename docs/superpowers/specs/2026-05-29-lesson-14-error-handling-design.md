# Lesson 14 — Error handling — design

The third lesson of Phase 3 (Abstraction). Errors in Rust are *values*,
not exceptions: `Result<T, E>`, propagated with the `?` operator. The
`?` operator converts the error type via `From` — exactly what
`thiserror`'s `#[from]` automates. Progression: `Result` + `?` with std
errors (warm-up) → a real custom error enum built with `thiserror`
(main). `anyhow` is taught conceptually as the application-side
counterpart. This is the first lesson whose graded crate uses an
external dependency.

## Audience and prerequisites

- Has completed Lessons 01-13
- Comfortable with enums + `match` (L05), `Option` (L05), structs (L06),
  traits (L12), and iterators (L11)
- Has the toolchain installed and `make verify` working

## Learning goals

By the end of this lesson, the student can:

1. Explain that Rust models recoverable errors as `Result<T, E>` values
   (no exceptions), and matched/handled like any enum
2. Use the `?` operator to propagate an error, early-returning the `Err`
3. Explain that `?` converts the error via the `From` trait, which is
   why a custom error type "just works" with `?`
4. Define a custom error enum with `thiserror`: `#[derive(Error)]`,
   per-variant `#[error("...")]` messages, and `#[from]` for automatic
   conversion
5. Recognize `anyhow::Result` + `.context()` as the application-side
   alternative, and the rule of thumb (libraries define typed errors
   with `thiserror`; applications use `anyhow`)

## Scope

In scope: `Result<T, E>` (recap of the enum, `Ok`/`Err`); the `?`
operator for propagation; `?` relying on `From` for error conversion;
converting `Option` to `Result` with `ok_or`; defining a custom error
enum with `thiserror` (`#[derive(Error)]`, `#[error("...")]`,
`#[from]`); `anyhow` (conceptual — `anyhow::Result`, `.context()`, when
to choose it). The exercises drill `Result` + `?` with std errors
(warm-up) and a `thiserror` custom error enum (main). New infrastructure:
add `thiserror = "2"` to the workspace dependencies.

Out of scope (deferred or skipped): `panic!`/`unwrap`/`expect` beyond a
one-line contrast (panics are *unrecoverable* errors — mentioned, not
drilled); `Box<dyn Error>` as an error type (thiserror/anyhow supersede
it for this course); error `source()` chains and `#[source]`/
`#[error(transparent)]` (brief mention at most); custom `Display`/
`std::error::Error` *by hand* (shown as "the boilerplate thiserror
removes", not exercised); `?` in `main` returning `Result`; `anyhow`
used in the graded exercises (it stays conceptual — only `thiserror` is
a lesson-crate dependency); the `?`-on-`Option` form beyond a mention;
fallible iterators / `collect` into `Result`. Error handling is
introduced as *Result + ? + typed errors*; the deeper ecosystem (error
chains, backtraces) waits for the production-services phase.

## New dependency infrastructure

This is the first lesson whose graded crate depends on an external
crate. Handling:

- `anyhow = "1"` is *already* in `[workspace.dependencies]` (used by the
  `tools/`), and in `Cargo.lock`. It stays conceptual here — the lesson
  crates do **not** depend on it.
- `thiserror` is **not** yet a workspace dependency. The plan adds
  `thiserror = "2"` to the root `[workspace.dependencies]` (resolves to
  2.0.x — verified 2.0.18 during design), and both lesson crates declare
  `thiserror = { workspace = true }` in their `[dependencies]`.
- `Cargo.lock` is updated (a `cargo build` pulls `thiserror`,
  `thiserror-impl`, and their `proc-macro2`/`quote`/`syn`/
  `unicode-ident` build deps) and committed. CI fetches from crates.io
  (no `--offline`/`--locked`), and the cache key already hashes
  `**/Cargo.toml`, so the new dep refetches cleanly.

## Slide arc (10 slides)

1. **Title — Error handling.** Hook: *"Rust has no exceptions. A
   function that can fail returns its error as a value — a `Result` —
   and the caller must deal with it. The `?` operator makes that
   ergonomic, and two crates make it pleasant."*
2. **Errors are values.** No `try`/`catch`. A fallible operation returns
   `Result<T, E>`: either `Ok(value)` or `Err(error)`. (Unrecoverable
   bugs use `panic!` — that's separate; this lesson is about
   *recoverable* errors.) You can't ignore a `Result` — it's
   `#[must_use]`.
3. **`Result<T, E>`.**
   ```rust
   fn parse(s: &str) -> Result<i32, std::num::ParseIntError> {
       s.parse::<i32>()   // returns Ok(n) or Err(e)
   }

   match parse("42") {
       Ok(n) => println!("got {n}"),
       Err(e) => println!("failed: {e}"),
   }
   ```
   `Result` is just an enum — match it like any other.
4. **The `?` operator.**
   ```rust
   fn sum_fields(a: &str, b: &str) -> Result<i32, std::num::ParseIntError> {
       let x: i32 = a.parse()?;   // if Err, return it now
       let y: i32 = b.parse()?;
       Ok(x + y)
   }
   ```
   `?` unwraps an `Ok`, or early-returns the `Err`. It replaces a pile
   of `match`es with one character.
5. **`?` converts the error via `From`.** When the `Err` type of the
   `?`-expression differs from the function's error type, `?` calls
   `From::from` to convert it. So if your function returns
   `Result<_, MyError>` and `MyError: From<ParseIntError>`, then
   `s.parse()?` *just works* — the `ParseIntError` becomes a `MyError`
   automatically. This is the hook for custom error types.
6. **Converting `Option` to `Result`.**
   ```rust
   let (key, value) = input.split_once('=').ok_or(ConfigError::MissingEquals)?;
   ```
   `Option::ok_or` turns `None` into `Err(...)`, so you can `?` an
   absent value into a typed error.
7. **Custom errors with `thiserror`.**
   ```rust
   use thiserror::Error;

   #[derive(Debug, Error)]
   pub enum ConfigError {
       #[error("missing '=' in setting")]
       MissingEquals,
       #[error("value is not a valid integer: {0}")]
       BadValue(#[from] std::num::ParseIntError),  // generates From + powers ?
   }
   ```
   `#[derive(Error)]` writes the `Display`/`Error` boilerplate;
   `#[error("...")]` is the message; `#[from]` generates the `From` impl
   that `?` uses. Each variant is one failure mode.
8. **`anyhow` — the application side.**
   ```rust
   use anyhow::{Context, Result};

   fn load() -> Result<Config> {
       let text = std::fs::read_to_string("app.toml")
           .context("reading app.toml")?;   // attach context, keep ?-ing
       // ...
   }
   ```
   `anyhow::Result<T>` holds *any* error behind one type, with
   `.context()` for human breadcrumbs. **Rule of thumb:** libraries
   define typed errors with `thiserror` (callers can match on them);
   applications use `anyhow` (you just want it to bubble up with
   context).
9. **Putting it together.** Walk through the exercises: `sum_fields`
   propagates a std `ParseIntError` with `?` (warm-up), and
   `parse_setting` builds a `ConfigError` enum with `thiserror` —
   `ok_or(...)?`, a validation check, and `parse()?` converting via
   `#[from]` (main). The compile-fail shows `?` used in a function that
   doesn't return `Result`.
10. **Wrap — robust errors.** Five takeaways: errors are `Result`
    values, not exceptions; `?` propagates and early-returns; `?`
    converts via `From`; `thiserror` derives typed library errors with
    `#[error]`/`#[from]`; `anyhow` carries any error with `.context()`
    for apps. Next: **Lesson 15 — Modules, crates, workspaces**.

## Exercise spec

`lessons/14-error-handling/` follows the standard four-part lesson
shape, plus a dependency in each crate's `Cargo.toml`:

```
14-error-handling/
├── README.md
├── slides/
│   ├── index.html
│   └── slides.md
├── exercises/
│   ├── Cargo.toml          # adds thiserror = { workspace = true }
│   ├── src/lib.rs
│   ├── tests/exercise.rs
│   └── compile_fails/14-question-mark-in-unit-fn.rs
└── solutions/
    ├── Cargo.toml          # adds thiserror = { workspace = true }
    ├── src/lib.rs
    └── tests/exercise.rs
```

Cargo package names: `error-handling-exercises` and
`error-handling-solutions` (the lesson's "bare" name is
`error-handling`; the import idents are `error_handling_exercises` /
`error_handling_solutions`). This matches the build-index master
registry slug `error-handling`, so the landing page links it without any
change.

### Cargo.toml dependency

Both `exercises/Cargo.toml` and `solutions/Cargo.toml` (which the
scaffolder generates without a `[dependencies]` section) gain:

```toml
[dependencies]
thiserror = { workspace = true }
```

placed before the existing `[lints]` section. And the root
`Cargo.toml` `[workspace.dependencies]` gains `thiserror = "2"`.

### Exercise stub (`exercises/src/lib.rs`)

The `ConfigError` enum ships **complete** (it is the thiserror teaching
artifact — students see a worked custom-error type), with the two
function bodies as `todo!()`. Because the enum and signatures are
complete, the crate and its tests *compile* in the undone state; the
tests fail at runtime with the `todo!()` panic, like every prior lesson.

```rust
//! Lesson 14 — exercises.
//!
//! Implement `sum_fields` (warm-up) and `parse_setting` (main) so that
//! `cargo test --manifest-path
//! lessons/14-error-handling/exercises/Cargo.toml` passes. The
//! `ConfigError` enum is given as a worked example of a `thiserror`
//! custom error type. The tests live in `tests/exercise.rs`.

use thiserror::Error;

pub fn sum_fields(_a: &str, _b: &str) -> Result<i32, std::num::ParseIntError> {
    todo!("parse both fields with `?` and return their sum")
}

#[derive(Debug, Error, PartialEq)]
pub enum ConfigError {
    #[error("missing '=' in setting")]
    MissingEquals,
    #[error("key is empty")]
    EmptyKey,
    #[error("value is not a valid integer: {0}")]
    BadValue(#[from] std::num::ParseIntError),
}

pub fn parse_setting(_input: &str) -> Result<(String, i32), ConfigError> {
    todo!("split on '=', validate the key, then parse the value with `?`")
}
```

### Warm-up: `sum_fields`

Reference solution:

```rust
pub fn sum_fields(a: &str, b: &str) -> Result<i32, std::num::ParseIntError> {
    let x: i32 = a.parse()?;
    let y: i32 = b.parse()?;
    Ok(x + y)
}
```

Pedagogical packing: two `?` propagations of a std error
(`ParseIntError`), then a combine. The `?` is genuinely needed (there's
a transform after it), so `clippy::needless_question_mark` does not
fire. Note: this function returns `Result`, which is already
`#[must_use]`, so it must **not** carry a `#[must_use]` attribute —
doing so trips `clippy::double_must_use` (verified during design; this
is the opposite of earlier lessons whose functions returned plain
values).

Four tests:

```rust
#[test]
fn warmup_sum_ok() {
    assert_eq!(sum_fields("2", "3"), Ok(5));
}

#[test]
fn warmup_sum_first_bad() {
    assert!(sum_fields("x", "3").is_err());
}

#[test]
fn warmup_sum_second_bad() {
    assert!(sum_fields("2", "y").is_err());
}

#[test]
fn warmup_sum_negative() {
    assert_eq!(sum_fields("-4", "10"), Ok(6));
}
```

### Main: `parse_setting`

Reference solution:

```rust
pub fn parse_setting(input: &str) -> Result<(String, i32), ConfigError> {
    let (key, value) = input.split_once('=').ok_or(ConfigError::MissingEquals)?;
    if key.is_empty() {
        return Err(ConfigError::EmptyKey);
    }
    let value: i32 = value.parse()?;
    Ok((key.to_string(), value))
}
```

Pedagogical packing: `split_once('=')` returns `Option<(&str, &str)>`;
`.ok_or(ConfigError::MissingEquals)?` converts `None` to a typed `Err`
and `?`-propagates it (Option→Result). The empty-key check is an
explicit `return Err(...)`. `value.parse()?` parses the value and, on
failure, `?` converts the `ParseIntError` into `ConfigError::BadValue`
*automatically* via the `#[from]`-generated `From` impl — the lesson's
centerpiece. Returns the `(key, value)` tuple. Like `sum_fields`, it
returns `Result` and must not carry `#[must_use]`.

Four tests (the `BadValue` case uses `matches!` because the wrapped
`ParseIntError` is awkward to construct for `assert_eq!`):

```rust
#[test]
fn main_setting_ok() {
    assert_eq!(parse_setting("port=8080"), Ok(("port".to_string(), 8080)));
}

#[test]
fn main_setting_missing_equals() {
    assert_eq!(parse_setting("noequals"), Err(ConfigError::MissingEquals));
}

#[test]
fn main_setting_empty_key() {
    assert_eq!(parse_setting("=5"), Err(ConfigError::EmptyKey));
}

#[test]
fn main_setting_bad_value() {
    assert!(matches!(
        parse_setting("port=abc"),
        Err(ConfigError::BadValue(_))
    ));
}
```

**Eight tests total** (four warm-up + four main). `ConfigError` derives
`PartialEq` so the unit-variant cases compare with `assert_eq!`;
`ParseIntError` itself implements `PartialEq`, so the derive succeeds.

### Compile-fail: `14-question-mark-in-unit-fn.rs`

Path: `exercises/compile_fails/14-question-mark-in-unit-fn.rs`. A
self-contained, std-only file (no `thiserror`, since the compile-fails
tool compiles single files with `rustc`) whose function uses `?` but
returns `()`. Ships broken; the student changes the return type to a
`Result` until it compiles.

```rust
// Compile-fail exercise: this file MUST NOT compile until you fix it.
//
// The `?` operator only works inside a function whose return type can
// represent failure — a `Result` or an `Option`. On an `Err`, `?` needs
// to *return that error from the function*; a function returning `()`
// has nowhere to return it to.
//
// rustc reports E0277: "the `?` operator can only be used in a function
// that returns `Result` or `Option`".
//
// The fix: give the function a fallible return type. Have it return
// `Result<(), std::num::ParseIntError>`, `?`-parse, do the work, then
// return `Ok(())`.
//
// Hint: change the signature to
//     fn parse_and_double(input: &str) -> Result<(), std::num::ParseIntError>
// and end the body with `Ok(())`.

fn parse_and_double(input: &str) {
    let n: i32 = input.parse()?;
    println!("{}", n * 2);
}

fn main() {
    parse_and_double("21");
}
```

Pass condition: the student changes the return type to
`Result<(), std::num::ParseIntError>` (and ends with `Ok(())`), or
otherwise makes the `?` valid. rustc reports E0277 with "cannot use the
`?` operator in a function that returns `()`" — verified during design.

This is the lesson's centerpiece for `?`: the operator's whole job is to
*return the error from the current function*, so the function must have
a type that can carry one.

## README structure

`lessons/14-error-handling/README.md` follows the established shape:

- **Title + one-paragraph hook**
- **Learning goals** — the five bullets above
- **Self-study notes** with five subsections:
  - Errors are values — `Result<T, E>`
  - The `?` operator
  - `?` converts the error via `From`
  - Custom errors with `thiserror`
  - `anyhow` — the application side (and when to use which)
- **Exercises** — four subsections: Warm-up (`sum_fields`), Main
  (`parse_setting`), Compile-fail, Run
- **Solutions** — pointer to `solutions/src/lib.rs`

Each `###` subsection runs ~4-6 sentences plus a small code block. The
"`thiserror`" and "`?` converts via `From`" sections are the heaviest —
they carry the lesson's payoff.

## Lint expectations

Lesson 14's reference solution code is clippy-clean (with `clippy::all`
+ `clippy::pedantic` denied) without `#[allow]` attributes — verified
during design:

- Neither `sum_fields` nor `parse_setting` carries `#[must_use]`: both
  return `Result`, which is already `#[must_use]`, so adding the
  attribute trips `clippy::double_must_use`. (This is the opposite of
  earlier lessons, whose functions returned plain `u32`/`Vec` and *did*
  need `#[must_use]`.)
- `sum_fields` uses `?` followed by a transform, so
  `clippy::needless_question_mark` does not fire.
- `parse_setting` uses `.ok_or(ConfigError::MissingEquals)?` — the
  argument is a cheap enum-variant value, not a function call, so
  `clippy::or_fun_call` (which would suggest `ok_or_else`) does not fire.
- `ConfigError` derives `Debug, Error, PartialEq`; the `#[from]` variant
  coexists with the `PartialEq` derive (`ParseIntError: PartialEq`).
- In the *exercise stub*, the two `todo!()` bodies with unused
  `_a`/`_b`/`_input` params compile and lint clean (verified).

If clippy fires on anything unexpected, fix the code rather than adding
an allow, and report it.

## Done criteria

- `lessons/14-error-handling/` exists with the four-part structure
- Root `Cargo.toml` `[workspace.dependencies]` includes `thiserror = "2"`;
  both lesson `Cargo.toml`s declare `thiserror = { workspace = true }`;
  `Cargo.lock` is updated and committed
- Cargo manifests use the correct package names
  (`error-handling-exercises`, `error-handling-solutions`)
- `exercises/src/lib.rs` and `solutions/src/lib.rs` define the same
  `ConfigError` enum and the `sum_fields` / `parse_setting` signatures;
  the exercise ships `todo!()` bodies, the solution ships real bodies
- `cargo test --package error-handling-solutions` → 8 tests pass
- `cargo test --manifest-path lessons/14-error-handling/exercises/Cargo.toml`
  → compiles, all 8 tests panic with `not yet implemented` (the intended
  undone state)
- `cargo run --package compile-fails -- --expect broken lessons/14-error-handling`
  → ok
- `cargo run --package compile-fails -- --expect compiles lessons/14-error-handling`
  → fails (file ships broken, that's the point)
- `make ci` is green
- `make slides-build` produces `dist/lessons/14-error-handling/slides/index.html`
- `dist/index.html` lists lesson 14 as a clickable link (registry slug
  `error-handling` already matches this directory name)
- One push to `origin/main` triggers a green CI run and a green Deploy
  run; `https://rust.ristkari.dev/lessons/14-error-handling/slides/`
  returns 200

## Open questions

None.
