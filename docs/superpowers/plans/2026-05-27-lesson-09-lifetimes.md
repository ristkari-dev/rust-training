# Lesson 09 — Lifetimes — Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Author the third lesson of Phase 2 of the Rust training course: lifetimes. Warm-up is `longest<'a>` (the canonical "elision fails, must annotate" function). Main is the `Excerpt<'a>` struct (the canonical "struct holding a reference needs explicit lifetimes"). Compile-fail is "forgot the `<'a>` on a struct field" — rustc's diagnostic literally tells you the fix.

**Architecture:** Use the existing `make new-lesson` scaffolder. The exercise stub ships with the lifetime annotations already in place — students fill in the function bodies, not the signatures. The lesson is "understand what these annotations mean and why they're necessary."

**Tech Stack:** Rust 2024 edition, existing tools (`new-lesson`, `compile-fails`, `slides-dev`, `build-index`), reveal.js (vendored), GNU Make.

**Spec:** [`docs/superpowers/specs/2026-05-27-lesson-09-lifetimes-design.md`](../specs/2026-05-27-lesson-09-lifetimes-design.md).

**Working directory:** `/Users/ristkari/code/private/rust-training`.

---

## Task 1: Scaffold lessons/09-lifetimes

**Files (all created by the scaffolder):**
- `lessons/09-lifetimes/README.md` (placeholder, replaced in Task 4)
- `lessons/09-lifetimes/slides/index.html` (final — no edit needed)
- `lessons/09-lifetimes/slides/slides.md` (placeholder, replaced in Task 5)
- `lessons/09-lifetimes/exercises/Cargo.toml` (final — no edit needed)
- `lessons/09-lifetimes/exercises/src/lib.rs` (placeholder, replaced in Task 2)
- `lessons/09-lifetimes/exercises/tests/exercise.rs` (placeholder, replaced in Task 2)
- `lessons/09-lifetimes/solutions/Cargo.toml` (final — no edit needed)
- `lessons/09-lifetimes/solutions/src/lib.rs` (placeholder, replaced in Task 3)
- `lessons/09-lifetimes/solutions/tests/exercise.rs` (placeholder, replaced in Task 3)

- [ ] **Step 1: Run the scaffolder**

```bash
make new-lesson NAME=09-lifetimes
```

Expected: `scaffolded ./lessons/09-lifetimes`.

- [ ] **Step 2: Verify directory structure**

```bash
ls lessons/09-lifetimes/
ls lessons/09-lifetimes/slides/ lessons/09-lifetimes/exercises/ lessons/09-lifetimes/solutions/
```

Expected: top-level `README.md`, `slides/`, `exercises/`, `solutions/` populated from templates.

- [ ] **Step 3: Verify Cargo package names**

```bash
grep '^name' lessons/09-lifetimes/exercises/Cargo.toml lessons/09-lifetimes/solutions/Cargo.toml
```

Expected:
```
lessons/09-lifetimes/exercises/Cargo.toml:name = "lifetimes-exercises"
lessons/09-lifetimes/solutions/Cargo.toml:name = "lifetimes-solutions"
```

- [ ] **Step 4: Verify the workspace picks up the new crates**

```bash
cargo metadata --no-deps --format-version 1 | grep -o '"name":"lifetimes-[^"]*"' | sort -u
```

Expected output:
```
"name":"lifetimes-exercises"
"name":"lifetimes-solutions"
```

- [ ] **Step 5: Verify the scaffolded workspace builds clean**

```bash
cargo build --workspace
```

Expected: warning-free build.

- [ ] **Step 6: Commit the scaffold**

```bash
git add lessons/09-lifetimes
git commit -m "chore: scaffold lessons/09-lifetimes"
```

---

## Task 2: Exercise content (struct + function stubs + tests + compile-fail)

**Files:**
- Overwrite: `lessons/09-lifetimes/exercises/src/lib.rs`
- Overwrite: `lessons/09-lifetimes/exercises/tests/exercise.rs`
- Create: `lessons/09-lifetimes/exercises/compile_fails/09-missing-lifetime.rs`

- [ ] **Step 1: Overwrite `lessons/09-lifetimes/exercises/src/lib.rs`**

```rust
//! Lesson 09 — exercises.
//!
//! Implement `longest` (warm-up) and the `Excerpt` struct's methods
//! (main) so that `cargo test --manifest-path
//! lessons/09-lifetimes/exercises/Cargo.toml` passes. The tests live
//! in `tests/exercise.rs`.

#[must_use]
pub fn longest<'a>(_a: &'a str, _b: &'a str) -> &'a str {
    todo!("return whichever of a and b has the greater (or equal) length")
}

pub struct Excerpt<'a> {
    pub text: &'a str,
}

impl<'a> Excerpt<'a> {
    #[must_use]
    pub fn new(_text: &'a str) -> Self {
        todo!("construct an Excerpt holding text")
    }

    #[must_use]
    pub fn length(&self) -> usize {
        todo!("return the length of the held text")
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        todo!("return whether the held text is empty")
    }
}
```

- [ ] **Step 2: Overwrite `lessons/09-lifetimes/exercises/tests/exercise.rs`**

```rust
use lifetimes_exercises::{Excerpt, longest};

// Warm-up: longest

#[test]
fn warmup_b_is_longer() {
    assert_eq!(longest("hi", "hello"), "hello");
}

#[test]
fn warmup_a_is_longer() {
    assert_eq!(longest("hello", "hi"), "hello");
}

#[test]
fn warmup_equal_takes_a() {
    assert_eq!(longest("same", "size"), "same");
}

#[test]
fn warmup_empty_a() {
    assert_eq!(longest("", "anything"), "anything");
}

// Main: Excerpt<'a>

#[test]
fn main_text_field_accessible() {
    let e = Excerpt::new("hello");
    assert_eq!(e.text, "hello");
}

#[test]
fn main_length() {
    assert_eq!(Excerpt::new("hello").length(), 5);
}

#[test]
fn main_is_empty_true() {
    assert!(Excerpt::new("").is_empty());
}

#[test]
fn main_is_empty_false() {
    assert!(!Excerpt::new("hi").is_empty());
}
```

- [ ] **Step 3: Create `lessons/09-lifetimes/exercises/compile_fails/09-missing-lifetime.rs`**

```rust
// Compile-fail exercise: this file MUST NOT compile until you fix it.
//
// Any struct that holds a reference must declare a lifetime parameter
// and use it on the field. Otherwise the compiler can't track how
// long the struct is valid — and refuses to compile.
//
// The struct below tries to hold a `&str` without a lifetime
// parameter. rustc will say "missing lifetime specifier" and suggest
// the fix.
//
// Hint: read the rustc error. The fix is to declare `<'a>` on the
// struct and use `&'a str` for the field. Like this:
//
//     struct Excerpt<'a> {
//         text: &'a str,
//     }

struct Excerpt {
    text: &str,
}

fn main() {
    let s = String::from("hello world");
    let e = Excerpt { text: &s };
    println!("{}", e.text);
}
```

- [ ] **Step 4: Verify exercise tests fail with `todo!()` panic (intentional)**

```bash
cargo test --manifest-path lessons/09-lifetimes/exercises/Cargo.toml
```

Expected: all 8 tests fail with `not yet implemented` panic message.

- [ ] **Step 5: Verify the exercises crate still builds cleanly**

```bash
cargo build --package lifetimes-exercises
```

Expected: warning-free build.

- [ ] **Step 6: Verify compile-fail ships broken**

```bash
cargo run --package compile-fails -- --expect broken lessons/09-lifetimes
```

Expected: `ok: lessons/09-lifetimes/exercises/compile_fails/09-missing-lifetime.rs` and exit 0.

- [ ] **Step 7: Verify compile-fail's student-mode check fires**

```bash
cargo run --package compile-fails -- --expect compiles lessons/09-lifetimes
```

Expected: non-zero exit with `FAIL: file did not compile, but was expected to: lessons/09-lifetimes/...`.

- [ ] **Step 8: Verify lint passes on the exercises crate**

```bash
cargo clippy --package lifetimes-exercises --all-targets -- -D warnings
cargo fmt --check --package lifetimes-exercises
```

Expected: both exit 0.

- [ ] **Step 9: Commit**

```bash
git add lessons/09-lifetimes/exercises
git commit -m "feat(lesson-09): add longest fn, Excerpt struct stubs, tests, and compile-fail"
```

---

## Task 3: Reference solutions

**Files:**
- Overwrite: `lessons/09-lifetimes/solutions/src/lib.rs`
- Overwrite: `lessons/09-lifetimes/solutions/tests/exercise.rs`

- [ ] **Step 1: Overwrite `lessons/09-lifetimes/solutions/src/lib.rs`**

```rust
//! Lesson 09 — reference solutions.

#[must_use]
pub fn longest<'a>(a: &'a str, b: &'a str) -> &'a str {
    if a.len() >= b.len() { a } else { b }
}

pub struct Excerpt<'a> {
    pub text: &'a str,
}

impl<'a> Excerpt<'a> {
    #[must_use]
    pub fn new(text: &'a str) -> Self {
        Excerpt { text }
    }

    #[must_use]
    pub fn length(&self) -> usize {
        self.text.len()
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.text.is_empty()
    }
}
```

> Pedagogical notes baked into the solutions:
> - `longest<'a>` is the canonical "elision fails — two input refs, one output ref" function. The `<'a>` declares one lifetime that all three references share.
> - `Excerpt<'a>` is the canonical "struct holds a reference" pattern. The `<'a>` appears on the struct declaration, on the `impl` block, and on the constructor's parameter type — three places, all required.
> - `length` and `is_empty` take `&self` and return owned types (`usize`, `bool`). No further lifetime annotation is needed inside their bodies because elision handles the `&self` case.
>
> No `#[allow]` attributes should be needed.

- [ ] **Step 2: Overwrite `lessons/09-lifetimes/solutions/tests/exercise.rs`**

```rust
use lifetimes_solutions::{Excerpt, longest};

// Warm-up: longest

#[test]
fn warmup_b_is_longer() {
    assert_eq!(longest("hi", "hello"), "hello");
}

#[test]
fn warmup_a_is_longer() {
    assert_eq!(longest("hello", "hi"), "hello");
}

#[test]
fn warmup_equal_takes_a() {
    assert_eq!(longest("same", "size"), "same");
}

#[test]
fn warmup_empty_a() {
    assert_eq!(longest("", "anything"), "anything");
}

// Main: Excerpt<'a>

#[test]
fn main_text_field_accessible() {
    let e = Excerpt::new("hello");
    assert_eq!(e.text, "hello");
}

#[test]
fn main_length() {
    assert_eq!(Excerpt::new("hello").length(), 5);
}

#[test]
fn main_is_empty_true() {
    assert!(Excerpt::new("").is_empty());
}

#[test]
fn main_is_empty_false() {
    assert!(!Excerpt::new("hi").is_empty());
}
```

- [ ] **Step 3: Verify solution tests pass**

```bash
cargo test --package lifetimes-solutions
```

Expected: 8 tests pass.

- [ ] **Step 4: Verify lint passes on the solutions crate**

```bash
cargo clippy --package lifetimes-solutions --all-targets -- -D warnings
cargo fmt --check --package lifetimes-solutions
```

Expected: both exit 0. No `#[allow]` attributes needed. If clippy fires, STOP and report.

- [ ] **Step 5: Commit**

```bash
git add lessons/09-lifetimes/solutions
git commit -m "feat(lesson-09): add reference solutions"
```

---

## Task 4: Lesson README

**Files:**
- Overwrite: `lessons/09-lifetimes/README.md`

- [ ] **Step 1: Overwrite `lessons/09-lifetimes/README.md`**

The complete file content (write this as the file's actual content, starting with the `# Lesson 09` heading). Code fences inside the markdown are plain triple-backticks.

```markdown
# Lesson 09 — Lifetimes

Every reference has a lifetime — a scope during which the reference
is valid. Most of the time, Rust figures lifetimes out for you. This
lesson is about the syntax for the cases where it can't, and why.

## Learning goals

- Explain that every reference has a lifetime — a scope during which
  it's valid — and that the compiler tracks it
- Identify when lifetime elision applies (single input reference,
  methods with `&self`) and when it doesn't
- Write a function signature with explicit `<'a>` annotation when
  elision fails
- Define a struct that holds a reference, declaring and using the
  lifetime parameter consistently across struct, `impl`, and methods
- Recognize `&'static str` as a reference that lives for the entire
  program

## Self-study notes

### Every reference has a lifetime

A **lifetime** is the scope during which a reference is valid. The
compiler tracks it for every reference. Most of the time you never
see it — Rust infers it through *lifetime elision*. When you do see
it, like `&'static str` for string literals, the syntax is just
surfacing what was already there.

### Elision handles the common cases

For most function signatures, Rust deduces the lifetime relationships
automatically:

​```rust
fn first_char(s: &str) -> &str { /* ... */ }
fn area(rect: &Rectangle) -> u32 { /* ... */ }
​```

You've been writing code like this since Lesson 04 without thinking
about lifetimes. The rule, informally: when there's exactly one
input reference, the output reference (if any) borrows from it.

### When elision fails, and the `<'a>` syntax

Two reference parameters, one returned reference — the compiler
can't guess which input the output borrows from:

​```rust
fn longest(a: &str, b: &str) -> &str {     // won't compile
    if a.len() >= b.len() { a } else { b }
}
​```

rustc says: *"this function's return type contains a borrowed value,
but the signature does not say whether it is borrowed from `a` or
`b`."*

You spell out the relationship with `<'a>`:

​```rust
fn longest<'a>(a: &'a str, b: &'a str) -> &'a str {
    if a.len() >= b.len() { a } else { b }
}
​```

Read this as: "for some lifetime `'a`, both inputs and the output
all share that lifetime." The compiler picks `'a` at each call site
to be the shorter of the two input lifetimes — that's the lifetime
of the returned reference.

### Lifetimes in struct fields

Any struct that holds a reference must declare a lifetime parameter:

​```rust
struct Excerpt<'a> {
    text: &'a str,
}
​```

This says: "the struct cannot outlive the `&str` it holds." If the
original string is dropped, any `Excerpt` referencing it is
invalidated by the compiler.

The `impl` block also carries `<'a>`:

​```rust
impl<'a> Excerpt<'a> {
    pub fn new(text: &'a str) -> Self {
        Excerpt { text }
    }

    pub fn length(&self) -> usize {
        self.text.len()
    }
}
​```

Within methods that take `&self`, you don't need to annotate further
— the receiver's lifetime is implicit and elision handles it.

### `'static` — the special lifetime

`&'static str` is a reference that lives for the entire program.
String literals are `&'static str` (slices into the compiled binary):

​```rust
let greeting: &'static str = "hello";
​```

Most other references have shorter, scope-limited lifetimes. You'll
occasionally need `'static` in trait bounds and APIs (it'll come up
later in the course), but it's not common in everyday function
signatures.

## Exercises

### Warm-up: `longest`

Implement `longest<'a>(a: &'a str, b: &'a str) -> &'a str` in
`exercises/src/lib.rs`. Return whichever of `a` and `b` has the
greater length (or `a` on a tie).

The signature is already annotated for you — your job is to fill in
the body. The `<'a>` says "the returned reference borrows from one
of `a` or `b`, both of which share lifetime `'a`."

### Main: `Excerpt<'a>`

The exercises crate ships an `Excerpt<'a>` struct holding a `&'a
str` field, plus an `impl` block with three stub methods:

- `Excerpt::new(text: &'a str) -> Self` — constructor
- `excerpt.length()` — returns the length of the held text
- `excerpt.is_empty()` — returns whether the held text is empty

Fill in the three `todo!()` bodies. The `<'a>` plumbing is already
done — the lesson is to see how it threads through the struct, the
impl block, and the constructor.

### Compile-fail

`exercises/compile_fails/09-missing-lifetime.rs` ships with a struct
holding a `&str` field but no lifetime parameter. rustc's error
message *includes the fix* — it'll suggest adding `<'a>` to the
struct and `&'a str` to the field. Read the error carefully; the
compiler is your friend here.

### Run

​```bash
make verify LESSON=09-lifetimes
​```

This runs your exercise tests and asserts the compile-fail file now
compiles.

## Solutions

See `solutions/src/lib.rs` for the reference implementations. Try
the exercises before peeking.
```

> **IMPORTANT for the implementer:** the code-fence escape in the markdown above uses an invisible zero-width character (shown as `​```) in front of each triple-backtick block — that's only there so this plan file can nest fenced markdown inside an outer fenced markdown block. When you write the actual `README.md`, every fence must be three PLAIN backticks `` ``` `` with NO leading invisible character. After writing, `grep -c '^```' lessons/09-lifetimes/README.md` should return 14 (7 code blocks × 2 fence lines).

- [ ] **Step 2: Spot-check the README**

```bash
head -1 lessons/09-lifetimes/README.md
grep -c '^### ' lessons/09-lifetimes/README.md
grep -c '^```' lessons/09-lifetimes/README.md
```

Expected:
- First line: `# Lesson 09 — Lifetimes`
- `grep -c '^### '` returns 9 (five subsections under self-study + four under exercises)
- `grep -c '^```'` returns 14 (7 code blocks × 2 fence lines)

- [ ] **Step 3: Commit**

```bash
git add lessons/09-lifetimes/README.md
git commit -m "docs(lesson-09): write self-study notes"
```

---

## Task 5: Slide deck

**Files:**
- Overwrite: `lessons/09-lifetimes/slides/slides.md`

- [ ] **Step 1: Overwrite `lessons/09-lifetimes/slides/slides.md`**

The complete file content (write this as the file's actual content, starting with the `# Lifetimes` heading):

````
# Lifetimes

> Every reference has a lifetime. Most of the time, Rust figures it out for you. Today we learn the syntax for when it can't — and why.

---

## Recap

Lesson 08 introduced `&T` and `&mut T`. We mentioned that every reference has a **lifetime** — and that elision usually handles them silently.

Today: we open the box.

---

## Every reference has a lifetime

Conceptually, a lifetime is the **scope during which a reference is valid**.

The compiler tracks it for every reference.

Most of the time it's implicit and you never see it. When you do — like `&'static str` for string literals — that's lifetime syntax surfacing.

---

## Elision handles the common cases

```rust
fn first_char(s: &str) -> &str { /* ... */ }
fn area(rect: &Rectangle) -> u32 { /* ... */ }
```

For one-input-reference functions and methods (`&self`), Rust deduces the relationship between input and output references.

You've been writing code like this since Lesson 04 without thinking about it.

---

## When elision fails

Two reference parameters, one returned reference — the compiler can't guess which one the output borrows from:

```rust
fn longest(a: &str, b: &str) -> &str {
    if a.len() >= b.len() { a } else { b }
}
```

rustc says: *"this function's return type contains a borrowed value, but the signature does not say whether it is borrowed from `a` or `b`."*

You have to spell out the relationship.

---

## `<'a>` syntax

```rust
fn longest<'a>(a: &'a str, b: &'a str) -> &'a str {
    if a.len() >= b.len() { a } else { b }
}
```

Read this as: "for some lifetime `'a`, both inputs and the output all share that lifetime."

The compiler picks `'a` at each call site to be the **shortest** of the two inputs' lifetimes — that's the lifetime of the returned reference.

---

## The `longest` function

Without `'a`, the signature is ambiguous and won't compile.

With `'a`, the compiler accepts the body and call sites.

At each call:

- The returned reference is valid for as long as **both** input references are valid.
- The shorter of the two lifetimes wins.

This is the warm-up exercise.

---

## Structs holding references

A struct field that's a reference forces the struct to declare a lifetime parameter:

```rust
struct Excerpt<'a> {
    text: &'a str,
}
```

This says: "the struct cannot outlive the `&str` it holds."

If the original string is dropped, any `Excerpt` referencing it is invalidated by the compiler.

The compile-fail exercise drills the canonical mistake — forgetting the `<'a>`.

---

## `'static` — the special lifetime

`&'static str` is a reference that lives for the **entire program**.

String literals are `&'static str` (slices into the compiled binary):

```rust
let greeting: &'static str = "hello";
```

Most other references have shorter, scope-limited lifetimes. You'll occasionally need `'static` in trait bounds and APIs, but it's not common in everyday function signatures.

---

## Wrap — Phase 2 progress

- Every reference has a lifetime
- Elision handles the common cases (single-input, `&self`)
- Explicit `<'a>` is needed for multi-input functions returning a reference, and for any struct with a reference field
- Lifetime parameters say "all these references share a relationship"
- `'static` lives for the whole program

Next: **Lesson 10 — Smart pointers** (`Box`, `Rc`, `Arc`, `RefCell`).
````

The OUTER fence above (quadruple backticks) is just delimiting this content in the plan. The FILE you write should NOT contain the outer fence — only the inner triple-backtick `rust` blocks.

The file should:
- Start with `# Lifetimes` on line 1
- Have exactly 9 `---` slide separators (between 10 slides)
- Contain 5 triple-backtick `rust` code fences

- [ ] **Step 2: Verify `make slides-build` succeeds and includes lesson 09**

```bash
make slides-build
test -f dist/lessons/09-lifetimes/slides/slides.md
test -f dist/lessons/09-lifetimes/slides/index.html
grep -c "09-lifetimes" dist/index.html
```

Expected: `slides.md` and `index.html` copied into dist; `grep -c "09-lifetimes"` returns at least 1.

- [ ] **Step 3: Spot-check slide separators**

```bash
grep -c '^---$' lessons/09-lifetimes/slides/slides.md
```

Expected: `9` (between 10 slides).

- [ ] **Step 4: Commit**

```bash
git add lessons/09-lifetimes/slides/slides.md
git commit -m "feat(lesson-09): write slide deck"
```

---

## Task 6: End-to-end verification + push

- [ ] **Step 1: `make ci` is green**

```bash
make ci
```

Expected: exit 0. Clippy clean, fmt clean, workspace builds, default-members tests pass (now includes the 8 new tests in `lifetimes-solutions`), compile-fail `--expect broken` passes for lesson 09.

- [ ] **Step 2: `make verify LESSON=09-lifetimes` fails (the exercise is undone — intentional)**

```bash
make verify LESSON=09-lifetimes || echo "expected: exercise tests fail with todo!() panic"
```

Expected: non-zero exit. All 8 exercise tests panic with `not yet implemented`.

- [ ] **Step 3: `make slides-build` final state**

```bash
make slides-build
ls dist/lessons/
grep -c "09-lifetimes" dist/index.html
```

Expected: `dist/lessons/` contains all nine lessons. `grep -c "09-lifetimes"` ≥ 1.

- [ ] **Step 4: Push**

```bash
git push
```

Expected: push succeeds. CI runs and is green; Deploy rebuilds the static site so lesson 09 appears live.

- [ ] **Step 5: Smoke-test the deployed site**

After the push, wait for the Deploy workflow to finish (`gh run watch <id>`). Then:

```bash
curl -sS -o /dev/null -w "%{http_code}\n" https://rust.ristkari.dev/
curl -sS -o /dev/null -w "%{http_code}\n" https://rust.ristkari.dev/lessons/09-lifetimes/slides/
```

Expected: both return `200`.

---

## Done criteria

- `lessons/09-lifetimes/` exists with all four parts
- Both `exercises/src/lib.rs` and `solutions/src/lib.rs` define the `Excerpt<'a>` struct with identical signatures
- `cargo test --package lifetimes-solutions` → 8 passing tests
- `cargo test --manifest-path lessons/09-lifetimes/exercises/Cargo.toml` → 8 panicking tests (intentional)
- `cargo run --package compile-fails -- --expect broken lessons/09-lifetimes` → ok
- `cargo run --package compile-fails -- --expect compiles lessons/09-lifetimes` → fails (intentional)
- `make ci` → green
- `make slides-build` → produces `dist/lessons/09-lifetimes/slides/index.html`
- `dist/index.html` lists lesson 09 as a clickable link
- All changes committed and pushed
- Deployed site returns HTTP 200 for `/` and `/lessons/09-lifetimes/slides/`
