# Control flow & functions

> In Rust, control flow doesn't just *do* things — it *evaluates* to things.

---

## Recap

Lesson 02 introduced `let mut`.

Today: use it inside loops, and meet the deeper reason for some of Rust's syntax — **control-flow constructs are expressions**.

---

## `if` as an expression

```rust
let max = if a > b { a } else { b };
```

It's not just a control flow statement — it's an expression that has a value.

Chained:

```rust
let label = if n < 0 {
    "negative"
} else if n == 0 {
    "zero"
} else {
    "positive"
};
```

Each branch is an expression with no trailing semicolon. The whole `if` has the value of whichever branch ran.

---

## `for ... in range`

The most common Rust loop:

```rust
let mut sum = 0;
for i in 1..=10 {
    sum += i;
}
// sum == 55
```

- `..` is exclusive: `1..10` covers 1 through 9
- `..=` is inclusive: `1..=10` covers 1 through 10

Use `for` when you know the iteration shape up front.

---

## `while`

Loop while a condition is true:

```rust
let mut n = 1234;
while n > 0 {
    n /= 10;  // 123, 12, 1, 0 -> stops
}
```

Use `while` when continuation depends on state that changes inside the loop.

---

## `loop`

Unconditional loop. Paired with `break`:

```rust
let mut tries = 0;
loop {
    tries += 1;
    if tries == 3 {
        break;
    }
}
```

Note: `loop` is also an expression and can return a value via `break value;`. Useful for retry-until-success patterns. More in Lesson 16.

Use `loop` when the exit condition is naturally expressed mid-body.

---

## `break` and `continue`

`break` exits the loop immediately:

```rust
for i in 0..100 {
    if i == 10 {
        break;
    }
}
```

`continue` skips to the next iteration:

```rust
for i in 0..10 {
    if i % 2 == 0 {
        continue;
    }
    // body runs only for odd i
}
```

---

## Expressions vs statements

Two kinds of code in Rust:

- **Statements** end with `;` and produce the unit type `()`
- **Expressions** produce a value

```rust
let x = 5;     // statement
let y = 5;     // statement
x + y          // expression, value 10
x + y;         // statement, value () — the semicolon discards the result
```

`if`, `loop`, `{ block }`, and most everything else are expressions.

A trailing semicolon is not whitespace — it changes what your code returns.

---

## Functions: tail returns

A function's body is a block. The block's tail expression — the last expression with no trailing semicolon — IS the function's return value.

```rust
fn double(n: i32) -> i32 {
    n * 2     // tail expression, no semicolon
}
```

`return` works too, but is a statement (and needs `;`):

```rust
fn first_positive(a: i32, b: i32) -> i32 {
    if a > 0 {
        return a;   // early return
    }
    b              // tail expression
}
```

Note: prefer tail expressions for the normal path; use `return` for early exits.

---

## Wrap

- `if` is an expression — use it as a value
- `for` / `while` / `loop` for different iteration shapes
- Statements produce `()`; expressions produce values
- Functions return their tail expression

Next: Lesson 04 — compound types (tuples, arrays, slices, String vs &str).
