# Lesson 15 — Modules, crates, workspaces — design

The fourth and final lesson of Phase 3 (Abstraction). How Rust code is
organized: `mod` (with the name→file mapping people actually trip on),
`pub` visibility, paths (`crate::`/`self::`/`super::`/`use`), `pub use`
re-exports, and the crate/workspace picture. Unlike every prior lesson,
the exercise crate is genuinely **multi-file** so students see the
module-to-file mapping firsthand. Closes Phase 3.

## Audience and prerequisites

- Has completed Lessons 01-14
- Comfortable with functions, structs, traits, iterators (L11), and
  building/testing with Cargo
- Has the toolchain installed and `make verify` working

## Learning goals

By the end of this lesson, the student can:

1. Declare modules both inline (`mod foo { ... }`) and as separate files
   (`mod foo;` → `foo.rs`, `pub mod bar;` → `foo/bar.rs`)
2. Control visibility with `pub` — items are private by default, and the
   *entire path* to an item must be public to reach it from outside
3. Refer to items by path (`crate::`, `self::`, `super::`) and bring
   names into scope with `use`
4. Flatten a public API with `pub use` re-exports
5. Explain the crate (a compilation unit — library or binary) and
   workspace (a group of crates sharing one lockfile and `target/`)
   picture — this course is itself a Cargo workspace

## Scope

In scope: declaring modules inline and as files; the module→file mapping
(`mod foo;`/`foo.rs` and `pub mod bar;`/`foo/bar.rs`); `pub` visibility
and the rule that the whole path must be public; paths with `crate`,
`self`, `super`; `use` to bring names into scope; `pub use` re-exports;
the crate-vs-workspace picture (conceptual, with this repo as the
example). The exercise builds a real multi-file module tree: a leaf
function (warm-up) and a parent-module function that calls the leaf via a
path (main), re-exported at the crate root.

Out of scope (deferred or skipped): `pub(crate)`/`pub(super)`/
`pub(in path)` restricted visibility (mentioned at most in passing —
plain `pub` vs private is the lesson); the 2015-edition `mod.rs` file
convention (we teach only the modern `foo.rs` + `foo/` form); external
dependencies and `Cargo.toml` `[dependencies]` mechanics (covered in
L14); publishing to crates.io; `cfg`/feature-gated modules; glob imports
(`use foo::*`) beyond a caution; binary-plus-library crate layout
(`main.rs` + `lib.rs`) beyond a mention; creating a workspace from
scratch (shown conceptually via this repo, not built). Modules are
introduced as *organize code + control visibility*; the packaging and
publishing depth waits for the tooling phase.

## Slide arc (10 slides)

1. **Title — Modules, crates, workspaces.** Hook: *"As a program grows,
   you split it into modules — namespaces that group related code and
   control what's visible. Modules map to files, files group into a
   crate, and crates group into a workspace."*
2. **Why modules.** They organize code into namespaces, control
   visibility (what's public API vs internal), and prevent name
   clashes. Everything you've written so far lived in one `lib.rs`; real
   crates split into many modules.
3. **Defining modules.**
   ```rust
   mod geometry {            // inline module
       pub fn area() -> u32 { 0 }
   }

   // or, in its own file:
   mod geometry;             // looks for geometry.rs
   ```
   A module is a named container. You can write it inline with braces,
   or point at a file with `mod name;`.
4. **The file mapping.**
   ```text
   src/
   ├── lib.rs            // mod geometry;
   ├── geometry.rs       // pub mod shapes;
   └── geometry/
       └── shapes.rs     // pub fn rectangle_area(...)
   ```
   `mod geometry;` in `lib.rs` loads `geometry.rs`. `pub mod shapes;`
   inside it loads `geometry/shapes.rs`. The module path mirrors the
   directory path. (This is the modern form; older code uses `mod.rs`.)
5. **Visibility — `pub`.** Items are **private by default** — visible
   only within their module and descendants. Add `pub` to expose them.
   The catch: the *whole path* must be public. A `pub fn` inside a
   private `mod` is still unreachable from outside — both the module and
   the function need `pub`.
6. **Paths — `crate`, `self`, `super`, `use`.**
   ```rust
   crate::geometry::shapes::rectangle_area(3, 4);  // from the crate root
   self::shapes::rectangle_area(3, 4);             // relative to here
   super::sibling_fn();                            // parent module
   use crate::geometry::shapes::rectangle_area;    // bring it into scope
   ```
   An absolute path starts at `crate`; relative paths use `self`/`super`.
   `use` creates a shortcut so you can call by the short name.
7. **Re-exports — `pub use`.**
   ```rust
   // in lib.rs:
   pub use geometry::shapes::rectangle_area;
   ```
   `pub use` re-exports an item, so callers can reach it by a short
   crate-root path (`my_crate::rectangle_area`) even though it lives deep
   in the tree. This is how libraries present a flat, friendly public
   API.
8. **Crates & workspaces.** A **crate** is a compilation unit — a
   library (`lib.rs`) or a binary (`main.rs`). A **workspace** groups
   several crates that share one `Cargo.lock` and one `target/`
   directory. *This course is a workspace*: `tools/*`,
   `lessons/*/exercises`, and `lessons/*/solutions` are all member
   crates under the root `Cargo.toml`.
9. **Putting it together.** Walk through the exercise's real module tree:
   `geometry/shapes.rs` holds `rectangle_area` (warm-up — a leaf
   function), `geometry.rs` holds `total_area` which calls
   `shapes::rectangle_area` via a path (main), and `lib.rs` re-exports
   both with `pub use`. The compile-fail shows a private function being
   called from outside its module.
10. **Wrap — Phase 3 complete.** Five takeaways: `mod` declares a
    module, inline or as a file; the module tree mirrors the file tree;
    items are private until `pub`, and the whole path must be public;
    paths (`crate`/`self`/`super`) and `use` navigate the tree;
    `pub use` flattens the public API; crates group into workspaces.
    Next: **Phase 4 — Lesson 16, Threads & channels** (concurrency).

## Exercise spec

`lessons/15-modules/` follows the standard four-part lesson shape, but
the exercise and solution crates each carry a **multi-file** `src/`
module tree:

```
15-modules/
├── README.md
├── slides/
│   ├── index.html
│   └── slides.md
├── exercises/
│   ├── Cargo.toml
│   ├── src/
│   │   ├── lib.rs               # pub mod geometry; + pub use re-exports
│   │   ├── geometry.rs          # pub mod shapes; + total_area (main)
│   │   └── geometry/
│   │       └── shapes.rs        # rectangle_area (warm-up)
│   ├── tests/exercise.rs
│   └── compile_fails/15-private-fn.rs
└── solutions/
    ├── Cargo.toml
    ├── src/
    │   ├── lib.rs
    │   ├── geometry.rs
    │   └── geometry/
    │       └── shapes.rs
    └── tests/exercise.rs
```

Cargo package names: `modules-exercises` and `modules-solutions` (the
lesson's "bare" name is `modules`; the import idents are
`modules_exercises` / `modules_solutions`). This matches the build-index
master registry slug `modules`, so the landing page links it without any
change. The scaffolder generates only `src/lib.rs`; the implementer
creates `src/geometry.rs` and `src/geometry/shapes.rs` in both crates.

### Exercise stub

The module structure — the `mod`/`pub mod` declarations, the `pub use`
re-exports, and the function signatures — ships **complete**. Students
fill the two function bodies (`rectangle_area`, `total_area`). Because
the structure is complete, the crate and its tests *compile* in the
undone state; the tests fail at runtime with the `todo!()` panic, like
every prior lesson.

`exercises/src/lib.rs`:

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

`exercises/src/geometry.rs`:

```rust
//! Geometry helpers (the parent module).

pub mod shapes;

#[must_use]
pub fn total_area(_rects: &[(u32, u32)]) -> u32 {
    todo!("sum the area of every (width, height) rectangle using shapes::rectangle_area")
}
```

`exercises/src/geometry/shapes.rs`:

```rust
//! Rectangle area calculations (a leaf module).

#[must_use]
pub fn rectangle_area(_width: u32, _height: u32) -> u32 {
    todo!("return the rectangle's area: width times height")
}
```

### Warm-up: `rectangle_area`

Reference solution (`solutions/src/geometry/shapes.rs`):

```rust
//! Rectangle area calculations (a leaf module).

#[must_use]
pub fn rectangle_area(width: u32, height: u32) -> u32 {
    width * height
}
```

Pedagogical packing: a `pub` leaf function living two modules deep
(`crate::geometry::shapes`). Simple body; the lesson is *where it lives*
and *how it's reached*. It is re-exported at the crate root, so callers
can use either `crate_root::rectangle_area` or the full
`crate_root::geometry::shapes::rectangle_area` path.

### Main: `total_area`

Reference solution (`solutions/src/geometry.rs`):

```rust
//! Geometry helpers (the parent module).

pub mod shapes;

#[must_use]
pub fn total_area(rects: &[(u32, u32)]) -> u32 {
    rects.iter().map(|&(w, h)| shapes::rectangle_area(w, h)).sum()
}
```

Pedagogical packing: a parent-module function that reaches into its
child module via the `shapes::` path to call `rectangle_area`, then sums
with an iterator chain (reuses L11). The `|&(w, h)|` destructures the
`&(u32, u32)` slice items into owned `u32`s. Returns `u32`, so
`#[must_use]` is appropriate (unlike L14's `Result`-returning
functions).

`solutions/src/lib.rs` is identical to the exercise `lib.rs` minus the
"exercises" wording in the doc comment:

```rust
//! Lesson 15 — reference solutions.

pub mod geometry;

pub use geometry::shapes::rectangle_area;
pub use geometry::total_area;
```

### Tests

Both `exercises/tests/exercise.rs` and `solutions/tests/exercise.rs`
exercise the re-exported short paths *and* the full module paths, so
students see both ways to reach an item. (Replace `modules_exercises`
with `modules_solutions` in the solutions copy.)

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

**Eight tests total** (four warm-up + four main). Test arithmetic:
`main_total_many` → `2*3 + 4*5 + 1*1 = 6 + 20 + 1 = 27`.

### Compile-fail: `15-private-fn.rs`

Path: `exercises/compile_fails/15-private-fn.rs`. A self-contained file
with an inline module whose function lacks `pub`, called from `main`.
Ships broken; the student adds `pub` until it compiles.

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

Pass condition: the student adds `pub` to `fn rectangle_area`. rustc
reports E0603 "function `rectangle_area` is private" and points at the
item — verified during design. After the fix the file compiles.

This is the lesson's centerpiece for visibility: privacy is the default,
and `pub` is the deliberate act of exposing an item.

## README structure

`lessons/15-modules/README.md` follows the established shape:

- **Title + one-paragraph hook**
- **Learning goals** — the five bullets above
- **Self-study notes** with five subsections:
  - Modules and the file mapping
  - Visibility — `pub` and private-by-default
  - Paths — `crate`, `self`, `super`, `use`
  - Re-exports — `pub use`
  - Crates & workspaces
- **Exercises** — four subsections: Warm-up (`rectangle_area`), Main
  (`total_area`), Compile-fail, Run
- **Solutions** — pointer to `solutions/src/`

Each `###` subsection runs ~4-6 sentences plus a small code block. The
"Modules and the file mapping" and "Visibility" sections are the
heaviest — they carry the lesson's core.

## Lint expectations

Lesson 15's reference solution code is clippy-clean (with `clippy::all`
+ `clippy::pedantic` denied) without `#[allow]` attributes — verified
during design (the full multi-file tree, tests, and stub all pass
`cargo clippy --all-targets`):

- `rectangle_area` and `total_area` return `u32`, so each carries
  `#[must_use]` (unlike L14's `Result`-returning functions). This does
  not trip `double_must_use`.
- `total_area` uses `.map(|&(w, h)| shapes::rectangle_area(w, h)).sum()`
  — the closure rearranges tuple fields into positional args, so it is
  not a redundant closure and `clippy::redundant_closure_for_method_calls`
  does not fire.
- The `pub mod` declarations and `pub use` re-exports are all used (the
  re-exports reference the items, the tests reference the re-exports), so
  no `unused` warnings.
- In the *exercise stub*, the two `todo!()` bodies with unused
  `_width`/`_height`/`_rects` params compile and lint clean (verified).

If clippy fires on anything unexpected, fix the code rather than adding
an allow, and report it.

## Done criteria

- `lessons/15-modules/` exists with the four-part structure, and both
  crates carry the multi-file `src/` tree (`lib.rs`, `geometry.rs`,
  `geometry/shapes.rs`)
- Cargo manifests use the correct package names (`modules-exercises`,
  `modules-solutions`)
- `exercises/src/` and `solutions/src/` define the same module tree and
  the `rectangle_area` / `total_area` signatures; the exercise ships
  `todo!()` bodies, the solution ships real bodies
- `cargo test --package modules-solutions` → 8 tests pass
- `cargo test --manifest-path lessons/15-modules/exercises/Cargo.toml`
  → compiles, all 8 tests panic with `not yet implemented` (the intended
  undone state)
- `cargo run --package compile-fails -- --expect broken lessons/15-modules`
  → ok
- `cargo run --package compile-fails -- --expect compiles lessons/15-modules`
  → fails (file ships broken, that's the point)
- `make ci` is green
- `make slides-build` produces `dist/lessons/15-modules/slides/index.html`
- `dist/index.html` lists lesson 15 as a clickable link (registry slug
  `modules` already matches this directory name)
- One push to `origin/main` triggers a green CI run and a green Deploy
  run; `https://rust.ristkari.dev/lessons/15-modules/slides/` returns 200

## Open questions

None.
