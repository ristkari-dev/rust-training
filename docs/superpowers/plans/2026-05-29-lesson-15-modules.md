# Lesson 15 — Modules, crates, workspaces — Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Author the fourth and final lesson of Phase 3 of the Rust training course: modules, crates, and workspaces. The exercise crate is genuinely multi-file so students see the module→file mapping. Warm-up: `rectangle_area` (a leaf function in `geometry/shapes.rs`). Main: `total_area` (in `geometry.rs`, calling `shapes::rectangle_area` via a path). Compile-fail: a private function called from outside its module (E0603).

**Architecture:** Use the existing `make new-lesson` scaffolder, then replace the single scaffolded `src/lib.rs` with a real multi-file module tree (`lib.rs` + `geometry.rs` + `geometry/shapes.rs`) in both the exercises and solutions crates. The module structure, `mod`/`pub mod` declarations, and `pub use` re-exports ship complete; students fill the two function bodies. The crate and tests compile in the undone state; tests panic at runtime, like every prior lesson. All reference code in this plan was empirically verified clippy-pedantic-clean during design.

**Tech Stack:** Rust 2024 edition, existing tools (`new-lesson`, `compile-fails`, `slides-dev`, `build-index`), reveal.js (vendored), GNU Make. No external dependencies.

**Spec:** [`docs/superpowers/specs/2026-05-29-lesson-15-modules-design.md`](../specs/2026-05-29-lesson-15-modules-design.md).

**Working directory:** `/Users/ristkari/code/private/rust-training`.

**Commit convention:** Plain commit messages only — no `Co-Authored-By` trailer or any AI attribution.

---

## Task 1: Scaffold lessons/15-modules

**Files (all created by the scaffolder):**
- `lessons/15-modules/README.md` (placeholder, replaced in Task 4)
- `lessons/15-modules/slides/index.html` (final — no edit needed)
- `lessons/15-modules/slides/slides.md` (placeholder, replaced in Task 5)
- `lessons/15-modules/exercises/Cargo.toml` (final — no edit needed)
- `lessons/15-modules/exercises/src/lib.rs` (replaced + tree added in Task 2)
- `lessons/15-modules/exercises/tests/exercise.rs` (placeholder, replaced in Task 2)
- `lessons/15-modules/solutions/Cargo.toml` (final — no edit needed)
- `lessons/15-modules/solutions/src/lib.rs` (replaced + tree added in Task 3)
- `lessons/15-modules/solutions/tests/exercise.rs` (placeholder, replaced in Task 3)

- [ ] **Step 1: Run the scaffolder**

```bash
make new-lesson NAME=15-modules
```

Expected: `scaffolded ./lessons/15-modules`.

- [ ] **Step 2: Verify directory structure**

```bash
ls lessons/15-modules/
ls lessons/15-modules/slides/ lessons/15-modules/exercises/ lessons/15-modules/solutions/
```

Expected: top-level `README.md`, `slides/`, `exercises/`, `solutions/` populated from templates.

- [ ] **Step 3: Verify Cargo package names**

```bash
grep '^name' lessons/15-modules/exercises/Cargo.toml lessons/15-modules/solutions/Cargo.toml
```

Expected:
```
lessons/15-modules/exercises/Cargo.toml:name = "modules-exercises"
lessons/15-modules/solutions/Cargo.toml:name = "modules-solutions"
```

- [ ] **Step 4: Verify the workspace picks up the new crates**

```bash
cargo metadata --no-deps --format-version 1 | grep -o '"name":"modules-[^"]*"' | sort -u
```

Expected output:
```
"name":"modules-exercises"
"name":"modules-solutions"
```

- [ ] **Step 5: Verify the scaffolded workspace builds clean**

```bash
cargo build --workspace
```

Expected: warning-free build.

- [ ] **Step 6: Commit the scaffold**

```bash
git add lessons/15-modules
git commit -m "chore: scaffold lessons/15-modules"
```

---

## Task 2: Exercise content (multi-file module tree + stubs + tests + compile-fail)

**Files:**
- Overwrite: `lessons/15-modules/exercises/src/lib.rs`
- Create: `lessons/15-modules/exercises/src/geometry.rs`
- Create: `lessons/15-modules/exercises/src/geometry/shapes.rs`
- Overwrite: `lessons/15-modules/exercises/tests/exercise.rs`
- Create: `lessons/15-modules/exercises/compile_fails/15-private-fn.rs`

- [ ] **Step 1: Overwrite `lessons/15-modules/exercises/src/lib.rs`**

```rust
//! Lesson 15 — exercises.
//!
//! Implement `rectangle_area` (warm-up, in `geometry/shapes.rs`) and
//! `total_area` (main, in `geometry.rs`) so that `cargo test
//! --manifest-path lessons/15-modules/exercises/Cargo.toml` passes. The
//! module structure and re-exports are given — this lesson is about how
//! code is organized into modules. The tests live in
//! `tests/exercise.rs`.

pub mod geometry;

pub use geometry::shapes::rectangle_area;
pub use geometry::total_area;
```

- [ ] **Step 2: Create `lessons/15-modules/exercises/src/geometry.rs`**

```rust
//! Geometry helpers (the parent module).

pub mod shapes;

#[must_use]
pub fn total_area(_rects: &[(u32, u32)]) -> u32 {
    todo!("sum the area of every (width, height) rectangle using shapes::rectangle_area")
}
```

- [ ] **Step 3: Create `lessons/15-modules/exercises/src/geometry/shapes.rs`**

The `src/geometry/` directory does not exist yet — create it.

```rust
//! Rectangle area calculations (a leaf module).

#[must_use]
pub fn rectangle_area(_width: u32, _height: u32) -> u32 {
    todo!("return the rectangle's area: width times height")
}
```

- [ ] **Step 4: Overwrite `lessons/15-modules/exercises/tests/exercise.rs`**

```rust
use modules_exercises::{rectangle_area, total_area};

// Warm-up: rectangle_area (leaf module geometry::shapes)

#[test]
fn warmup_area_basic() {
    assert_eq!(rectangle_area(3, 4), 12);
}

#[test]
fn warmup_area_zero() {
    assert_eq!(rectangle_area(0, 5), 0);
}

#[test]
fn warmup_area_square() {
    assert_eq!(rectangle_area(6, 6), 36);
}

#[test]
fn warmup_area_via_full_path() {
    // the full module path also works
    assert_eq!(modules_exercises::geometry::shapes::rectangle_area(2, 7), 14);
}

// Main: total_area (parent module geometry)

#[test]
fn main_total_empty() {
    let rects: [(u32, u32); 0] = [];
    assert_eq!(total_area(&rects), 0);
}

#[test]
fn main_total_one() {
    assert_eq!(total_area(&[(3, 4)]), 12);
}

#[test]
fn main_total_many() {
    assert_eq!(total_area(&[(2, 3), (4, 5), (1, 1)]), 27);
}

#[test]
fn main_total_via_module_path() {
    assert_eq!(modules_exercises::geometry::total_area(&[(10, 10)]), 100);
}
```

- [ ] **Step 5: Create `lessons/15-modules/exercises/compile_fails/15-private-fn.rs`**

The `compile_fails/` directory does not exist yet — create it. This file is self-contained and std-only. Write this file:

```rust
// Compile-fail exercise: this file MUST NOT compile until you fix it.
//
// Items in a module are PRIVATE by default — visible only inside that
// module (and its descendants). The `rectangle_area` function below has
// no `pub`, so `main`, which is outside the `geometry` module, cannot
// call it. The compiler reports E0603: "function `rectangle_area` is
// private".
//
// The fix: mark the function `pub` so it becomes part of the module's
// public surface. (If the module itself were nested and private, it
// would need `pub` too — the whole path must be public.)
//
// Hint: add `pub` in front of `fn rectangle_area`.

mod geometry {
    fn rectangle_area(width: u32, height: u32) -> u32 {
        width * height
    }
}

fn main() {
    let area = geometry::rectangle_area(3, 4);
    println!("{area}");
}
```

- [ ] **Step 6: Verify exercise tests compile and fail with `todo!()` panic (intentional)**

```bash
cargo test --manifest-path lessons/15-modules/exercises/Cargo.toml
```

Expected: the crate COMPILES, then all 8 tests FAIL with `not yet implemented` panic message. (Compilation succeeding while tests panic is the correct undone state — the module tree and signatures are complete; only the two function bodies are `todo!()`.)

- [ ] **Step 7: Verify the exercises crate still builds cleanly**

```bash
cargo build --package modules-exercises
```

Expected: warning-free build.

- [ ] **Step 8: Verify compile-fail ships broken**

```bash
cargo run --package compile-fails -- --expect broken lessons/15-modules
```

Expected: prints `ok: lessons/15-modules/exercises/compile_fails/15-private-fn.rs` and exits 0. (The tool printing the rustc E0603 error text is expected — what matters is the final `ok:` line and exit 0.)

- [ ] **Step 9: Verify compile-fail's student-mode check fires**

```bash
cargo run --package compile-fails -- --expect compiles lessons/15-modules
```

Expected: non-zero exit with a `FAIL: file did not compile, but was expected to: lessons/15-modules/...` message. (This is correct — the file ships broken on purpose.)

- [ ] **Step 10: Verify lint passes on the exercises crate**

```bash
cargo clippy --package modules-exercises --all-targets -- -D warnings
cargo fmt --check --package modules-exercises
```

Expected: both exit 0. (The `todo!()` bodies with unused `_width`/`_height`/`_rects` params lint clean — verified during design.)

- [ ] **Step 11: Commit**

```bash
git add lessons/15-modules/exercises
git commit -m "feat(lesson-15): add module tree, exercise stubs, tests, and compile-fail"
```

---

## Task 3: Reference solutions

**Files:**
- Overwrite: `lessons/15-modules/solutions/src/lib.rs`
- Create: `lessons/15-modules/solutions/src/geometry.rs`
- Create: `lessons/15-modules/solutions/src/geometry/shapes.rs`
- Overwrite: `lessons/15-modules/solutions/tests/exercise.rs`

- [ ] **Step 1: Overwrite `lessons/15-modules/solutions/src/lib.rs`**

```rust
//! Lesson 15 — reference solutions.

pub mod geometry;

pub use geometry::shapes::rectangle_area;
pub use geometry::total_area;
```

- [ ] **Step 2: Create `lessons/15-modules/solutions/src/geometry.rs`**

```rust
//! Geometry helpers (the parent module).

pub mod shapes;

#[must_use]
pub fn total_area(rects: &[(u32, u32)]) -> u32 {
    rects.iter().map(|&(w, h)| shapes::rectangle_area(w, h)).sum()
}
```

- [ ] **Step 3: Create `lessons/15-modules/solutions/src/geometry/shapes.rs`**

The `src/geometry/` directory does not exist yet — create it.

```rust
//! Rectangle area calculations (a leaf module).

#[must_use]
pub fn rectangle_area(width: u32, height: u32) -> u32 {
    width * height
}
```

> Pedagogical notes:
> - `rectangle_area` is a `pub` leaf function two modules deep (`crate::geometry::shapes`). Simple body; the lesson is *where it lives* and *how it's reached*.
> - `total_area` (in the parent module) reaches its child module via the `shapes::` path, then sums with an iterator chain (reuses L11). `|&(w, h)|` destructures the `&(u32, u32)` slice items into owned `u32`s.
> - Both functions return `u32`, so each carries `#[must_use]` (this does NOT trip `clippy::double_must_use`, which only fires on already-`#[must_use]` return types like `Result`).
> - The `.map(|&(w, h)| shapes::rectangle_area(w, h))` closure rearranges tuple fields into positional args, so it is NOT a redundant closure — `clippy::redundant_closure_for_method_calls` does not fire.
> - No `#[allow]` attributes should be needed. If clippy fires unexpectedly, fix the code rather than adding an allow, and report the deviation.

- [ ] **Step 4: Overwrite `lessons/15-modules/solutions/tests/exercise.rs`**

```rust
use modules_solutions::{rectangle_area, total_area};

// Warm-up: rectangle_area (leaf module geometry::shapes)

#[test]
fn warmup_area_basic() {
    assert_eq!(rectangle_area(3, 4), 12);
}

#[test]
fn warmup_area_zero() {
    assert_eq!(rectangle_area(0, 5), 0);
}

#[test]
fn warmup_area_square() {
    assert_eq!(rectangle_area(6, 6), 36);
}

#[test]
fn warmup_area_via_full_path() {
    // the full module path also works
    assert_eq!(modules_solutions::geometry::shapes::rectangle_area(2, 7), 14);
}

// Main: total_area (parent module geometry)

#[test]
fn main_total_empty() {
    let rects: [(u32, u32); 0] = [];
    assert_eq!(total_area(&rects), 0);
}

#[test]
fn main_total_one() {
    assert_eq!(total_area(&[(3, 4)]), 12);
}

#[test]
fn main_total_many() {
    assert_eq!(total_area(&[(2, 3), (4, 5), (1, 1)]), 27);
}

#[test]
fn main_total_via_module_path() {
    assert_eq!(modules_solutions::geometry::total_area(&[(10, 10)]), 100);
}
```

- [ ] **Step 5: Verify solution tests pass**

```bash
cargo test --package modules-solutions
```

Expected: 8 tests pass.

- [ ] **Step 6: Verify lint passes on the solutions crate**

```bash
cargo clippy --package modules-solutions --all-targets -- -D warnings
cargo fmt --check --package modules-solutions
```

Expected: both exit 0. No `#[allow]` attributes needed. If clippy fires on anything, fix the code (not with an allow) and report it.

- [ ] **Step 7: Commit**

```bash
git add lessons/15-modules/solutions
git commit -m "feat(lesson-15): add reference solutions"
```

---

## Task 4: Lesson README

**Files:**
- Overwrite: `lessons/15-modules/README.md`

- [ ] **Step 1: Overwrite `lessons/15-modules/README.md`**

The complete file content is below, delimited by an OUTER quadruple-backtick fence (` ```` `). That outer fence is ONLY a delimiter for this plan — do NOT write it into the file. The file must start with `# Lesson 15` on line 1 and contain only PLAIN triple-backtick (` ``` `) code fences.

````markdown
# Lesson 15 — Modules, crates, workspaces

As a program grows, you split it into modules — namespaces that group
related code and control what's visible. Modules map to files, files
group into a crate, and crates group into a workspace. This lesson is
hands-on with a real multi-file module tree, and it closes Phase 3.

## Learning goals

- Declare modules inline (`mod foo { ... }`) and as files (`mod foo;` →
  `foo.rs`, `pub mod bar;` → `foo/bar.rs`)
- Control visibility with `pub` — items are private by default, and the
  whole path must be public to reach an item from outside
- Refer to items by path (`crate::`, `self::`, `super::`) and bring
  names into scope with `use`
- Flatten a public API with `pub use` re-exports
- Explain the crate (a compilation unit) and workspace (a group of
  crates) picture

## Self-study notes

### Modules and the file mapping

A module is a named container for code. Declare it inline with braces,
or point at a file with `mod name;`:

```rust
mod geometry;        // in lib.rs: loads geometry.rs
```

```text
src/
├── lib.rs            // mod geometry;
├── geometry.rs       // pub mod shapes;
└── geometry/
    └── shapes.rs     // pub fn rectangle_area(...)
```

`mod geometry;` loads `geometry.rs`; `pub mod shapes;` inside it loads
`geometry/shapes.rs`. The module path mirrors the directory path. (This
is the modern form; older code used `mod.rs` files.)

### Visibility — `pub` and private-by-default

Items are **private by default** — visible only within their module and
its descendants. Add `pub` to expose them:

```rust
mod geometry {
    pub fn area() -> u32 { 0 }   // reachable from outside
    fn helper() {}               // private to `geometry`
}
```

The catch: the *whole path* must be public. A `pub fn` inside a private
`mod` is still unreachable from outside — the module needs `pub` too.

### Paths — `crate`, `self`, `super`, `use`

Refer to an item by its path. Absolute paths start at `crate`; relative
paths use `self` (here) and `super` (parent):

```rust
crate::geometry::shapes::rectangle_area(3, 4);
self::shapes::rectangle_area(3, 4);
use crate::geometry::shapes::rectangle_area;   // shortcut
rectangle_area(3, 4);                           // now callable by short name
```

`use` brings a name into scope so you can call it without the full path.

### Re-exports — `pub use`

`pub use` re-exports an item so callers can reach it by a short
crate-root path, even though it lives deep in the tree:

```rust
// in lib.rs:
pub use geometry::shapes::rectangle_area;
// callers can now write `my_crate::rectangle_area`
```

This is how libraries present a flat, friendly public API while keeping
their internals organized.

### Crates & workspaces

A **crate** is a compilation unit — a library (`lib.rs`) or a binary
(`main.rs`). A **workspace** groups several crates that share one
`Cargo.lock` and one `target/` directory. This course is itself a
workspace: `tools/*`, `lessons/*/exercises`, and `lessons/*/solutions`
are all member crates under the root `Cargo.toml`.

## Exercises

### Warm-up: `rectangle_area`

The exercise crate ships a real module tree. In
`src/geometry/shapes.rs`, implement the leaf function:

```rust
pub fn rectangle_area(width: u32, height: u32) -> u32 {
    // width * height
    todo!()
}
```

It's re-exported at the crate root, so tests reach it by both
`modules_exercises::rectangle_area` and the full
`modules_exercises::geometry::shapes::rectangle_area` path.

### Main: `total_area`

In `src/geometry.rs` (the parent module), implement `total_area`, which
calls the child module's function via a path and sums the results:

```rust
pub fn total_area(rects: &[(u32, u32)]) -> u32 {
    // rects.iter().map(|&(w, h)| shapes::rectangle_area(w, h)).sum()
    todo!()
}
```

Note the `shapes::rectangle_area` path — the parent reaching into its
child module.

### Compile-fail

`exercises/compile_fails/15-private-fn.rs` calls a function that has no
`pub`, from outside its module — the compiler rejects it (E0603 —
the function is private). Fix it by adding `pub` to the function.

### Run

```bash
make verify LESSON=15-modules
```

This runs your exercise tests and asserts the compile-fail file now
compiles.

## Solutions

See `solutions/src/` for the reference implementations. Try the
exercises before peeking.
````

- [ ] **Step 2: Spot-check the README**

```bash
head -1 lessons/15-modules/README.md
grep -c '^### ' lessons/15-modules/README.md
grep -c '^```' lessons/15-modules/README.md
```

Expected:
- First line: `# Lesson 15 — Modules, crates, workspaces`
- `grep -c '^### '` returns `9` (five subsections under self-study + four under exercises)
- `grep -c '^```'` returns `16` (8 code blocks × 2 fence lines — the "Modules and the file mapping" subsection has two blocks, a `rust` and a `text` tree; the "Crates & workspaces" and "Compile-fail" subsections are prose only)

If either count is wrong, the file content is off — re-check it against the content above and fix before committing.

- [ ] **Step 3: Commit**

```bash
git add lessons/15-modules/README.md
git commit -m "docs(lesson-15): write self-study notes"
```

---

## Task 5: Slide deck

**Files:**
- Overwrite: `lessons/15-modules/slides/slides.md`

- [ ] **Step 1: Overwrite `lessons/15-modules/slides/slides.md`**

The complete file content is below, delimited by an OUTER quadruple-backtick fence (` ```` `). That outer fence is ONLY a delimiter for this plan — do NOT write it into the file. The file must start with `# Modules, crates, workspaces` on line 1 and contain only PLAIN triple-backtick (` ``` `) code fences.

````markdown
# Modules, crates, workspaces

> As a program grows, you split it into modules — namespaces that group related code and control what's visible. Modules map to files, files group into a crate, and crates group into a workspace.

---

## Why modules

They organize code into namespaces, control visibility (public API vs internal), and prevent name clashes.

Everything you've written so far lived in one `lib.rs`. Real crates split into many modules.

---

## Defining modules

```rust
mod geometry {            // inline module
    pub fn area() -> u32 { 0 }
}

// or, in its own file:
mod geometry;             // looks for geometry.rs
```

A module is a named container. Write it inline with braces, or point at a file with `mod name;`.

---

## The file mapping

```text
src/
├── lib.rs            // mod geometry;
├── geometry.rs       // pub mod shapes;
└── geometry/
    └── shapes.rs     // pub fn rectangle_area(...)
```

`mod geometry;` loads `geometry.rs`. `pub mod shapes;` inside it loads `geometry/shapes.rs`. The module path mirrors the directory path.

---

## Visibility — `pub`

Items are **private by default** — visible only within their module and its descendants. Add `pub` to expose them.

The catch: the *whole path* must be public. A `pub fn` inside a private `mod` is still unreachable — both the module and the function need `pub`.

---

## Paths — `crate`, `self`, `super`, `use`

```rust
crate::geometry::shapes::rectangle_area(3, 4);  // from the crate root
self::shapes::rectangle_area(3, 4);             // relative to here
super::sibling_fn();                            // parent module
use crate::geometry::shapes::rectangle_area;    // bring it into scope
```

Absolute paths start at `crate`; relative paths use `self`/`super`. `use` creates a shortcut to call by the short name.

---

## Re-exports — `pub use`

```rust
// in lib.rs:
pub use geometry::shapes::rectangle_area;
```

`pub use` re-exports an item, so callers reach it by a short crate-root path (`my_crate::rectangle_area`) even though it lives deep in the tree. This is how libraries present a flat public API.

---

## Crates & workspaces

A **crate** is a compilation unit — a library (`lib.rs`) or a binary (`main.rs`).

A **workspace** groups several crates that share one `Cargo.lock` and one `target/`. *This course is a workspace*: `tools/*`, `lessons/*/exercises`, and `lessons/*/solutions` are all member crates.

---

## Putting it together

Today's exercise is a real module tree:

- **Warm-up** `rectangle_area` — a leaf function in `geometry/shapes.rs`
- **Main** `total_area` in `geometry.rs`, calling `shapes::rectangle_area` via a path

`lib.rs` re-exports both with `pub use`. The compile-fail shows a private function called from outside its module.

---

## Wrap — Phase 3 complete

- `mod` declares a module, inline or as a file
- The module tree mirrors the file tree
- Items are private until `pub`, and the whole path must be public
- Paths (`crate`/`self`/`super`) and `use` navigate the tree
- `pub use` flattens the public API; crates group into workspaces

Next: **Phase 4 — Lesson 16, Threads & channels** (concurrency).
````

- [ ] **Step 2: Verify `make slides-build` succeeds and includes lesson 15**

```bash
make slides-build
test -f dist/lessons/15-modules/slides/slides.md
test -f dist/lessons/15-modules/slides/index.html
grep -c "15-modules" dist/index.html
```

Expected: `slides.md` and `index.html` copied into dist; `grep -c "15-modules"` returns at least 1. (The build-index master registry already has lesson 15 registered with slug `modules`, matching this directory, so it renders as a clickable link.)

- [ ] **Step 3: Spot-check slide separators**

```bash
grep -c '^---$' lessons/15-modules/slides/slides.md
```

Expected: `9` (between 10 slides).

- [ ] **Step 4: Commit**

```bash
git add lessons/15-modules/slides/slides.md
git commit -m "feat(lesson-15): write slide deck"
```

---

## Task 6: End-to-end verification + push

- [ ] **Step 1: `make ci` is green**

```bash
make ci
```

Expected: exit 0. Clippy clean, fmt clean, workspace builds, default-members tests pass (now includes the 8 new tests in `modules-solutions`), compile-fail `--expect broken` passes for lesson 15.

- [ ] **Step 2: `make verify LESSON=15-modules` fails (the exercise is undone — intentional)**

```bash
make verify LESSON=15-modules || echo "expected: exercise tests fail with todo!() panic"
```

Expected: non-zero exit. All 8 exercise tests panic with `not yet implemented`.

- [ ] **Step 3: `make slides-build` final state**

```bash
make slides-build
ls dist/lessons/
grep -c "15-modules" dist/index.html
```

Expected: `dist/lessons/` contains all fifteen lessons. `grep -c "15-modules"` ≥ 1.

- [ ] **Step 4: Push**

```bash
git push
```

Expected: push succeeds. CI runs and is green; Deploy rebuilds the static site so lesson 15 appears live.

- [ ] **Step 5: Smoke-test the deployed site**

After the push, wait for the Deploy workflow to finish (`gh run watch <id>`). Then:

```bash
curl -sS -o /dev/null -w "%{http_code}\n" https://rust.ristkari.dev/
curl -sS -o /dev/null -w "%{http_code}\n" https://rust.ristkari.dev/lessons/15-modules/slides/
```

Expected: both return `200`.

---

## Done criteria

- `lessons/15-modules/` exists with all four parts; both crates carry the multi-file `src/` tree (`lib.rs`, `geometry.rs`, `geometry/shapes.rs`)
- Both `exercises/src/` and `solutions/src/` define the same module tree and the `rectangle_area` / `total_area` signatures (exercise ships `todo!()` bodies, solution ships real bodies)
- `cargo test --package modules-solutions` → 8 passing tests
- `cargo test --manifest-path lessons/15-modules/exercises/Cargo.toml` → compiles, 8 panicking tests (intentional)
- `cargo run --package compile-fails -- --expect broken lessons/15-modules` → ok
- `cargo run --package compile-fails -- --expect compiles lessons/15-modules` → fails (intentional)
- `make ci` → green
- `make slides-build` → produces `dist/lessons/15-modules/slides/index.html`
- `dist/index.html` lists lesson 15 as a clickable link
- All changes committed and pushed (plain commit messages, no co-author trailer)
- Deployed site returns HTTP 200 for `/` and `/lessons/15-modules/slides/`
