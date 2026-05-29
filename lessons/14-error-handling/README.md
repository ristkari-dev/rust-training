# Lesson 14 — Error handling

Rust has no exceptions. A function that can fail returns its error as a
*value* — a `Result<T, E>` — and the caller must deal with it. The `?`
operator makes propagation ergonomic, and two crates make custom errors
pleasant: `thiserror` for libraries and `anyhow` for applications. This
lesson covers `Result`, `?`, and `thiserror`.

## Learning goals

- Explain that Rust models recoverable errors as `Result<T, E>` values
  (no exceptions)
- Use the `?` operator to propagate an error, early-returning the `Err`
- Explain that `?` converts the error via the `From` trait
- Define a custom error enum with `thiserror` (`#[derive(Error)]`,
  `#[error("...")]`, `#[from]`)
- Recognize `anyhow` as the application-side alternative, and when to
  use which

## Self-study notes

### Errors are values — `Result<T, E>`

A fallible operation returns `Result<T, E>`: either `Ok(value)` or
`Err(error)`. It's just an enum — match it like any other:

```rust
fn parse(s: &str) -> Result<i32, std::num::ParseIntError> {
    s.parse::<i32>()
}

match parse("42") {
    Ok(n) => println!("got {n}"),
    Err(e) => println!("failed: {e}"),
}
```

`Result` is `#[must_use]`, so you can't silently ignore a failure.
(Unrecoverable bugs use `panic!` — that's separate from recoverable
errors.)

### The `?` operator

Matching every `Result` by hand is noisy. The `?` operator unwraps an
`Ok`, or early-returns the `Err`:

```rust
fn sum_fields(a: &str, b: &str) -> Result<i32, std::num::ParseIntError> {
    let x: i32 = a.parse()?;   // on Err, return it now
    let y: i32 = b.parse()?;
    Ok(x + y)
}
```

One character replaces a pile of `match`es. `?` only works in a function
whose return type can carry the error (a `Result` or `Option`).

### `?` converts the error via `From`

When the error from a `?`-expression differs from the function's error
type, `?` calls `From::from` to convert it. So if your function returns
`Result<_, MyError>` and `MyError: From<ParseIntError>`, then
`s.parse()?` *just works* — the `ParseIntError` becomes a `MyError`
automatically. This is what makes custom error types ergonomic. You can
also turn an `Option` into a `Result` to `?` it:

```rust
let (key, value) = input.split_once('=').ok_or(ConfigError::MissingEquals)?;
```

`Option::ok_or` maps `None` to `Err(...)`.

### Custom errors with `thiserror`

Writing `Display` and `std::error::Error` by hand is boilerplate. The
`thiserror` crate derives them:

```rust
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("missing '=' in setting")]
    MissingEquals,
    #[error("value is not a valid integer: {0}")]
    BadValue(#[from] std::num::ParseIntError),
}
```

`#[error("...")]` is the `Display` message; `#[from]` generates the
`From` impl that `?` uses. Each variant is one failure mode the caller
can match on.

### `anyhow` — the application side (and when to use which)

For applications you often don't need a typed error — you just want it
to bubble up with context:

```rust
use anyhow::{Context, Result};

fn load() -> Result<String> {
    let text = std::fs::read_to_string("app.toml")
        .context("reading app.toml")?;
    Ok(text)
}
```

`anyhow::Result<T>` holds *any* error behind one type, and `.context()`
adds human breadcrumbs. **Rule of thumb:** libraries define typed errors
with `thiserror` (so callers can match); applications use `anyhow`.

## Exercises

### Warm-up: `sum_fields`

Implement `sum_fields(a: &str, b: &str) -> Result<i32, std::num::ParseIntError>`
that parses both strings as integers and returns their sum, propagating
any parse error with `?`:

```rust
pub fn sum_fields(a: &str, b: &str) -> Result<i32, std::num::ParseIntError> {
    // let x: i32 = a.parse()?; ... Ok(x + y)
    todo!()
}
```

### Main: `parse_setting`

The exercises crate ships a worked `thiserror` error type:

```rust
#[derive(Debug, Error, PartialEq)]
pub enum ConfigError {
    #[error("missing '=' in setting")]
    MissingEquals,
    #[error("key is empty")]
    EmptyKey,
    #[error("value is not a valid integer: {0}")]
    BadValue(#[from] std::num::ParseIntError),
}
```

Implement `parse_setting(input: &str) -> Result<(String, i32), ConfigError>`.
Split `input` on `'='` (`MissingEquals` if there's no `=`), reject an
empty key (`EmptyKey`), then parse the value with `?` — which converts a
`ParseIntError` into `BadValue` automatically via the `#[from]`.

### Compile-fail

`exercises/compile_fails/14-question-mark-in-unit-fn.rs` uses `?` inside
a function that returns `()`, which the compiler rejects (E0277 — `?`
needs a function returning `Result` or `Option`). Fix it by giving the
function a `Result<(), std::num::ParseIntError>` return type and ending
with `Ok(())`.

### Run

```bash
make verify LESSON=14-error-handling
```

This runs your exercise tests and asserts the compile-fail file now
compiles.

## Solutions

See `solutions/src/lib.rs` for the reference implementations. Try the
exercises before peeking.
