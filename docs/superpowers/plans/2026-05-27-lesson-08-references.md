# Lesson 08 — References & borrowing — Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Author the second lesson of Phase 2 of the Rust training course: a warm-up + main exercise pair on references and borrowing. Warm-up is `wrap_in_quotes` (borrow to read, return owned); main is `merge_into` (one mut reference + two shared, demonstrates the borrowing rules in a single signature). Compile-fail is the canonical "mut + shared simultaneously" borrow-checker error.

**Architecture:** Use the existing `make new-lesson` scaffolder to lay down the four-part lesson structure, then overwrite the placeholder content with lesson-specific README, slides, exercises, and solutions per the design spec.

**Tech Stack:** Rust 2024 edition, existing tools (`new-lesson`, `compile-fails`, `slides-dev`, `build-index`), reveal.js (vendored), GNU Make.

**Spec:** [`docs/superpowers/specs/2026-05-27-lesson-08-references-design.md`](../specs/2026-05-27-lesson-08-references-design.md).

**Working directory:** `/Users/ristkari/code/private/rust-training`.

---

## Task 1: Scaffold lessons/08-references

**Files (all created by the scaffolder):**
- `lessons/08-references/README.md` (placeholder, replaced in Task 4)
- `lessons/08-references/slides/index.html` (final — no edit needed)
- `lessons/08-references/slides/slides.md` (placeholder, replaced in Task 5)
- `lessons/08-references/exercises/Cargo.toml` (final — no edit needed)
- `lessons/08-references/exercises/src/lib.rs` (placeholder, replaced in Task 2)
- `lessons/08-references/exercises/tests/exercise.rs` (placeholder, replaced in Task 2)
- `lessons/08-references/solutions/Cargo.toml` (final — no edit needed)
- `lessons/08-references/solutions/src/lib.rs` (placeholder, replaced in Task 3)
- `lessons/08-references/solutions/tests/exercise.rs` (placeholder, replaced in Task 3)

- [ ] **Step 1: Run the scaffolder**

```bash
make new-lesson NAME=08-references
```

Expected: `scaffolded ./lessons/08-references`.

- [ ] **Step 2: Verify directory structure**

```bash
ls lessons/08-references/
ls lessons/08-references/slides/ lessons/08-references/exercises/ lessons/08-references/solutions/
```

Expected: top-level `README.md`, `slides/`, `exercises/`, `solutions/` populated from templates.

- [ ] **Step 3: Verify Cargo package names**

```bash
grep '^name' lessons/08-references/exercises/Cargo.toml lessons/08-references/solutions/Cargo.toml
```

Expected:
```
lessons/08-references/exercises/Cargo.toml:name = "references-exercises"
lessons/08-references/solutions/Cargo.toml:name = "references-solutions"
```

- [ ] **Step 4: Verify the workspace picks up the new crates**

```bash
cargo metadata --no-deps --format-version 1 | grep -o '"name":"references-[^"]*"' | sort -u
```

Expected output:
```
"name":"references-exercises"
"name":"references-solutions"
```

- [ ] **Step 5: Verify the scaffolded workspace builds clean**

```bash
cargo build --workspace
```

Expected: warning-free build.

- [ ] **Step 6: Commit the scaffold**

```bash
git add lessons/08-references
git commit -m "chore: scaffold lessons/08-references"
```

---

## Task 2: Exercise content (stubs + tests + compile-fail)

**Files:**
- Overwrite: `lessons/08-references/exercises/src/lib.rs`
- Overwrite: `lessons/08-references/exercises/tests/exercise.rs`
- Create: `lessons/08-references/exercises/compile_fails/08-mut-and-shared.rs`

- [ ] **Step 1: Overwrite `lessons/08-references/exercises/src/lib.rs`**

```rust
//! Lesson 08 — exercises.
//!
//! Implement `wrap_in_quotes` (warm-up) and `merge_into` (main) so
//! that `cargo test --manifest-path
//! lessons/08-references/exercises/Cargo.toml` passes. The tests live
//! in `tests/exercise.rs`.

#[must_use]
pub fn wrap_in_quotes(_s: &str) -> String {
    todo!("return a String containing s wrapped in double quotes, e.g. wrap_in_quotes(\"hi\") -> \"\\\"hi\\\"\"")
}

pub fn merge_into(_target: &mut String, _parts: &[&str], _separator: &str) {
    todo!("append parts joined by separator to target; do nothing if parts is empty")
}
```

- [ ] **Step 2: Overwrite `lessons/08-references/exercises/tests/exercise.rs`**

```rust
use references_exercises::{merge_into, wrap_in_quotes};

// Warm-up: wrap_in_quotes

#[test]
fn warmup_typical() {
    assert_eq!(wrap_in_quotes("hello"), "\"hello\"");
}

#[test]
fn warmup_empty() {
    assert_eq!(wrap_in_quotes(""), "\"\"");
}

#[test]
fn warmup_with_spaces() {
    assert_eq!(wrap_in_quotes("hello world"), "\"hello world\"");
}

#[test]
fn warmup_with_inner_quote() {
    assert_eq!(wrap_in_quotes("she said \"hi\""), "\"she said \"hi\"\"");
}

// Main: merge_into

#[test]
fn main_typical() {
    let mut target = String::from("items: ");
    merge_into(&mut target, &["a", "b", "c"], "-");
    assert_eq!(target, "items: a-b-c");
}

#[test]
fn main_empty_parts_no_change() {
    let mut target = String::from("unchanged");
    merge_into(&mut target, &[], "-");
    assert_eq!(target, "unchanged");
}

#[test]
fn main_single_part_no_separator() {
    let mut target = String::new();
    merge_into(&mut target, &["solo"], "-");
    assert_eq!(target, "solo");
}

#[test]
fn main_multi_char_separator() {
    let mut target = String::new();
    merge_into(&mut target, &["one", "two", "three"], ", ");
    assert_eq!(target, "one, two, three");
}
```

- [ ] **Step 3: Create `lessons/08-references/exercises/compile_fails/08-mut-and-shared.rs`**

```rust
// Compile-fail exercise: this file MUST NOT compile until you fix it.
//
// Rust's borrowing rules: at any moment, for any value, you can have
// either
//   - any number of SHARED references (`&T`), OR
//   - exactly one MUTABLE reference (`&mut T`).
//
// You CANNOT have a mutable reference and shared references to the
// same value at the same time. The borrow checker enforces this — it
// prevents data races by construction.
//
// The function below tries to take a `&mut s` while a shared `&s` is
// still in use. rustc will say "cannot borrow `s` as mutable because
// it is also borrowed as immutable."
//
// Hint: read the rustc error. The fix is to drop the shared borrow
// before taking the mutable one — for example, by moving the
// `println!` of `r1` to BEFORE the line that takes `&mut s`. Once
// `r1` is no longer used, its borrow ends, and the mutable borrow
// becomes legal.

fn main() {
    let mut s = String::from("hello");
    let r1 = &s;
    let r2 = &mut s;
    r2.push_str(" world");
    println!("{r1}");
}
```

- [ ] **Step 4: Verify exercise tests fail with `todo!()` panic (intentional)**

```bash
cargo test --manifest-path lessons/08-references/exercises/Cargo.toml
```

Expected: all 8 tests fail with `not yet implemented` panic message.

- [ ] **Step 5: Verify the exercises crate still builds cleanly**

```bash
cargo build --package references-exercises
```

Expected: warning-free build.

- [ ] **Step 6: Verify compile-fail ships broken**

```bash
cargo run --package compile-fails -- --expect broken lessons/08-references
```

Expected: `ok: lessons/08-references/exercises/compile_fails/08-mut-and-shared.rs` and exit 0.

- [ ] **Step 7: Verify compile-fail's student-mode check fires**

```bash
cargo run --package compile-fails -- --expect compiles lessons/08-references
```

Expected: non-zero exit with `FAIL: file did not compile, but was expected to: lessons/08-references/...`.

- [ ] **Step 8: Verify lint passes on the exercises crate**

```bash
cargo clippy --package references-exercises --all-targets -- -D warnings
cargo fmt --check --package references-exercises
```

Expected: both exit 0.

- [ ] **Step 9: Commit**

```bash
git add lessons/08-references/exercises
git commit -m "feat(lesson-08): add warm-up + main exercise stubs, tests, and compile-fail"
```

---

## Task 3: Reference solutions

**Files:**
- Overwrite: `lessons/08-references/solutions/src/lib.rs`
- Overwrite: `lessons/08-references/solutions/tests/exercise.rs`

- [ ] **Step 1: Overwrite `lessons/08-references/solutions/src/lib.rs`**

```rust
//! Lesson 08 — reference solutions.

#[must_use]
pub fn wrap_in_quotes(s: &str) -> String {
    let mut result = String::from("\"");
    result.push_str(s);
    result.push('"');
    result
}

pub fn merge_into(target: &mut String, parts: &[&str], separator: &str) {
    let mut first = true;
    for part in parts {
        if !first {
            target.push_str(separator);
        }
        target.push_str(part);
        first = false;
    }
}
```

> Pedagogical notes baked into the solutions:
> - `wrap_in_quotes` borrows `s` to read (`push_str(s)`), then returns a freshly-built owned `String`. The canonical "borrow to read, return new owned data" pattern.
> - `merge_into` has the climactic L08 signature: one `&mut String`, one `&[&str]`, one `&str` — three live references, demonstrating that the "one mut OR many shared" rule is per-value, not per-function.
> - The `first: bool` flag pattern reuses the technique from L04's `join_with_dashes`.
>
> No `#[allow]` attributes should be needed. If clippy fires, fix the code rather than allow-listing.

- [ ] **Step 2: Overwrite `lessons/08-references/solutions/tests/exercise.rs`**

```rust
use references_solutions::{merge_into, wrap_in_quotes};

// Warm-up: wrap_in_quotes

#[test]
fn warmup_typical() {
    assert_eq!(wrap_in_quotes("hello"), "\"hello\"");
}

#[test]
fn warmup_empty() {
    assert_eq!(wrap_in_quotes(""), "\"\"");
}

#[test]
fn warmup_with_spaces() {
    assert_eq!(wrap_in_quotes("hello world"), "\"hello world\"");
}

#[test]
fn warmup_with_inner_quote() {
    assert_eq!(wrap_in_quotes("she said \"hi\""), "\"she said \"hi\"\"");
}

// Main: merge_into

#[test]
fn main_typical() {
    let mut target = String::from("items: ");
    merge_into(&mut target, &["a", "b", "c"], "-");
    assert_eq!(target, "items: a-b-c");
}

#[test]
fn main_empty_parts_no_change() {
    let mut target = String::from("unchanged");
    merge_into(&mut target, &[], "-");
    assert_eq!(target, "unchanged");
}

#[test]
fn main_single_part_no_separator() {
    let mut target = String::new();
    merge_into(&mut target, &["solo"], "-");
    assert_eq!(target, "solo");
}

#[test]
fn main_multi_char_separator() {
    let mut target = String::new();
    merge_into(&mut target, &["one", "two", "three"], ", ");
    assert_eq!(target, "one, two, three");
}
```

- [ ] **Step 3: Verify solution tests pass**

```bash
cargo test --package references-solutions
```

Expected: 8 tests pass.

- [ ] **Step 4: Verify lint passes on the solutions crate**

```bash
cargo clippy --package references-solutions --all-targets -- -D warnings
cargo fmt --check --package references-solutions
```

Expected: both exit 0. No `#[allow]` attributes needed. If clippy fires, STOP and report.

- [ ] **Step 5: Commit**

```bash
git add lessons/08-references/solutions
git commit -m "feat(lesson-08): add reference solutions"
```

---

## Task 4: Lesson README

**Files:**
- Overwrite: `lessons/08-references/README.md`

- [ ] **Step 1: Overwrite `lessons/08-references/README.md`**

The complete file content (write this as the file's actual content, starting with the `# Lesson 08` heading). Code fences inside the markdown are plain triple-backticks.

```markdown
# Lesson 08 — References & borrowing

Lesson 07 taught moves: assigning or passing a `String` moves it, and
the original binding is dead. Today is the alternative. You can let a
function look at a value — or even modify it — without giving up
ownership. References make this possible, and the compiler enforces
rules to keep it safe.

## Learning goals

- Explain what `&T` and `&mut T` are and write functions that take
  them as parameters
- State the borrowing rules — at any moment for any value, either
  many shared references or one mutable reference, never both
- Choose between `T`, `&T`, `&mut T`, and `clone()` when designing a
  function signature
- Read rustc's borrow-checker errors and resolve them by reordering
  or dropping borrows
- Connect `&self` / `&mut self` from Lesson 06 to the reference
  syntax — they're just `&T` and `&mut T` where `T` is the receiver
  type

## Self-study notes

### `&T` — the shared reference

A reference is a pointer-like value that **borrows** another value
without taking ownership. The simplest kind is `&T`, the shared
(read-only) reference:

​```rust
let s = String::from("hello");
let r: &String = &s;
println!("len = {}", r.len());
println!("{s}");          // s is still ours — borrowed, not moved
​```

Multiple shared references to the same value can coexist. The
original owner stays valid throughout.

Most real Rust functions take `&T` (or `&str` for strings) so the
caller keeps ownership:

​```rust
fn show(s: &str) {
    println!("{s}");
}

let owned = String::from("hi");
show(&owned);
show(&owned);             // call as many times as you like
println!("{owned}");      // still ours
​```

### `&mut T` — the mutable reference

`&mut T` is a reference with permission to **modify** the borrowed
value:

​```rust
fn shout(s: &mut String) {
    s.push('!');
}

let mut owned = String::from("hi");
shout(&mut owned);
println!("{owned}");      // "hi!"
​```

To hand out a mutable reference, the caller's binding must be `mut`.
The function still doesn't own the value — it just borrows with
write access for the duration of the call.

### The borrowing rules

At any moment, for any value:

- **either** zero or more shared references (`&T`),
- **or** exactly one mutable reference (`&mut T`),
- **never both at once.**

The borrow checker enforces these rules at compile time. Code that
violates them fails to compile:

​```rust
let mut s = String::from("hi");
let r1 = &s;
let r2 = &mut s;   // ERROR: can't take mut while shared borrow is live
println!("{r1}");
​```

Borrows end at their **last use**, not at end-of-scope. If you
reorder so `r1` is used before `&mut s` is taken, the code compiles
— the shared borrow has ended by the time the mutable borrow begins.

### `&self` and `&mut self` revisited

The receiver kinds you learned in Lesson 06 are now demystified —
they're just `&T` and `&mut T` where `T` is the type the method
belongs to:

​```rust
impl Rectangle {
    fn area(&self) -> u32 {        // borrows self shared
        self.width * self.height
    }

    fn double_width(&mut self) {   // borrows self mutable
        self.width *= 2;
    }
}
​```

- `&self` is `&Rectangle` — a shared borrow
- `&mut self` is `&mut Rectangle` — a mutable borrow
- bare `self` is `Rectangle` — a move (Lesson 07)

Three modes, same machinery.

### Borrows vs moves — when to use which

| Situation | Use |
|---|---|
| Function only reads the value | `fn foo(x: &T)` |
| Function modifies the value in place | `fn foo(x: &mut T)` |
| Function consumes / transforms | `fn foo(x: T) -> T` |
| Need a copy to hand around | `clone()` then pass owned |

In real Rust, most functions take `&T` or `&mut T`. Ownership-taking
is reserved for genuinely transforming the value or storing it
somewhere new.

## Exercises

### Warm-up: `wrap_in_quotes`

Implement `wrap_in_quotes(s: &str) -> String` in
`exercises/src/lib.rs`. The function takes a shared reference and
returns a new owned `String` that wraps `s` in double quotes —
`wrap_in_quotes("hello")` returns `"\"hello\""`.

Use `String::from("\"")` to start the result, then `push_str(s)` and
`push('"')` to build it up.

### Main: `merge_into`

Implement `merge_into(target: &mut String, parts: &[&str], separator: &str)`
so it appends each part to `target` with `separator` between
consecutive parts. If `parts` is empty, leave `target` unchanged.

The signature uses one mutable reference (`target`) and two shared
references (`parts`, `separator`) at the same time. The "many shared
OR one mutable" rule applies per value — different values each have
their own borrows.

Reference solution uses the `first: bool` flag pattern:

​```rust
let mut first = true;
for part in parts {
    if !first {
        target.push_str(separator);
    }
    target.push_str(part);
    first = false;
}
​```

### Compile-fail

`exercises/compile_fails/08-mut-and-shared.rs` ships with code that
takes a `&mut s` while a shared `&s` is still in use. rustc's error
names both reference kinds. The fix is to reorder so the shared
borrow's last use happens before the mutable borrow begins.

### Run

​```bash
make verify LESSON=08-references
​```

This runs your exercise tests and asserts the compile-fail file now
compiles.

## Solutions

See `solutions/src/lib.rs` for the reference implementations. Try the
exercises before peeking.
```

> **IMPORTANT for the implementer:** the code-fence escape in the markdown above uses an invisible zero-width character (shown as `​```) in front of each triple-backtick block — that's only there so this plan file can nest fenced markdown inside an outer fenced markdown block. When you write the actual `README.md`, every fence must be three PLAIN backticks `` ``` `` with NO leading invisible character. After writing, `grep -c '^```' lessons/08-references/README.md` should return 14 (7 code blocks × 2 fence lines).

- [ ] **Step 2: Spot-check the README**

```bash
head -1 lessons/08-references/README.md
grep -c '^### ' lessons/08-references/README.md
grep -c '^```' lessons/08-references/README.md
```

Expected:
- First line: `# Lesson 08 — References & borrowing`
- `grep -c '^### '` returns 9 (five subsections under self-study + four under exercises)
- `grep -c '^```'` returns 14 (7 code blocks × 2 fence lines)

- [ ] **Step 3: Commit**

```bash
git add lessons/08-references/README.md
git commit -m "docs(lesson-08): write self-study notes"
```

---

## Task 5: Slide deck

**Files:**
- Overwrite: `lessons/08-references/slides/slides.md`

- [ ] **Step 1: Overwrite `lessons/08-references/slides/slides.md`**

The complete file content (write this as the file's actual content, starting with the `# References & borrowing` heading):

````
# References & borrowing

> Lesson 07 taught moves. Today: the alternative. You can let a function look at your value — or even modify it — without giving up ownership. The compiler enforces some rules to keep this safe.

---

## Recap

Lesson 07: assigning or passing a `String` **moves** it. The original binding is dead.

Today: the alternative. **Borrow** it. You hand the function a reference to the value and keep ownership yourself.

References come in two flavors.

---

## `&T` — the shared reference

```rust
let s = String::from("hello");
let r: &String = &s;

println!("len = {}", r.len());
println!("{s}");        // s is still ours
```

- `&T` is a **read-only** borrow
- Multiple shared references can coexist
- The original owner stays valid

---

## Functions taking `&T`

```rust
fn show(s: &str) {
    println!("{s}");
}

let owned = String::from("hi");
show(&owned);
show(&owned);     // ...as many times as you like
```

No `.clone()` needed. The function reads through the reference; the caller keeps ownership.

This is why most real Rust functions take `&T`.

---

## `&mut T` — the mutable reference

```rust
fn shout(s: &mut String) {
    s.push('!');
}

let mut owned = String::from("hi");
shout(&mut owned);
println!("{owned}");   // "hi!"
```

- `&mut T` is a borrow **with permission to modify**
- The caller's binding must be `mut` to hand out a mutable reference

---

## The borrowing rules

At any moment for any value:

- **either** zero or more shared references (`&T`)
- **or** exactly one mutable reference (`&mut T`)
- **never both at once**

The borrow checker enforces this at compile time. The compile-fail exercise drills the canonical violation.

```rust
let mut s = String::from("hi");
let r1 = &s;
let r2 = &mut s;   // ERROR: shared borrow still live
println!("{r1}");
```

---

## `&self` / `&mut self` revisited

The receiver kinds from Lesson 06 are just `&T` / `&mut T` where `T` is the type:

```rust
impl Rectangle {
    fn area(&self) -> u32 {        // borrows shared
        self.width * self.height
    }

    fn double_width(&mut self) {   // borrows mutable
        self.width *= 2;
    }
}
```

- `&self` — read the fields
- `&mut self` — modify the fields
- `self` — consume (move, Lesson 07)

Three modes, same machinery.

---

## Borrows vs moves — when to use which

| Situation | Use |
|---|---|
| Function only reads the value | `fn foo(x: &T)` |
| Function modifies in place | `fn foo(x: &mut T)` |
| Function consumes/transforms | `fn foo(x: T) -> T` |
| Need a copy to hand around | `clone()` then pass owned |

In real Rust, most functions take `&T` or `&mut T`. Ownership transfer is reserved for transformation or new storage.

---

## Putting it together

Today's exercises:

- **Warm-up** `wrap_in_quotes(s: &str) -> String` — borrow to read, return new owned data
- **Main** `merge_into(target: &mut String, parts: &[&str], separator: &str)` — one mut, two shared, all live together

Every reference has a *lifetime* — for today, elision handles all our cases.

**Lesson 09** is when we annotate them explicitly.

---

## Wrap — Phase 2 progress

- **`&T`** reads; **`&mut T`** modifies; both leave the original owner intact
- Many shared OR one mutable, never both
- **`&self`**/**`&mut self`** are these receiver kinds applied to methods
- Choose `&T` / `&mut T` / `T` based on what the function does
- Ownership transfer is rare in real Rust code

Next: **Lesson 09 — Lifetimes**.
````

The OUTER fence above (quadruple backticks) is just delimiting this content in the plan. The FILE you write should NOT contain the outer fence — only the inner triple-backtick `rust` blocks.

The file should:
- Start with `# References & borrowing` on line 1
- Have exactly 9 `---` slide separators (between 10 slides)
- Contain 6 triple-backtick `rust` code fences

- [ ] **Step 2: Verify `make slides-build` succeeds and includes lesson 08**

```bash
make slides-build
test -f dist/lessons/08-references/slides/slides.md
test -f dist/lessons/08-references/slides/index.html
grep -c "08-references" dist/index.html
```

Expected: `slides.md` and `index.html` copied into dist; `grep -c "08-references"` returns at least 1.

- [ ] **Step 3: Spot-check slide separators**

```bash
grep -c '^---$' lessons/08-references/slides/slides.md
```

Expected: `9` (between 10 slides).

- [ ] **Step 4: Commit**

```bash
git add lessons/08-references/slides/slides.md
git commit -m "feat(lesson-08): write slide deck"
```

---

## Task 6: End-to-end verification + push

- [ ] **Step 1: `make ci` is green**

```bash
make ci
```

Expected: exit 0. Clippy clean, fmt clean, workspace builds, default-members tests pass (now includes the 8 new tests in `references-solutions`), compile-fail `--expect broken` passes for lesson 08.

- [ ] **Step 2: `make verify LESSON=08-references` fails (the exercise is undone — intentional)**

```bash
make verify LESSON=08-references || echo "expected: exercise tests fail with todo!() panic"
```

Expected: non-zero exit. All 8 exercise tests panic with `not yet implemented`.

- [ ] **Step 3: `make slides-build` final state**

```bash
make slides-build
ls dist/lessons/
grep -c "08-references" dist/index.html
```

Expected: `dist/lessons/` contains all eight lessons (01-hello-rust through 08-references). `grep -c "08-references"` ≥ 1.

- [ ] **Step 4: Push**

```bash
git push
```

Expected: push succeeds. CI runs and is green; Deploy rebuilds the static site so lesson 08 appears live.

- [ ] **Step 5: Smoke-test the deployed site**

After the push, wait for the Deploy workflow to finish (`gh run watch <id>`). Then:

```bash
curl -sS -o /dev/null -w "%{http_code}\n" https://rust.ristkari.dev/
curl -sS -o /dev/null -w "%{http_code}\n" https://rust.ristkari.dev/lessons/08-references/slides/
```

Expected: both return `200`.

---

## Done criteria

- `lessons/08-references/` exists with all four parts
- `cargo test --package references-solutions` → 8 passing tests
- `cargo test --manifest-path lessons/08-references/exercises/Cargo.toml` → 8 panicking tests (intentional)
- `cargo run --package compile-fails -- --expect broken lessons/08-references` → ok
- `cargo run --package compile-fails -- --expect compiles lessons/08-references` → fails (intentional)
- `make ci` → green
- `make slides-build` → produces `dist/lessons/08-references/slides/index.html`
- `dist/index.html` lists lesson 08 as a clickable link
- All changes committed and pushed
- Deployed site returns HTTP 200 for `/` and `/lessons/08-references/slides/`
