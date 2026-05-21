# Lesson 01 — Hello, Rust

Your first Rust program. By the end you will have installed the
toolchain, run a binary built by `cargo`, and met your first compiler
error.

## Learning goals

- Install `rustup` and verify `cargo`, `rustc`, and `rustfmt` work
- Read a minimal `fn main()` and understand the `println!` macro
- Write and call a small library function (`greet`)
- Experience a deliberate compile error and read the diagnostic

## Self-study notes

### The toolchain

Rust ships through `rustup`, a version manager that installs `cargo`
(build tool), `rustc` (compiler), `rustfmt` (formatter), and `clippy`
(linter). Install it from <https://rustup.rs> and then run:

```bash
rustc --version
cargo --version
```

You should see version `1.85` or newer.

### The shape of a program

A Rust binary's entry point is `fn main()`. A Rust library exposes
functions other crates can call. This lesson's exercise asks you to write
a library function called `greet`. The tests already know how `greet`
should behave; your job is to make them stop failing.

### Macros vs functions

`println!` is a macro, not a function. The `!` is the giveaway. For now
treat it as "print this and a newline." We will explore macros properly
later in the course.

## Exercises

### Failing tests

Open `exercises/src/lib.rs` and implement `greet` so that the tests pass:

```bash
cargo test --package hello-rust-exercises
```

### Compile-fail exercise

Open `exercises/compile_fails/01-immutable-binding.rs`. The file does
**not** compile — read the comment, then fix it. The check passes once
the file no longer compiles cleanly *and* it makes the change the comment
asks for (which is the same thing here: an immutable binding cannot be
reassigned).

Verify:

```bash
cargo run --package compile-fails -- lessons/01-hello-rust
```

The runner reports `ok` when the file still fails to compile, and
`FAIL` when it compiles cleanly.

## Solutions

See `solutions/src/lib.rs` for the reference implementation. Try first.
