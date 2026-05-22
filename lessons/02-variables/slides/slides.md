# Variables, types, mutability

> In Rust, the default is no mutation. The type system makes that practical.

---

## Recap

Lesson 01 showed this error:

```rust
let x = 1;
x = 2; // ERROR: cannot assign twice to immutable variable `x`
```

Now let's understand why, and learn what to do about it.

---

## `let` — the default

```rust
let x = 5;
```

- `x` is bound to `5`
- `x` cannot be reassigned

This is the default for every binding.

Most variables in a typical Rust program are immutable.

---

## `let mut` — opting into mutation

```rust
let mut counter = 0;
counter = counter + 1;
counter = counter + 1;
```

- `mut` is a deliberate, local opt-in
- It signals to future readers: *"yes, this thing changes"*

Note: prefer immutable when you can. Reach for `mut` only when you need it.

---

## Type inference

Rust infers types from values:

```rust
let x = 5;       // i32
let y = 5.0;     // f64
let ok = true;   // bool
```

Hovering over a binding in your editor shows the inferred type.

Note: aside on dot-syntax — numbers have methods. You'll see `factor.powi(2)` in this lesson's main exercise. `.powi` is a method on `f64`. We'll cover methods properly with structs in Lesson 06.

---

## Type annotations

```rust
let x: u32 = 5;
let pi: f64 = 3.14159;
```

When to annotate:

- You want to be explicit for the reader
- Rust can't infer (e.g., `Vec::new()` later)

Annotations enforce:

```rust
let x: u32 = -1; // ERROR: -1 is not a u32
```

---

## Shadowing

```rust
let x = 5;
let x = x + 1;    // new x, value 6
let x = "five";   // new x, type &str
```

This is NOT mutation. Each `let` makes a fresh binding.

The old `x` is dropped when the new one shadows it.

---

## `mut` vs shadowing

|                | `let mut x = …; x = …;` | `let x = …; let x = …;` |
|----------------|-------------------------|-------------------------|
| binding        | same                    | new                     |
| type           | must stay the same      | can change              |
| old value      | overwritten             | dropped                 |
| signals reader | "this changes"          | "next step"             |

---

## Common mistake

```rust
let mut s = "hello";
s = s.len();
// ERROR: expected &str, found integer
```

`mut` does NOT let the type change. Use shadowing:

```rust
let s = "hello";
let s = s.len();   // s is now usize
```

Note: this is exactly what the lesson's compile-fail exercise drives home — try it before reading the solution.

---

## `const`

```rust
const MAX_RETRIES: u32 = 3;
const HUNDRED: f64 = 100.0;
```

Three rules:

1. Type **must** be annotated — no inference
2. Value must be a compile-time constant
3. Convention: `SCREAMING_SNAKE_CASE`

`const` is the right choice for values known at compile time that never change.

---

## Wrap

- Immutability is the default
- `mut` is a local, opt-in escape hatch
- Shadowing is a different tool for a different job
- `const` for true compile-time constants

Next: Lesson 03 — control flow & functions.
