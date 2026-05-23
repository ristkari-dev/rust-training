# Lesson 03 — Control flow & functions — Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Author the third lesson of the Rust training course: a warm-up + main exercise pair on `if`-as-expression, three loop constructs (`for`/`while`/`loop`), `break`/`continue`, the expressions-vs-statements distinction, and tail-expression returns. Plus one compile-fail exercise on the trailing-semicolon-makes-statement-not-expression misconception.

**Architecture:** Use the existing `make new-lesson` scaffolder to lay down the four-part lesson structure, then overwrite the placeholder content with lesson-specific README, slides, exercises, and solutions per the design spec. The workspace `members` glob picks the new crates up automatically.

**Tech Stack:** Rust 2024 edition, existing tools (`new-lesson`, `compile-fails`, `slides-dev`, `build-index`), reveal.js (vendored), GNU Make.

**Spec:** [`docs/superpowers/specs/2026-05-23-lesson-03-control-flow-design.md`](../specs/2026-05-23-lesson-03-control-flow-design.md).

**Working directory:** `/Users/ristkari/code/private/rust-training`.

---

## Task 1: Scaffold lessons/03-control-flow

Use the existing tool to create the lesson skeleton from `templates/`.

**Files (all created by the scaffolder):**
- `lessons/03-control-flow/README.md` (placeholder, replaced in Task 4)
- `lessons/03-control-flow/slides/index.html` (final — no edit needed)
- `lessons/03-control-flow/slides/slides.md` (placeholder, replaced in Task 5)
- `lessons/03-control-flow/exercises/Cargo.toml` (final — no edit needed)
- `lessons/03-control-flow/exercises/src/lib.rs` (placeholder, replaced in Task 2)
- `lessons/03-control-flow/exercises/tests/exercise.rs` (placeholder, replaced in Task 2)
- `lessons/03-control-flow/solutions/Cargo.toml` (final — no edit needed)
- `lessons/03-control-flow/solutions/src/lib.rs` (placeholder, replaced in Task 3)
- `lessons/03-control-flow/solutions/tests/exercise.rs` (placeholder, replaced in Task 3)

- [ ] **Step 1: Run the scaffolder**

```bash
make new-lesson NAME=03-control-flow
```

Expected: `scaffolded ./lessons/03-control-flow`.

- [ ] **Step 2: Verify directory structure**

```bash
ls lessons/03-control-flow/
ls lessons/03-control-flow/slides/ lessons/03-control-flow/exercises/ lessons/03-control-flow/solutions/
```

Expected: top-level `README.md`, `slides/`, `exercises/`, `solutions/`. Each subdirectory populated from templates.

- [ ] **Step 3: Verify Cargo package names**

```bash
grep '^name' lessons/03-control-flow/exercises/Cargo.toml lessons/03-control-flow/solutions/Cargo.toml
```

Expected:
```
lessons/03-control-flow/exercises/Cargo.toml:name = "control-flow-exercises"
lessons/03-control-flow/solutions/Cargo.toml:name = "control-flow-solutions"
```

- [ ] **Step 4: Verify the workspace picks up the new crates**

```bash
cargo metadata --no-deps --format-version 1 | grep -o '"name":"control-flow-[^"]*"' | sort -u
```

Expected output:
```
"name":"control-flow-exercises"
"name":"control-flow-solutions"
```

- [ ] **Step 5: Verify the scaffolded workspace builds clean**

```bash
cargo build --workspace
```

Expected: warning-free build.

- [ ] **Step 6: Commit the scaffold**

```bash
git add lessons/03-control-flow
git commit -m "chore: scaffold lessons/03-control-flow"
```

---

## Task 2: Exercise content (stubs + tests + compile-fail)

**Files:**
- Overwrite: `lessons/03-control-flow/exercises/src/lib.rs`
- Overwrite: `lessons/03-control-flow/exercises/tests/exercise.rs`
- Create: `lessons/03-control-flow/exercises/compile_fails/03-trailing-semicolon.rs`

- [ ] **Step 1: Overwrite `lessons/03-control-flow/exercises/src/lib.rs`**

```rust
//! Lesson 03 — exercises.
//!
//! Implement `classify` (warm-up) and `count_digits` (main) so that
//! `cargo test --manifest-path lessons/03-control-flow/exercises/Cargo.toml`
//! passes. The tests live in `tests/exercise.rs`.

#[must_use]
pub fn classify(_n: i32) -> &'static str {
    todo!("return \"negative\" / \"zero\" / \"positive\" using an if-as-expression")
}

#[must_use]
pub fn count_digits(_n: u32) -> u32 {
    todo!("count the number of decimal digits; treat 0 as having 1 digit")
}
```

- [ ] **Step 2: Overwrite `lessons/03-control-flow/exercises/tests/exercise.rs`**

```rust
use control_flow_exercises::{classify, count_digits};

// Warm-up: classify

#[test]
fn classify_negative() {
    assert_eq!(classify(-42), "negative");
}

#[test]
fn classify_zero() {
    assert_eq!(classify(0), "zero");
}

#[test]
fn classify_positive() {
    assert_eq!(classify(7), "positive");
}

// Main: count_digits

#[test]
fn count_digits_zero() {
    assert_eq!(count_digits(0), 1);
}

#[test]
fn count_digits_single() {
    assert_eq!(count_digits(7), 1);
}

#[test]
fn count_digits_two() {
    assert_eq!(count_digits(42), 2);
}

#[test]
fn count_digits_three() {
    assert_eq!(count_digits(100), 3);
}

#[test]
fn count_digits_u32_max() {
    assert_eq!(count_digits(4_294_967_295), 10);
}
```

- [ ] **Step 3: Create `lessons/03-control-flow/exercises/compile_fails/03-trailing-semicolon.rs`**

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

- [ ] **Step 4: Verify exercise tests fail with `todo!()` panic (intentional)**

```bash
cargo test --manifest-path lessons/03-control-flow/exercises/Cargo.toml
```

Expected: all 8 tests fail with `not yet implemented` panic message. This is the intentional shipped state.

- [ ] **Step 5: Verify the exercises crate still builds cleanly**

```bash
cargo build --package control-flow-exercises
```

Expected: warning-free build (the `#[must_use]` attribute and the `_`-prefixed parameter names keep clippy happy).

- [ ] **Step 6: Verify compile-fail ships broken (the author/CI check)**

```bash
cargo run --package compile-fails -- --expect broken lessons/03-control-flow
```

Expected: `ok: lessons/03-control-flow/exercises/compile_fails/03-trailing-semicolon.rs` and exit 0.

- [ ] **Step 7: Verify compile-fail's student-mode check fires (the exercise hasn't been fixed yet)**

```bash
cargo run --package compile-fails -- --expect compiles lessons/03-control-flow
```

Expected: non-zero exit with `FAIL: file did not compile, but was expected to: lessons/03-control-flow/...`.

- [ ] **Step 8: Verify lint passes on the exercises crate**

```bash
cargo clippy --package control-flow-exercises --all-targets -- -D warnings
cargo fmt --check --package control-flow-exercises
```

Expected: both exit 0.

- [ ] **Step 9: Commit**

```bash
git add lessons/03-control-flow/exercises
git commit -m "feat(lesson-03): add warm-up + main exercise stubs, tests, and compile-fail"
```

---

## Task 3: Reference solutions

**Files:**
- Overwrite: `lessons/03-control-flow/solutions/src/lib.rs`
- Overwrite: `lessons/03-control-flow/solutions/tests/exercise.rs`

- [ ] **Step 1: Overwrite `lessons/03-control-flow/solutions/src/lib.rs`**

```rust
//! Lesson 03 — reference solutions.

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

> Pedagogical notes baked into the solution:
>
> - `classify` returns the `if`-expression directly as the function's tail. No `return`, no intermediate `let`. This is the canonical "if as expression" pattern.
> - `count_digits` uses an explicit `return` for the zero edge case (demonstrating `return` as a statement), then two `let mut` bindings, a `while` loop with mutation, and a tail-expression return — all five of the lesson's syntactic concepts in one ~10-line function.
>
> Unlike Lesson 02, no `#[allow]` attributes are needed: integer-only arithmetic and string-slice returns don't trigger pedantic clippy. If clippy surprises you, fix the code rather than allow-listing.

- [ ] **Step 2: Overwrite `lessons/03-control-flow/solutions/tests/exercise.rs`**

```rust
use control_flow_solutions::{classify, count_digits};

// Warm-up: classify

#[test]
fn classify_negative() {
    assert_eq!(classify(-42), "negative");
}

#[test]
fn classify_zero() {
    assert_eq!(classify(0), "zero");
}

#[test]
fn classify_positive() {
    assert_eq!(classify(7), "positive");
}

// Main: count_digits

#[test]
fn count_digits_zero() {
    assert_eq!(count_digits(0), 1);
}

#[test]
fn count_digits_single() {
    assert_eq!(count_digits(7), 1);
}

#[test]
fn count_digits_two() {
    assert_eq!(count_digits(42), 2);
}

#[test]
fn count_digits_three() {
    assert_eq!(count_digits(100), 3);
}

#[test]
fn count_digits_u32_max() {
    assert_eq!(count_digits(4_294_967_295), 10);
}
```

- [ ] **Step 3: Verify solution tests pass**

```bash
cargo test --package control-flow-solutions
```

Expected: 8 tests pass.

- [ ] **Step 4: Verify lint passes on the solutions crate**

```bash
cargo clippy --package control-flow-solutions --all-targets -- -D warnings
cargo fmt --check --package control-flow-solutions
```

Expected: both exit 0.

- [ ] **Step 5: Commit**

```bash
git add lessons/03-control-flow/solutions
git commit -m "feat(lesson-03): add reference solutions"
```

---

## Task 4: Lesson README

**Files:**
- Overwrite: `lessons/03-control-flow/README.md`

- [ ] **Step 1: Overwrite `lessons/03-control-flow/README.md`**

The complete file content (write this as the file's actual content, starting with the `# Lesson 03` heading). Code fences inside the markdown are plain triple-backticks:

```markdown
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

​```rust
let max = if a > b { a } else { b };
​```

Chained with `else if`, it works the same way:

​```rust
let label = if n < 0 {
    "negative"
} else if n == 0 {
    "zero"
} else {
    "positive"
};
​```

The two key things to notice: each branch's last line has **no trailing
semicolon** (so it's an expression, not a statement), and every branch
must produce the same type (here, all three branches produce
`&'static str`).

### The three loops

Rust has three loop forms:

​```rust
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
​```

Choose by shape:

- `for` when you know the range or sequence up front.
- `while` when continuation depends on state that changes inside the loop.
- `loop` when the exit condition is naturally expressed mid-body.

The `..` and `..=` operators build ranges: `..` is exclusive, `..=` is
inclusive.

### `break` and `continue`

`break` exits the current loop immediately. `continue` skips to the next
iteration. Both work in all three loop forms.

​```rust
for i in 0..100 {
    if i == 10 {
        break;        // stop the loop
    }
    if i % 2 == 0 {
        continue;     // skip the rest of this iteration
    }
    // body runs only for odd i below 10
}
​```

### Expressions vs statements — and the unit type

Rust has two kinds of code: **statements** and **expressions**.

- A **statement** does something but produces no value. It ends with `;`
  and its "type" is `()` — the unit type, an empty tuple.
- An **expression** produces a value. Its type is the type of that value.

​```rust
let x = 5;     // statement: produces ()
let y = 5;     // statement: produces ()
x + y          // expression: produces 10 (of type i32)
x + y;         // statement: throws away the value, produces ()
​```

`if`, `loop`, `{ block }`, function calls, and most everything else in
Rust are expressions. A trailing semicolon is not just punctuation — it
changes what your code returns. This is the single most important
syntactic insight in Rust.

### Functions: tail returns and explicit `return`

A function's body is a block. The block's **tail expression** — the last
expression with no trailing semicolon — is the function's return value.

​```rust
fn double(n: i32) -> i32 {
    n * 2     // tail expression — this is what `double` returns
}
​```

You can also use the `return` keyword to return early. `return` IS a
statement, so it needs a `;`:

​```rust
fn first_positive(a: i32, b: i32) -> i32 {
    if a > 0 {
        return a;   // early return
    }
    b               // tail expression — used when a <= 0
}
​```

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

​```bash
make verify LESSON=03-control-flow
​```

This runs your exercise tests and asserts the compile-fail file now
compiles.

## Solutions

See `solutions/src/lib.rs` for the reference implementations. Try the
exercises before peeking.
```

> **IMPORTANT for the implementer:** the code-fence escape in the
> markdown above uses an invisible zero-width character (shown as `​```)
> in front of each triple-backtick BLOCK — that's only there so this
> plan file can nest fenced markdown inside an outer fenced markdown
> block. When you write the actual `README.md`, every fence must be
> three PLAIN backticks `` ``` `` with NO leading invisible character.
> The actual file should contain seven Rust code blocks plus one bash
> code block = 8 code blocks = 16 fence lines.

- [ ] **Step 2: Spot-check the README**

```bash
head -1 lessons/03-control-flow/README.md
grep -c '^### ' lessons/03-control-flow/README.md
grep -c '^```' lessons/03-control-flow/README.md
```

Expected:
- First line: `# Lesson 03 — Control flow & functions`
- `grep -c '^### '` returns 9 (five subsections under self-study + four under exercises: Warm-up, Main, Compile-fail, Run)
- `grep -c '^```'` returns 16 (8 code blocks × 2 fence lines)

- [ ] **Step 3: Commit**

```bash
git add lessons/03-control-flow/README.md
git commit -m "docs(lesson-03): write self-study notes"
```

---

## Task 5: Slide deck

**Files:**
- Overwrite: `lessons/03-control-flow/slides/slides.md`

- [ ] **Step 1: Overwrite `lessons/03-control-flow/slides/slides.md`**

The complete file content (write this as the file's actual content, starting with the `# Control flow & functions` heading). Code fences inside this content are plain triple-backticks:

````
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
````

The OUTER fence above (quadruple backticks) is just delimiting this content in the plan. The FILE should NOT contain the outer fence — only the inner triple-backtick `rust` blocks.

- [ ] **Step 2: Verify `make slides-build` succeeds and includes lesson 03**

```bash
make slides-build
test -f dist/lessons/03-control-flow/slides/slides.md
test -f dist/lessons/03-control-flow/slides/index.html
grep -c "03-control-flow" dist/index.html
```

Expected: `slides.md` and `index.html` copied into dist; `grep -c "03-control-flow"` returns at least 1 (the published lesson link in the landing page).

- [ ] **Step 3: Spot-check slide separators**

```bash
grep -c '^---$' lessons/03-control-flow/slides/slides.md
```

Expected: `9` (between 10 slides).

- [ ] **Step 4: Commit**

```bash
git add lessons/03-control-flow/slides/slides.md
git commit -m "feat(lesson-03): write slide deck"
```

---

## Task 6: End-to-end verification + push

- [ ] **Step 1: `make ci` is green**

```bash
make ci
```

Expected: exit 0. Clippy clean, fmt clean, workspace builds, default-members tests pass (now includes the 8 new tests in `control-flow-solutions`), compile-fail `--expect broken` passes for lesson 03.

- [ ] **Step 2: `make verify LESSON=03-control-flow` fails (the exercise is undone — intentional)**

```bash
make verify LESSON=03-control-flow || echo "expected: exercise tests fail with todo!() panic"
```

Expected: non-zero exit. All 8 exercise tests panic with `not yet implemented`. The student fixing the exercise is what makes this pass.

- [ ] **Step 3: `make slides-build` final state**

```bash
make slides-build
ls dist/lessons/
grep -c "03-control-flow" dist/index.html
```

Expected: `dist/lessons/` contains `01-hello-rust`, `02-variables`, and `03-control-flow`. `grep -c "03-control-flow"` ≥ 1.

- [ ] **Step 4: Push**

```bash
git push
```

Expected: push succeeds. CI workflow runs and is green; deploy workflow runs and rebuilds the static site so lesson 03 appears live at `rust.ristkari.dev/lessons/03-control-flow/slides/`.

- [ ] **Step 5: Smoke-test the deployed site**

```bash
curl -sS -o /dev/null -w "%{http_code}\n" https://rust.ristkari.dev/
curl -sS -o /dev/null -w "%{http_code}\n" https://rust.ristkari.dev/lessons/03-control-flow/slides/
```

Expected: both return `200`.

---

## Done criteria

- `lessons/03-control-flow/` exists with all four parts
- `cargo test --package control-flow-solutions` → 8 passing tests
- `cargo test --manifest-path lessons/03-control-flow/exercises/Cargo.toml` → 8 panicking tests (intentional)
- `cargo run --package compile-fails -- --expect broken lessons/03-control-flow` → ok
- `cargo run --package compile-fails -- --expect compiles lessons/03-control-flow` → fails (intentional)
- `make ci` → green
- `make slides-build` → produces `dist/lessons/03-control-flow/slides/index.html`
- `dist/index.html` lists lesson 03 as a clickable link
- All changes committed and pushed
- Deployed site returns HTTP 200 for `/` and `/lessons/03-control-flow/slides/`
