# Lesson 03 — Control flow & functions

In Rust, control flow doesn't just *do* things — it *evaluates* to
things. `if` is an expression. A function's body is a block whose tail
expression is its return value. Once you see those two together, the
rest of the lesson's syntax (the three loops, `break`/`continue`,
statements ending in `;`) clicks into place.

## Learning goals

- Use `if` as an expression and read its result
- Choose between `for`, `while`, and `loop` for different iteration shapes
- Use `break` and `continue` to alter loop control
- Explain the difference between statements (which produce `()`) and
  expressions (which produce values)
- Return values from functions by tail expression and by explicit `return`

## Self-study notes

### `if` as an expression

In Rust, `if`/`else` is not just a control flow statement — it produces
a value. You can use it on the right-hand side of `let`:

```rust
let max = if a > b { a } else { b };
```

Chained with `else if`, it works the same way:

```rust
let label = if n < 0 {
    "negative"
} else if n == 0 {
    "zero"
} else {
    "positive"
};
```

The two key things to notice: each branch's last line has **no trailing
semicolon** (so it's an expression, not a statement), and every branch
must produce the same type (here, all three branches produce
`&'static str`).

### The three loops

Rust has three loop forms:

```rust
// for ... in range — the most common
for i in 1..=10 {
    println!("{i}");
}

// while — condition-controlled
let mut n = 1234;
while n > 0 {
    n /= 10;
}

// loop — unconditional, paired with `break`
let mut tries = 0;
loop {
    tries += 1;
    if tries == 3 {
        break;
    }
}
```

Choose by shape:

- `for` when you know the range or sequence up front.
- `while` when continuation depends on state that changes inside the loop.
- `loop` when the exit condition is naturally expressed mid-body.

The `..` and `..=` operators build ranges: `..` is exclusive, `..=` is
inclusive.

### `break` and `continue`

`break` exits the current loop immediately. `continue` skips to the next
iteration. Both work in all three loop forms.

```rust
for i in 0..100 {
    if i == 10 {
        break;        // stop the loop
    }
    if i % 2 == 0 {
        continue;     // skip the rest of this iteration
    }
    // body runs only for odd i below 10
}
```

### Expressions vs statements — and the unit type

Rust has two kinds of code: **statements** and **expressions**.

- A **statement** does something but produces no value. It ends with `;`
  and its "type" is `()` — the unit type, an empty tuple.
- An **expression** produces a value. Its type is the type of that value.

```rust
let x = 5;     // statement: produces ()
let y = 5;     // statement: produces ()
x + y          // expression: produces 10 (of type i32)
x + y;         // statement: throws away the value, produces ()
```

`if`, `loop`, `{ block }`, function calls, and most everything else in
Rust are expressions. A trailing semicolon is not just punctuation — it
changes what your code returns. This is the single most important
syntactic insight in Rust.

### Functions: tail returns and explicit `return`

A function's body is a block. The block's **tail expression** — the last
expression with no trailing semicolon — is the function's return value.

```rust
fn double(n: i32) -> i32 {
    n * 2     // tail expression — this is what `double` returns
}
```

You can also use the `return` keyword to return early. `return` IS a
statement, so it needs a `;`:

```rust
fn first_positive(a: i32, b: i32) -> i32 {
    if a > 0 {
        return a;   // early return
    }
    b               // tail expression — used when a <= 0
}
```

Style: prefer tail expressions for the normal path; use `return` for
early exits. Rust accepts `return 5;` instead of `5` at the tail — but
most Rust code follows the convention of dropping the `return` when you
can.

## Exercises

### Warm-up: `classify`

Implement `classify(n: i32) -> &'static str` in `exercises/src/lib.rs`
so it returns:

- `"negative"` if `n < 0`
- `"zero"` if `n == 0`
- `"positive"` if `n > 0`

Use an if-expression as the function's tail. The whole function body
should be a single `if`/`else if`/`else` chain with no `return` keyword.

### Main: `count_digits`

Implement `count_digits(n: u32) -> u32` returning the number of decimal
digits in `n`. Treat `0` as having 1 digit.

You'll likely want:

- An early `return 1;` for the `n == 0` edge case
- A `while` loop that divides `n` by 10 repeatedly
- A `let mut count: u32 = 0;` accumulator
- A tail expression that returns `count`

### Compile-fail

`exercises/compile_fails/03-trailing-semicolon.rs` ships in a state that
does **not** compile. The function `double` declares it returns `i32`
but its body has a trailing `;` that turns the expression into a
statement returning `()`. Read the rustc error, then fix it — the fix
is removing one character.

### Run

```bash
make verify LESSON=03-control-flow
```

This runs your exercise tests and asserts the compile-fail file now
compiles.

## Solutions

See `solutions/src/lib.rs` for the reference implementations. Try the
exercises before peeking.
