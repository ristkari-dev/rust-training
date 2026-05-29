# Error handling

> Rust has no exceptions. A function that can fail returns its error as a value — a `Result` — and the caller must deal with it. The `?` operator makes that ergonomic, and two crates make it pleasant.

---

## Errors are values

No `try` / `catch`. A fallible operation returns `Result<T, E>`: either `Ok(value)` or `Err(error)`.

You can't ignore a `Result` — it's `#[must_use]`. (Unrecoverable bugs use `panic!` — that's separate; this lesson is about *recoverable* errors.)

---

## `Result<T, E>`

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

---

## The `?` operator

```rust
fn sum_fields(a: &str, b: &str) -> Result<i32, std::num::ParseIntError> {
    let x: i32 = a.parse()?;   // if Err, return it now
    let y: i32 = b.parse()?;
    Ok(x + y)
}
```

`?` unwraps an `Ok`, or early-returns the `Err`. It replaces a pile of `match`es with one character.

---

## `?` converts the error via `From`

When the `Err` type of the `?`-expression differs from the function's error type, `?` calls `From::from` to convert it.

So if your function returns `Result<_, MyError>` and `MyError: From<ParseIntError>`, then `s.parse()?` *just works* — the error converts automatically. This is the hook for custom error types.

---

## Converting `Option` to `Result`

```rust
let (key, value) = input.split_once('=').ok_or(ConfigError::MissingEquals)?;
```

`Option::ok_or` turns `None` into `Err(...)`, so you can `?` an absent value into a typed error.

---

## Custom errors with `thiserror`

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

`#[derive(Error)]` writes the `Display`/`Error` boilerplate; `#[error("...")]` is the message; `#[from]` generates the `From` impl that `?` uses.

---

## `anyhow` — the application side

```rust
use anyhow::{Context, Result};

fn load() -> Result<Config> {
    let text = std::fs::read_to_string("app.toml")
        .context("reading app.toml")?;   // attach context, keep ?-ing
    // ...
}
```

`anyhow::Result<T>` holds *any* error behind one type, with `.context()` for breadcrumbs.

**Rule of thumb:** libraries define typed errors with `thiserror`; applications use `anyhow`.

---

## Putting it together

Today's exercises:

- **Warm-up** `sum_fields` — propagate a std `ParseIntError` with `?`
- **Main** `parse_setting` — build a `ConfigError` enum with `thiserror`: `ok_or(...)?`, a validation check, and `parse()?` converting via `#[from]`

The compile-fail shows `?` used in a function that doesn't return `Result`.

---

## Wrap — robust errors

- Errors are `Result` values, not exceptions
- `?` propagates and early-returns the `Err`
- `?` converts the error via `From`
- `thiserror` derives typed library errors with `#[error]` / `#[from]`
- `anyhow` carries any error with `.context()` for apps

Next: **Lesson 15 — Modules, crates, workspaces**.
