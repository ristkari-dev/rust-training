# Hello, Rust

Your first compile error is a feature, not a bug.

---

## Why Rust

- Memory safety without a garbage collector
- Fearless concurrency
- A compiler that teaches you

---

## The toolchain

`rustup` installs:

- `cargo` — build tool and package manager
- `rustc` — the compiler
- `rustfmt` — formatter (defaults are correct)
- `clippy` — linter that catches common mistakes

```bash
rustup show
cargo --version
```

---

## Your first program

```rust
fn main() {
    println!("Hello, Rust!");
}
```

- `fn main()` is the entry point
- `println!` is a macro (note the `!`)

---

## Your first library function

```rust
pub fn greet(name: &str) -> String {
    format!("Hello, {name}!")
}
```

- `pub` makes it visible outside the crate
- `&str` is a borrowed string slice (more soon)
- `String` is an owned, heap-allocated string

---

## Compile errors are pedagogy

```rust
let x = 1;
x = 2; // ERROR: cannot assign twice to immutable variable
```

Read the error. The compiler usually tells you the fix.

---

## Wrap

- Installed the toolchain, ran `cargo`
- Wrote a function, made tests pass
- Met our first borrow-checker-adjacent error
- Next: variables, mutability, and shadowing
