# Lesson 02 — Variables, types, mutability

In Rust the default is no mutation. The type system makes that practical.
By the end of this lesson you'll know how to declare bindings, how to opt
into mutation when you need it, how to read inferred types and add
annotations, and how shadowing differs from mutation in subtle but
important ways.

## Learning goals

- Declare bindings with `let` and explain why they're immutable by default
- Opt into mutation with `let mut` and explain when it's appropriate
- Read Rust's inferred types and add explicit annotations when needed
- Re-bind a name using shadowing, and explain why it's different from mutation
- Declare compile-time constants with `const`

## Self-study notes

### The default is immutable

`let` creates a binding from a name to a value. Once made, that binding
cannot be reassigned:

```rust
let x = 5;
x = 6; // error: cannot assign twice to immutable variable `x`
```

You met this error in Lesson 01. Rust is telling you that the binding
`x` is immutable, which is its default. Most bindings in a typical Rust
program are immutable — and that's by design. Immutability removes a
whole class of bugs and makes code easier to read: you can be confident
that a variable's value doesn't change underneath you.

### Opting into mutation with `mut`

When you genuinely need a binding to change, opt in with `mut`:

```rust
let mut counter = 0;
counter = counter + 1;
counter = counter + 1;
```

The `mut` keyword is a deliberate, local signal to future readers (often
your future self): *"yes, this thing changes."* If you don't see `mut`,
you can trust that the binding holds the same value its whole life.

### Type inference and annotations

Rust infers the type of a binding from the value:

```rust
let x = 5;       // inferred: i32 (the default integer type)
let y = 5.0;     // inferred: f64
let ok = true;   // inferred: bool
```

Sometimes you want to be explicit, or Rust can't infer (we'll see one in
a few lessons when we call `Vec::new()`). Add the type after the name:

```rust
let x: u32 = 5;
let pi: f64 = 3.14159;
```

Annotations also enforce. If you try `let x: u32 = -1;`, the compiler
refuses — `-1` isn't a valid `u32`.

> **Aside on dot-syntax.** Numbers have methods. You'll see `factor.powi(2)`
> in this lesson's main exercise — `.powi` is a method on `f64`. We'll
> cover methods properly with structs in Lesson 06.

### Shadowing

You can declare a binding with a name that already exists. The new
binding takes over; the old one is gone:

```rust
let x = 5;
let x = x + 1;   // a new `x`, value 6
let x = "five";  // a new `x`, type &str
```

This is NOT mutation. Each `let` makes a fresh binding. The compiler
treats them as separate variables that happen to share a name. The old
`x` is dropped when the new one shadows it.

### `mut` vs shadowing — when to use which

Both let you "change" what `x` refers to. They're different tools for
different jobs:

| aspect       | `let mut x = ...; x = ...;` | `let x = ...; let x = ...;` |
|--------------|-----------------------------|-----------------------------|
| binding      | same binding                | new binding                 |
| type         | must stay the same          | can change                  |
| old value    | overwritten                 | dropped                     |
| signals to reader | "this thing changes"   | "I'm done with that; here's the next step" |

A common mistake is reaching for `mut` when the situation actually
wants shadowing — typically when you're transforming a value through a
few steps:

```rust
// Awkward — needs a different type at each step, but mut forces same-type:
let mut s = "hello";
// s = s.len();  // ERROR: cannot assign integer to &str binding

// Natural — each step is a fresh binding via shadowing:
let s = "hello";
let s = s.len();
```

This lesson's compile-fail exercise drives this misconception home with
the compiler's own diagnostic.

### `const`

For values known at compile time that never change, use `const`:

```rust
const MAX_RETRIES: u32 = 3;
const HUNDRED: f64 = 100.0;
```

Three rules to remember about `const`:

1. The type **must** be annotated — `const` doesn't infer.
2. The value must be a compile-time constant expression.
3. Convention is `SCREAMING_SNAKE_CASE`.

## Exercises

### Warm-up: `fahrenheit_to_celsius`

Open `exercises/src/lib.rs` and implement `fahrenheit_to_celsius` using
the formula `(F − 32) × 5 / 9`.

### Main: `compound_interest`

In the same file, implement `compound_interest(principal, rate_percent, years)`
returning `principal × (1 + rate_percent/100)^years`. You'll need
`f64::powi`. Hints in the slide aside.

### Compile-fail

`exercises/compile_fails/02-mut-cant-change-type.rs` ships in a state
that does **not** compile. Read the comment in the file, then fix it.
The fix is one keyword change on one line.

### Run

```bash
make verify LESSON=02-variables
```

This runs your exercise tests and then asserts the compile-fail file now
compiles.

## Solutions

See `solutions/src/lib.rs` for the reference implementations. Try the
exercises before peeking.
