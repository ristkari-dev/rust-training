# Lesson 26 — Testing — Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Author the first lesson of Phase 7: testing. Rust puts a test in one of three homes — `#[cfg(test)] mod tests` (sees private items), `tests/*.rs` (a separate crate, public API only), and `///` examples (documentation that is compiled and run) — and the lesson's thesis is that examples and properties catch different bugs. Warm-up (inverted: the *tests* are the deliverable): two unit tests on a private `run_len`, two doc tests on `encode`. Main: implement `decode`, graded by two examples plus a `proptest` property, with a fourth test acting as a tripwire against deleting the warm-up. Compile-fail: calling a `#[cfg(test)]` function from normal code (E0425).

**Architecture:** One domain for the whole lesson — run-length encoding, where each run of identical characters becomes `<count><char>` (`"aaab"` → `"3a1b"`, twelve `a`s → `"12a"`). `encode` and the private `run_len` ship complete in both crates; the student writes the four warm-up tests and `decode`. Four graded tests live in `tests/exercise.rs` as usual; the other four live in `src/lib.rs` (two in `#[cfg(test)] mod tests`, two in `encode`'s doc comment) because they are the warm-up deliverable. This works only because the root `Cargo.toml` sets `default-members = ["tools/*", "lessons/*/solutions"]`: `make test` runs the solutions' unit and doc tests (verifying the reference answers) and never the exercise stubs, while `make verify LESSON=26-testing` runs the exercise crate's own. `proptest` joins `[workspace.dependencies]` and becomes the repo's first dev-dependency. No external service, no C toolchain.

**Tech Stack:** Rust 2024 edition (toolchain 1.98), `proptest` 1.11.0 (dev-dependency), existing tools (`new-lesson`, `compile-fails`, `slides-dev`, `build-index`), reveal.js (vendored), GNU Make.

**Spec:** [`docs/superpowers/specs/2026-09-22-lesson-26-testing-design.md`](../specs/2026-09-22-lesson-26-testing-design.md).

**Working directory:** `/Users/ristkari/code/private/rust-training`.

**Commit convention:** Plain commit messages only — no `Co-Authored-By` trailer or any AI attribution. If a commit fails with a GPG/pinentry error, simply retry the same `git commit` command once or twice.

## Global Constraints

- Lesson directory: `lessons/26-testing/`; Cargo package names `testing-exercises` and `testing-solutions` (import idents `testing_exercises` / `testing_solutions`).
- Root `Cargo.toml` `[workspace.dependencies]` gains exactly one entry: `proptest = "1"`, inserted in alphabetical position.
- Both lesson crates declare it in a **`[dev-dependencies]`** section after `[lints]` — `proptest = { workspace = true }`. This is the repo's first `[dev-dependencies]` section and it is deliberate: it is a teaching point. Do NOT put proptest in `[dependencies]`.
- Workspace lints deny `clippy::all` + `clippy::pedantic`, `rust_2018_idioms` and `unused`. No `#[allow]` attributes anywhere.
- **The unit-test stub ships without `use super::*;` and with a bare `todo!("…")` as the whole test body.** Any other stub form fails `make lint`: `let expected: T = todo!(…)` trips `unused_variables` + `clippy::diverging_sub_expression` + unreachable code, and adding `use super::*;` to a bare-`todo!()` module trips `-D unused-imports`. Both verified during planning.
- **The doc-test stubs use `let expected: &str = todo!(…);` followed by the assertion.** Clippy never compiles doc tests, so the richer form is safe there and only there. `assert_eq!(x, todo!())` does NOT compile (E0277 `String == !`) — do not use it.
- `#[must_use]` on `encode` and `decode` (`clippy::must_use_candidate` is pedantic and denied).
- The `decode` stub takes `_input`, renamed by the student when they write the body — the Lesson 24/25 convention.
- `failure_persistence: None` in the `proptest_config` is mandatory: without it a failing run writes an untracked `.proptest-regressions` file into the student's tree.
- The proptest strategy is `"[^0-9]{0,30}"`, never `".*"` — the round-trip property is false for inputs containing digits, so this is exactly its domain. Verified: the reference passes 25/25 runs, and this strategy (unlike `"[a-z]"`) catches a byte-indexing `decode` with `minimal failing input: s = "¡"`.
- Compile-fail files are std-only (the tool runs bare `rustc` with no `--extern`).
- Out of scope — do not add: clippy/rustfmt/miri content (Lesson 27), benchmarking or `criterion` (Lesson 28), coverage, fuzzing, mutation testing, `insta`, `rstest`, mocking frameworks, `cargo-nextest`, custom harnesses.
- All code in this plan was verified on rustc 1.98.1 in a scratch crate carrying the workspace's exact `[lints]`: solutions 8/8 pass (2 unit + 4 integration + 2 doc), the stub compiles clippy- and rustfmt-clean with 7 failing and the `warmup_all_four_tests_are_still_there` tripwire passing, and no `.proptest-regressions` file is produced. If clippy or rustfmt fires, do NOT add an `#[allow]` and do NOT change the code; STOP and report the exact output.

## Deviations from the spec

- **The spec's "8 tests in `tests/exercise.rs`" house shape is split 4/4 across two files.** `tests/exercise.rs` holds the four main tests; the four warm-up tests live in `src/lib.rs` because they are the deliverable, not the grader. The spec already describes this; it is called out here so a reviewer does not read it as drift.
- **`cargo test` stops at the first failing target.** Verified during planning: with the shipped stub, `cargo test` reports only the two unit-test failures and never reaches the integration or doc tests. The student therefore meets the work in cargo's fixed target order — unit tests, then `decode`, then the doc tests — which is NOT the order the README introduces them in. The README must say so explicitly and name `--lib` / `--test exercise` / `--doc` for picking one target. This behaviour was not anticipated in the spec.
- **Three adversarial reviews ran against this plan before execution and their findings are already folded in.** The graded tests changed as a result: `decode("12a")` became `decode("1a12b")` (the old input let a decoder that parsed a multi-digit count only at the start of the string pass everything), the strategy became `"[^0-9]{0,30}"` (the old `"[a-z]"` could not catch a byte-indexing `decode`), and `main_expands_an_empty_string` was replaced by `warmup_all_four_tests_are_still_there` (the old test was fully subsumed by the property, and deleting the warm-up tests was a silent pass). Prose claims about doc-test compilation, `use super::*;`, and the `#`-hiding rule were corrected against measured behaviour. Do not "restore" any of these.
- **The deck adds `#[should_panic]`/`#[ignore]`/`--nocapture` to slide 6** and both README and deck add `--no-fail-fast`, which the spec's slide arc did not list.
- **On a firing lint, STOP instead of fixing.** This plan's code was verified clippy- and rustfmt-clean on rustc 1.98.1 against proptest 1.11.0, so a firing lint means toolchain or dependency drift worth reporting, not patching.

---

## Task 1: Scaffold lessons/26-testing

**Files (all created by the scaffolder):**
- `lessons/26-testing/README.md` (placeholder, replaced in Task 5)
- `lessons/26-testing/slides/index.html` (final — no edit needed)
- `lessons/26-testing/slides/slides.md` (placeholder, replaced in Task 6)
- `lessons/26-testing/exercises/Cargo.toml` (dev-dependency added in Task 2)
- `lessons/26-testing/exercises/src/lib.rs` (placeholder, replaced in Task 3)
- `lessons/26-testing/exercises/tests/exercise.rs` (placeholder, replaced in Task 3)
- `lessons/26-testing/solutions/Cargo.toml` (dev-dependency added in Task 2)
- `lessons/26-testing/solutions/src/lib.rs` (placeholder, replaced in Task 4)
- `lessons/26-testing/solutions/tests/exercise.rs` (placeholder, replaced in Task 4)

**Interfaces:**
- Consumes: nothing.
- Produces: workspace members `testing-exercises` and `testing-solutions`.

- [ ] **Step 1: Run the scaffolder**

```bash
make new-lesson NAME=26-testing
```

Expected: `scaffolded ./lessons/26-testing`.

- [ ] **Step 2: Verify Cargo package names**

```bash
grep '^name' lessons/26-testing/exercises/Cargo.toml lessons/26-testing/solutions/Cargo.toml
```

Expected:
```
lessons/26-testing/exercises/Cargo.toml:name = "testing-exercises"
lessons/26-testing/solutions/Cargo.toml:name = "testing-solutions"
```

- [ ] **Step 3: Verify the workspace picks up the new crates**

```bash
cargo metadata --no-deps --format-version 1 | grep -o '"name":"testing-[^"]*"' | sort -u
```

Expected output:
```
"name":"testing-exercises"
"name":"testing-solutions"
```

- [ ] **Step 4: Verify the scaffolded workspace builds clean**

```bash
cargo build --workspace
```

Expected: warning-free build.

- [ ] **Step 5: Commit**

```bash
git add lessons/26-testing
git commit -m "chore: scaffold lessons/26-testing"
```

---

## Task 2: Add proptest as the repo's first dev-dependency

**Files:**
- Modify: `Cargo.toml` (root — `[workspace.dependencies]`)
- Modify: `lessons/26-testing/exercises/Cargo.toml`
- Modify: `lessons/26-testing/solutions/Cargo.toml`

**Interfaces:**
- Consumes: the two crates from Task 1.
- Produces: `proptest` resolvable as `{ workspace = true }` from both crates' `[dev-dependencies]` — available anywhere the crate is compiled for testing (`tests/`, `#[cfg(test)]` modules and doc tests alike), but not from a normal build of `src/lib.rs`, and never to anyone who depends on the crate. (Verified: a `proptest!` block inside `#[cfg(test)] mod tests` and a `use proptest::prelude::*;` inside a doc test both compile.)

- [ ] **Step 1: Add proptest to the workspace dependencies**

In the root `Cargo.toml`, inside the existing `[workspace.dependencies]` table, add this line in alphabetical position (between `clap` and `serde_json`):

```toml
proptest = "1"
```

Leave every other entry untouched.

- [ ] **Step 2: Add the dev-dependency to both lesson crates**

Append to **both** `lessons/26-testing/exercises/Cargo.toml` and `lessons/26-testing/solutions/Cargo.toml`, after the existing `[lints]` section:

```toml
[dev-dependencies]
proptest = { workspace = true }
```

Note the section name: `[dev-dependencies]`, not `[dependencies]`. `proptest` is used only from `tests/exercise.rs`; putting it in `[dependencies]` would contradict the README and the slides.

- [ ] **Step 3: Verify it resolves and the version is what the plan assumes**

```bash
cargo tree --package testing-solutions --depth 1
```

Expected: a `proptest v1.11.x` line marked as a dev-dependency. If the major version is not 1, STOP and report.

- [ ] **Step 4: Verify the workspace still builds with all targets**

```bash
cargo build --workspace --all-targets
```

Expected: warning-free build (this is the step that first compiles proptest's tree).

- [ ] **Step 5: Verify proptest is NOT a normal dependency of the library**

```bash
cargo tree --package testing-solutions --edges normal --depth 1
```

Expected: `proptest` does **not** appear. If it does, it was put in `[dependencies]` — fix it.

- [ ] **Step 6: Commit**

```bash
git add Cargo.toml lessons/26-testing/exercises/Cargo.toml lessons/26-testing/solutions/Cargo.toml
git commit -m "feat(lesson-26): add proptest as a dev-dependency"
```

---

## Task 3: Exercise content (stub, tests, compile-fail)

**Files:**
- Replace: `lessons/26-testing/exercises/src/lib.rs`
- Replace: `lessons/26-testing/exercises/tests/exercise.rs`
- Create: `lessons/26-testing/exercises/compile_fails/26-cfg-test-not-compiled.rs`

**Interfaces:**
- Consumes: the crates and dev-dependency from Tasks 1-2.
- Produces: `pub fn encode(&str) -> String` (given), `pub fn decode(&str) -> String` (stubbed), private `fn run_len(&str) -> usize` (given). Task 4 mirrors all of it with the answers filled in.

- [ ] **Step 1: Write the exercise stub**

Replace `lessons/26-testing/exercises/src/lib.rs` entirely with:

```rust
//! Lesson 26 — exercises.
//!
//! This lesson turns the harness around: the warm-up is the *tests*, and the
//! main exercise is the code. `encode` and `run_len` are given; you write
//! two unit tests, two doc tests, and `decode`.
//!
//! Run `make verify LESSON=26-testing`. Note that `cargo test` stops at the
//! first target that fails, so you will meet the work in this order: the
//! unit tests below, then `decode`, then the doc tests on `encode`.

/// Run-length encode: every run of identical characters becomes
/// `<count><char>`.
///
/// ```
/// use testing_exercises::encode;
/// let expected: &str = todo!("what does encode(\"aaab\") return?");
/// assert_eq!(encode("aaab"), expected);
/// ```
///
/// A run of ten or more is still one run — the count just takes two digits:
///
/// ```
/// use testing_exercises::encode;
/// let expected: &str = todo!("what does encode(\"aaaaaaaaaaaa\") return?");
/// assert_eq!(encode("aaaaaaaaaaaa"), expected);
/// ```
#[must_use]
pub fn encode(input: &str) -> String {
    let mut out = String::new();
    let mut rest = input;
    while let Some(ch) = rest.chars().next() {
        let n = run_len(rest);
        out.push_str(&n.to_string());
        out.push(ch);
        rest = &rest[n * ch.len_utf8()..];
    }
    out
}

/// Expand a run-length encoded string: `"3a1b"` becomes `"aaab"`.
///
/// Counts can run to more than one digit, so keep reading digits until you
/// reach the character they belong to.
// The `_` prefix keeps the unfinished stub compiling (an unused variable is
// a compile error in this course). Rename it when you write the body.
#[must_use]
pub fn decode(_input: &str) -> String {
    todo!("expand each `<count><char>` run back into characters")
}

/// Length of the leading run of identical characters.
fn run_len(input: &str) -> usize {
    let Some(first) = input.chars().next() else {
        return 0;
    };
    input.chars().take_while(|c| *c == first).count()
}

#[cfg(test)]
mod tests {
    // Add `use super::*;` in the same edit as your first assertion: on its
    // own it is an unused import, which is a compile error in this course.
    // It is a convenience, not the access - this module can already see its
    // parent's private items, and `super::run_len(..)` works without it.

    #[test]
    fn warmup_run_len_counts_the_leading_run() {
        todo!("assert what run_len(\"aaab\") returns")
    }

    #[test]
    fn warmup_run_len_of_an_empty_string_is_zero() {
        todo!("assert what run_len(\"\") returns")
    }
}
```

- [ ] **Step 2: Write the main tests**

Replace `lessons/26-testing/exercises/tests/exercise.rs` entirely with:

```rust
use proptest::prelude::*;
use testing_exercises::{decode, encode};

#[test]
fn main_expands_each_run() {
    assert_eq!(decode("3a1b"), "aaab");
}

#[test]
fn main_reads_multi_digit_counts() {
    assert_eq!(
        decode("1a12b"),
        "abbbbbbbbbbbb",
        "a count can run to more than one digit, anywhere in the string - keep reading digits until the character"
    );
}

proptest! {
    #![proptest_config(ProptestConfig { failure_persistence: None, ..ProptestConfig::default() })]

    /// Whatever `encode` produced, `decode` must turn back into the original.
    ///
    /// `[^0-9]` is not an arbitrary alphabet: it is exactly the domain where
    /// this property holds, because a digit in the input makes
    /// `<count><char>` ambiguous.
    #[test]
    fn main_round_trips_whatever_encode_produced(s in "[^0-9]{0,30}") {
        prop_assert_eq!(decode(&encode(&s)), s);
    }
}

/// The warm-up tests are yours to write - but not yours to delete.
#[test]
fn warmup_all_four_tests_are_still_there() {
    let src = include_str!("../src/lib.rs");
    assert!(
        src.contains("#[cfg(test)]"),
        "the warm-up unit test module is gone - deleting a test is not passing it"
    );
    assert_eq!(
        src.matches("/// ```").count(),
        4,
        "both doc-test examples on `encode` must stay - deleting a test is not passing it"
    );
}
```

Three of these four are load-bearing in a way that was verified by attacking them, and the fourth is a tripwire:

- `decode("1a12b")` puts the multi-digit count in the **second** run, not the first. With the earlier `decode("12a")`, a decoder that parsed a multi-digit count only at the start of the string and single digits thereafter passed every test.
- The strategy is `"[^0-9]{0,30}"` — the exact domain where the round-trip holds — not `"[a-z]{0,30}"`. Over `[a-z]` the property could not catch a byte-indexing `decode` that breaks on any non-ASCII character; over `[^0-9]` it catches it and reports `minimal failing input: s = "¡"`.
- `warmup_all_four_tests_are_still_there` exists because the warm-up tests live in a file the student edits, so deleting them was a silent pass: `cargo test` reported `running 0 tests ... ok` and `make verify` exited 0.

- [ ] **Step 3: Write the compile-fail file**

Create `lessons/26-testing/exercises/compile_fails/26-cfg-test-not-compiled.rs`:

```rust
// Compile-fail exercise: this file MUST NOT compile until you fix it.
//
// `#[cfg(test)]` means "compile this only when building tests". In a normal
// build the item is not merely unused - it does not exist at all, so
// anything that calls it fails to resolve. rustc reports E0425 ("cannot
// find function") and then points straight at the gate: "found an item that
// was configured out".
//
// That is the whole reason unit tests can sit beside your code without ever
// being shipped in it.
//
// The fix: `double` is called by ordinary, non-test code, so it is not
// test-only. Delete the `#[cfg(test)]` line above it.

#[cfg(test)]
fn double(x: i32) -> i32 {
    x * 2
}

pub fn quadruple(x: i32) -> i32 {
    double(double(x))
}
```

- [ ] **Step 4: Verify the stub compiles and lints clean**

```bash
cargo clippy --package testing-exercises --all-targets -- -D warnings
cargo fmt --check -p testing-exercises
```

Expected: both silent. If clippy fires, STOP and report — do not add `#[allow]`.

- [ ] **Step 5: Verify all 8 tests fail**

`cargo test` stops at the first failing target, so check each target separately:

```bash
cargo test --manifest-path lessons/26-testing/exercises/Cargo.toml --lib 2>&1 | tail -5
cargo test --manifest-path lessons/26-testing/exercises/Cargo.toml --test exercise 2>&1 | tail -5
cargo test --manifest-path lessons/26-testing/exercises/Cargo.toml --doc 2>&1 | tail -5
```

Expected: `2 failed`; `1 passed; 3 failed`; `2 failed` — 7 failing in total. The one that passes is `warmup_all_four_tests_are_still_there`, and it is *supposed* to pass here: it is a tripwire that only trips if the student deletes the warm-up tests, and the shipped stub still has them.

- [ ] **Step 6: Verify the compile-fail file is broken as shipped**

```bash
cargo run --package compile-fails -- --expect broken lessons/26-testing
```

Expected: pass (the file does not compile, which is what "broken" asserts).

- [ ] **Step 7: Verify the compile-fail file compiles once fixed**

Run this as one block — the shell variable must survive between commands, and no temp file is needed:

```bash
CF=lessons/26-testing/exercises/compile_fails/26-cfg-test-not-compiled.rs
ORIG=$(cat "$CF")
sed -i '' '/^#\[cfg(test)\]$/d' "$CF"
cargo run --package compile-fails -- --expect compiles lessons/26-testing
printf '%s\n' "$ORIG" > "$CF"
git diff --stat lessons/26-testing/exercises/compile_fails/
```

Expected: the `--expect compiles` run passes (the file compiles once the gate is gone), and the final `git diff --stat` prints **nothing** — the restore was exact. `sed -i ''` is BSD/macOS syntax and the pattern matches exactly one line. If `git diff --stat` shows a change, STOP and restore the file from the plan text above before committing.

- [ ] **Step 8: Verify no proptest droppings**

```bash
find lessons/26-testing -name '*.proptest-regressions'
```

Expected: no output.

- [ ] **Step 9: Commit**

```bash
git add lessons/26-testing/exercises
git commit -m "feat(lesson-26): add exercise stubs, tests, and compile-fail"
```

---

## Task 4: Reference solutions

**Files:**
- Replace: `lessons/26-testing/solutions/src/lib.rs`
- Replace: `lessons/26-testing/solutions/tests/exercise.rs`

**Interfaces:**
- Consumes: the API from Task 3 — `encode`, `decode`, private `run_len`.
- Produces: a crate where all 8 tests pass, which `make test` runs (solutions are workspace default-members; exercises are not).

- [ ] **Step 1: Write the reference library**

Replace `lessons/26-testing/solutions/src/lib.rs` entirely with:

```rust
//! Lesson 26 — reference solutions.
//!
//! The four warm-up tests are here rather than in `tests/exercise.rs`:
//! two unit tests that reach the private `run_len`, and two doc tests on
//! `encode`. The main exercise, `decode`, is graded by `tests/exercise.rs`.

/// Run-length encode: every run of identical characters becomes
/// `<count><char>`.
///
/// ```
/// use testing_solutions::encode;
/// assert_eq!(encode("aaab"), "3a1b");
/// ```
///
/// A run of ten or more is still one run — the count just takes two digits:
///
/// ```
/// use testing_solutions::encode;
/// assert_eq!(encode("aaaaaaaaaaaa"), "12a");
/// ```
#[must_use]
pub fn encode(input: &str) -> String {
    let mut out = String::new();
    let mut rest = input;
    while let Some(ch) = rest.chars().next() {
        let n = run_len(rest);
        out.push_str(&n.to_string());
        out.push(ch);
        rest = &rest[n * ch.len_utf8()..];
    }
    out
}

/// Expand a run-length encoded string: `"3a1b"` becomes `"aaab"`.
///
/// Counts can run to more than one digit, so keep reading digits until you
/// reach the character they belong to.
#[must_use]
pub fn decode(input: &str) -> String {
    let mut out = String::new();
    let mut count = 0usize;
    for ch in input.chars() {
        if let Some(digit) = ch.to_digit(10) {
            count = count * 10 + digit as usize;
        } else {
            out.extend(std::iter::repeat_n(ch, count));
            count = 0;
        }
    }
    out
}

/// Length of the leading run of identical characters.
fn run_len(input: &str) -> usize {
    let Some(first) = input.chars().next() else {
        return 0;
    };
    input.chars().take_while(|c| *c == first).count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn warmup_run_len_counts_the_leading_run() {
        assert_eq!(run_len("aaab"), 3);
    }

    #[test]
    fn warmup_run_len_of_an_empty_string_is_zero() {
        assert_eq!(run_len(""), 0);
    }
}
```

- [ ] **Step 2: Write the solutions' main tests**

Replace `lessons/26-testing/solutions/tests/exercise.rs` with the Task 3 Step 2 file, changing only the crate in the `use` line:

```rust
use proptest::prelude::*;
use testing_solutions::{decode, encode};

#[test]
fn main_expands_each_run() {
    assert_eq!(decode("3a1b"), "aaab");
}

#[test]
fn main_reads_multi_digit_counts() {
    assert_eq!(
        decode("1a12b"),
        "abbbbbbbbbbbb",
        "a count can run to more than one digit, anywhere in the string - keep reading digits until the character"
    );
}

proptest! {
    #![proptest_config(ProptestConfig { failure_persistence: None, ..ProptestConfig::default() })]

    /// Whatever `encode` produced, `decode` must turn back into the original.
    ///
    /// `[^0-9]` is not an arbitrary alphabet: it is exactly the domain where
    /// this property holds, because a digit in the input makes
    /// `<count><char>` ambiguous.
    #[test]
    fn main_round_trips_whatever_encode_produced(s in "[^0-9]{0,30}") {
        prop_assert_eq!(decode(&encode(&s)), s);
    }
}

/// The warm-up tests are yours to write - but not yours to delete.
#[test]
fn warmup_all_four_tests_are_still_there() {
    let src = include_str!("../src/lib.rs");
    assert!(
        src.contains("#[cfg(test)]"),
        "the warm-up unit test module is gone - deleting a test is not passing it"
    );
    assert_eq!(
        src.matches("/// ```").count(),
        4,
        "both doc-test examples on `encode` must stay - deleting a test is not passing it"
    );
}
```

- [ ] **Step 3: Verify all 8 pass, in three blocks**

```bash
cargo test --manifest-path lessons/26-testing/solutions/Cargo.toml
```

Expected: three `running`/`test result` blocks — `2 passed` (unit), `4 passed` (integration), `2 passed` (Doc-tests). 8 total, 0 failed.

- [ ] **Step 4: Verify lint-clean**

```bash
cargo clippy --package testing-solutions --all-targets -- -D warnings
cargo fmt --check -p testing-solutions
```

Expected: both silent.

- [ ] **Step 5: Verify the two crates agree**

```bash
diff <(sed 's/testing_exercises/testing_solutions/' lessons/26-testing/exercises/tests/exercise.rs) lessons/26-testing/solutions/tests/exercise.rs
```

Expected: no output — the two test files differ only in the crate name.

- [ ] **Step 6: Commit**

```bash
git add lessons/26-testing/solutions
git commit -m "feat(lesson-26): add reference solutions"
```

---

## Task 5: Lesson README

**Files:**
- Replace: `lessons/26-testing/README.md`

**Interfaces:**
- Consumes: the API and test names from Tasks 3-4.
- Produces: the self-study text. Task 6's deck compresses the same material.

Wrap prose at 79 characters (Step 3 checks this). Five self-study `###` sections and four exercise `###` sections — the house shape.

- [ ] **Step 1: Write the README**

Replace `lessons/26-testing/README.md` entirely with:

````markdown
# Lesson 26 — Testing

For twenty-five lessons the tests told you what to build: every
`tests/exercise.rs` was the specification and your job was to satisfy it.
This lesson turns that around. Rust gives a test three homes — beside the
code, outside the crate, and inside the documentation — and each one can
reach a different part of what you wrote. Then it hands you a machine that
invents inputs you would never have thought of. The production skill: **the
tests you think of are not the ones that catch you**, so you write both the
examples you can name and the properties that must always hold.

## Learning goals

- Put a test in the right of its three homes — `#[cfg(test)] mod tests`,
  `tests/*.rs`, or a `///` example — and say what each one can reach
- Write a unit test that exercises a private item, and explain where that
  access actually comes from — being a child module, not the import
- Write a doc test, and explain why an example the tooling compiles and
  runs is the part of your documentation least able to rot
- State an invariant as a `proptest` property and read the *shrunk*
  counterexample it reports
- Explain why `#[cfg(test)]` code does not exist in a normal build, and
  what `[dev-dependencies]` is for

## Self-study notes

### Three homes for a test

| kind | where it lives | what it can see |
| --- | --- | --- |
| unit | `src/lib.rs`, in `#[cfg(test)] mod tests` | everything, private items included |
| integration | `tests/*.rs` | the public API only |
| doc | a `///` example | the public API only |

These are not three styles of the same thing. They are three vantage
points. A unit test stands *inside* the module and can poke at a private
helper. An integration test stands *outside* the crate entirely and sees
exactly what someone who depends on your library sees. A doc test stands
where a reader stands: it is the first example anyone reads, and `cargo
test` compiles and runs it so it cannot quietly go stale.

Pick by what you need to reach. Testing a private helper directly? Unit
test — nothing else can see it. Checking that the API you publish actually
works from outside? Integration test. Showing someone how to use a
function? Doc test, and you get a real test for free.

### Unit tests, and what `#[cfg(test)]` really means

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_len_counts_the_leading_run() {
        assert_eq!(run_len("aaab"), 3);
    }
}
```

Two attributes are doing the work. `#[test]` marks a function as a test.
`#[cfg(test)]` is the interesting one: it means *compile this only when
building tests*. In a normal `cargo build` the module is not compiled, not
included, not shipped — it does not exist. That is why you can keep tests
next to the code they test without bloating what you publish.

Now the part that is easy to get backwards. A child module may always see
its parent's private items — that access is a fact about module nesting,
not something an import grants. `use super::*;` only puts the parent's
names *in scope*, so you can write `run_len(...)` instead of
`super::run_len(...)`. Drop the import and the longer form still compiles.

"Building tests" means *this crate's* tests, and your library gets compiled
twice. When `cargo test` builds `tests/exercise.rs`, it links a copy
compiled **without** `cfg(test)` — so an integration test cannot see
`#[cfg(test)]` items either, even `pub` ones.

This lesson's compile-fail file shows that edge: call a `#[cfg(test)]`
function from ordinary code and it does not fail with "unused" — it fails
with *cannot find function*, because in that build the item was never
created.

### Doc tests — documentation that cannot rot

Any ```` ``` ```` block in a `///` comment is compiled and run by `cargo
test`:

```rust
/// Run-length encode: every run becomes `<count><char>`.
///
/// ```
/// use testing_solutions::encode;
/// assert_eq!(encode("aaab"), "3a1b");
/// ```
pub fn encode(input: &str) -> String {
    // ...
}
```

Doc tests are compiled *outside* your crate, as a consumer of it. That is
why the example needs a `use` naming your crate — exactly what a reader of
your documentation would have to write. Each ```` ``` ```` block is
reported and run as its own test, though current rustdoc compiles them
together into one bundle, which is why a failing example points at a
`doctest_bundle_….rs` temporary file rather than at `src/lib.rs`.

A comment can lie about the code it sits next to, and given enough commits
it will. An example that is compiled and run cannot: rename the function
and the doc test stops compiling. Doc examples are the one part of your
documentation the tooling checks for you — the prose around them can still
rot.

Two details worth knowing. A line starting with `# ` — hash, then a space —
runs but is hidden from the rendered page, so setup does not clutter the
example; write `##` at the start of a line when you want a literal `#`,
which is how an attribute such as `#[derive(Debug)]` stays visible. And the
fence takes attributes: ```` ```no_run ```` compiles without
running, ```` ```ignore ```` does neither, and ```` ```should_panic ````
expects the example to panic.

### Reading `cargo test` output

`cargo test` builds and runs several separate test binaries and prints a
block for each:

```text
     Running unittests src/lib.rs (target/debug/deps/testing_solutions-1877…)
running 2 tests
     Running tests/exercise.rs (target/debug/deps/exercise-25fa…)
running 4 tests
   Doc-tests testing_solutions
running 2 tests
```

Two things about that order matter in practice. It is fixed — library
tests, then `tests/*.rs`, then doc tests — and **the run stops at the first
binary that fails**. If your unit tests are red you will not even see
whether the doc tests pass. Two ways out:

```bash
cargo test --no-fail-fast     # run every binary, report all failures
cargo test --lib              # just the #[cfg(test)] mod tests
cargo test --test exercise    # just tests/exercise.rs
cargo test --doc              # just the examples in doc comments
cargo test run_len            # any test whose name contains "run_len"
```

That last one has a wrinkle: a bare name filter skips doc tests entirely.
Use `cargo test --doc run_len` to filter those.

Also useful: `#[should_panic]` for a test that must panic, `#[ignore]` for
one that only runs when asked (`cargo test -- --ignored`), and
`-- --nocapture` when you want to see the `println!`s from a passing test,
which cargo otherwise swallows.

### Properties, and what shrinking gives you

Every example test above has the same weakness: you can only write the
cases you thought of. Every example in this lesson feeds `encode` nothing
but letters, because letters are what came to mind.

A **property** test inverts that. You state something that must hold for
*all* inputs, and the machine generates them:

```rust
use proptest::prelude::*;

proptest! {
    #[test]
    fn round_trip(s in "[^0-9]{0,30}") {
        prop_assert_eq!(decode(&encode(&s)), s);
    }
}
```

`s in "[^0-9]{0,30}"` is a **strategy**: it describes the inputs to draw
from — here, a regex. `prop_assert_eq!` is the property-test flavour of
`assert_eq!`. proptest runs the body 256 times by default, with different
values each time.

Why `[^0-9]` rather than "any string"? Because the round-trip is simply
*not true* for any string, and naming the domain where a property holds is
part of stating it. See for yourself — widen the strategy to `".*"` and run
the test:

```text
minimal failing input: s = "0"
```

That is the other half of the payoff. proptest does not just hand you the
random 27-character string that broke; it **shrinks** it, retrying smaller
and simpler inputs until it finds the smallest one that still fails. One
character. And it is telling you something real: `encode("0")` is `"10"`,
which `decode` reads as ten of a character that never arrives. A digit in
the input makes `<count><char>` ambiguous, which is exactly why the shipped
strategy excludes digits. Put it back when you have seen it.

Now note what each kind of test catches, because the answer is not "the
property catches more". The example `decode("1a12b")` catches a decoder
that reads only the first digit of a count — and the property never will,
because a random draw will not produce ten identical characters in a row.
The property catches a `decode` that indexes bytes instead of characters,
reporting `minimal failing input: s = "¡"` — and no example here would have
thought to try a non-ASCII character. Neither kind subsumes the other.

Two housekeeping notes. `proptest` is declared under `[dev-dependencies]`,
and that declaration is what keeps it out of what this crate ships — anyone
who depends on the crate never builds it. (Where a crate is *used* does not
decide that; where it is *declared* does.) Dev-dependencies are available
anywhere this crate is compiled for testing: `tests/`, `#[cfg(test)]`
modules and doc tests alike. And by default proptest saves a failing case
to a `.proptest-regressions` file so the next run replays it first; this
lesson switches that off with `failure_persistence: None` so a red
`make verify` leaves no untracked files behind.

## Exercises

### Warm-up: write the tests

Unusually, the warm-up deliverable is *tests*. `encode` and the private
`run_len` are given and correct.

**The two unit tests** are stubbed in `src/lib.rs`:

```rust
#[cfg(test)]
mod tests {
    #[test]
    fn warmup_run_len_counts_the_leading_run() {
        todo!("assert what run_len(\"aaab\") returns")
    }
    // ...
}
```

Replace each `todo!()` with a real assertion. Add `use super::*;` in the
same edit as your first assertion — on its own it is an unused import,
which is a compile error in this course. (You can also skip the import and
write `super::run_len(...)`; the import is the conventional choice, not the
thing that grants access.)

**The two doc tests** are stubbed on `encode`:

```rust
/// ```
/// use testing_exercises::encode;
/// let expected: &str = todo!("what does encode(\"aaab\") return?");
/// assert_eq!(encode("aaab"), expected);
/// ```
```

Replace each `todo!()` with what `encode` actually returns, worked out from
the format. The second one, a run of twelve, is the fact the main exercise
turns around — get it right before you start on `decode`. Doc tests run
last, so to jump straight to them:
`cargo test --manifest-path lessons/26-testing/exercises/Cargo.toml --doc`.

Being straight with you: all four of these are on your honour. A test that
asserts nothing passes, and no harness can tell an empty assertion from a
real one — which is worth knowing about testing in general, not just about
this exercise. What *is* checked is that the four tests still exist: delete
them and `tests/exercise.rs` fails, because deleting a test is not passing
it.

### Main: `decode`

Implement `decode` so it expands what `encode` produced:

```rust
pub fn decode(_input: &str) -> String {
    todo!("expand each `<count><char>` run back into characters")
}
```

The parameter starts with `_` because unused variables are compile errors
here — rename it in the same edit where you replace the `todo!()`. Three
given tests in `tests/exercise.rs` grade it, including a proptest
round-trip. The one that catches most first attempts is `decode("1a12b")`:
a count is not always a single digit, and not always at the start.

### Compile-fail

`exercises/compile_fails/26-cfg-test-not-compiled.rs` calls a
`#[cfg(test)]` function from ordinary code. In a normal build that function
is never compiled, so the call cannot resolve — E0425, cannot find
function, with a note pointing at the gate that configured it out. (An
integration test in `tests/` gets this same error for the same reason.) Fix
it by deciding the function is not test-only. Check this one without waiting
for the tests:
`cargo run --package compile-fails -- --expect compiles lessons/26-testing`.

### Run

```bash
make verify LESSON=26-testing
```

This runs all three kinds of test and asserts the compile-fail file now
compiles. Remember it stops at the first failing binary, so you will meet
the work in this order: the unit tests, then `decode`, then the doc tests.
To see everything that is red at once, add `--no-fail-fast`:

```bash
cargo test --manifest-path lessons/26-testing/exercises/Cargo.toml --no-fail-fast
```

## Solutions

See `solutions/src/lib.rs` for the reference implementations — including
the reference answers to the four tests you are asked to write. Try the
exercises before peeking.
````

- [ ] **Step 2: Verify the section counts**

```bash
awk '/^## /{s=$0} /^### /{print (index(s,"Exercises")?"exercise":"self-study"), $0}' \
  lessons/26-testing/README.md
```

Expected: exactly 5 lines tagged `self-study` and 4 tagged `exercise`.

- [ ] **Step 3: Verify the prose wrap**

```bash
awk 'length > 79 {print FILENAME":"FNR": "length}' lessons/26-testing/README.md
```

Expected: only lines inside fenced code blocks or the markdown table may exceed 79. Prose lines must not.

- [ ] **Step 4: Commit**

```bash
git add lessons/26-testing/README.md
git commit -m "docs(lesson-26): write self-study notes"
```

---

## Task 6: Slide deck

**Files:**
- Replace: `lessons/26-testing/slides/slides.md`

**Interfaces:**
- Consumes: the material from Task 5.
- Produces: the deck `build-index` publishes.

Exactly 10 slides, so exactly 9 `---` separators on their own lines.

- [ ] **Step 1: Write the deck**

Replace `lessons/26-testing/slides/slides.md` entirely with:

````markdown
# Testing

> For twenty-five lessons the tests told you what to build. Now you write them — and find out that the tests you think of are never the ones that catch you.

---

## Three homes for a test

| kind | where | what it sees |
| --- | --- | --- |
| unit | `src/lib.rs`, in `#[cfg(test)] mod tests` | everything, private included |
| integration | `tests/*.rs` | the public API only |
| doc | a `///` example | the public API only |

Not three styles — three **vantage points**.

---

## Unit tests

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_len_counts_the_leading_run() {
        assert_eq!(run_len("aaab"), 3);
    }
}
```

`#[cfg(test)]` means *compile this only when building tests* — in a normal build the module does not exist. A child module already sees its parent's private items; `use super::*;` just puts them in scope.

---

## Integration tests

```rust
use testing_solutions::decode;   // your crate, from outside
```

`tests/exercise.rs` is its own crate. It sees only what is `pub`, which is the point: it tests the API you ship, not the code you happen to have. It links a copy of your library built *without* `cfg(test)`, so it cannot see your unit tests either.

You have been reading these all course.

---

## Doc tests

```rust
/// ```
/// use testing_solutions::encode;
/// assert_eq!(encode("aaab"), "3a1b");
/// ```
```

`cargo test` compiles and runs every example in your docs.

A comment can lie about the code. An example that is run cannot — rename the function and the doc test stops compiling.

---

## What `cargo test` actually runs

```text
     Running unittests src/lib.rs (…)     running 2 tests
     Running tests/exercise.rs (…)        running 4 tests
   Doc-tests testing_solutions            running 2 tests
```

Fixed order, and it **stops at the first one that fails**. `--no-fail-fast` runs them all; `--lib`, `--test exercise` and `--doc` pick one.

Also worth knowing: `#[should_panic]`, `#[ignore]`, and `-- --nocapture`.

---

## Examples run out

You can only write the cases you thought of.

Every example in this lesson feeds `encode` nothing but letters — because letters are what came to mind.

---

## Properties, and shrinking

```rust
proptest! {
    #[test]
    fn round_trip(s in "[^0-9]{0,30}") {   // now widen it to ".*"
        prop_assert_eq!(decode(&encode(&s)), s);
    }
}
```

```text
minimal failing input: s = "0"
```

You name an invariant; the machine hunts — 256 draws by default. When it finds a failure it **shrinks** it to the smallest input that still breaks.

---

## Each kind caught what the other missed

The **example** `decode("1a12b")` catches a decoder that reads only the first digit of a count — random draws never produce ten identical characters in a row.

The **property** catches a `decode` that indexes bytes rather than characters: `minimal failing input: s = "¡"`. No example here would have thought to try a non-ASCII character.

Neither subsumes the other.

---

## Wrap — testing in Rust

- three homes: beside the code, outside the crate, inside the docs
- `#[cfg(test)]` code does not exist in a normal build
- a doc example is the one part of your docs the tooling checks
- a property says what must always hold; shrinking says where it broke
- examples find what you thought of, properties find what you didn't

Next: **Lesson 27 — Clippy, rustfmt, miri** (the linter ecosystem & CI).
````

- [ ] **Step 2: Verify the slide count**

```bash
grep -c '^---$' lessons/26-testing/slides/slides.md
```

Expected: `9`.

- [ ] **Step 3: Verify the deck builds and renders**

```bash
make slides-build
grep -o '26-testing' dist/index.html | head -1
test -f dist/lessons/26-testing/slides/index.html && echo "deck present"
```

Expected: `26-testing` appears in the index and the deck file exists.

- [ ] **Step 4: Commit**

```bash
git add lessons/26-testing/slides/slides.md
git commit -m "feat(lesson-26): write slide deck"
```

---

## Task 7: End-to-end verification + push

**Files:** none modified — this task verifies and ships.

**Interfaces:**
- Consumes: everything from Tasks 1-6.
- Produces: lesson 26 merged to `main`, pushed, deployed.

- [ ] **Step 1: Full local CI**

```bash
make ci
```

Expected: exit 0. This runs `cargo clippy --workspace --all-targets -- -D warnings`, `cargo fmt --all --check`, `cargo build --workspace --all-targets`, `cargo test` (tools + solutions, including the solutions' unit and doc tests) and `compile-fails --expect broken lessons`.

- [ ] **Step 2: Verify the student path from a clean tree, twice**

Run it twice in a row. The second run is not redundant: it is what proves proptest's failure persistence is really off, which is the spec's Done criterion 5.

```bash
make verify LESSON=26-testing 2>&1 | tail -20
make verify LESSON=26-testing 2>&1 | tail -5
git status --porcelain
```

Expected: both runs FAIL — this is the shipped exercise and it must be red. Confirm the failure is test failures, not compile errors. `git status --porcelain` must print nothing: a `.proptest-regressions` file appearing here means `failure_persistence: None` was dropped from the test config.

- [ ] **Step 3: Verify the solutions path passes all 8**

```bash
cargo test --manifest-path lessons/26-testing/solutions/Cargo.toml 2>&1 | grep -E 'test result|running'
```

Expected: `2 passed`, `4 passed`, `2 passed`; nothing failed.

- [ ] **Step 4: Verify the tree is clean**

```bash
git status --porcelain
find . -name '*.proptest-regressions' -not -path './target/*'
```

Expected: no output from either.

- [ ] **Step 5: Merge to main and re-verify**

Follow superpowers:finishing-a-development-branch. Merge the branch into `main`, then re-run `make ci` on the merged result. If it fails, STOP — do not push.

- [ ] **Step 6: Push**

```bash
git push
```

Expected: push succeeds. The CI cache key hashes `**/Cargo.toml`, which changed when proptest was added, so the first run refetches on both the stable and beta legs.

- [ ] **Step 7: Watch CI and Deploy**

List the runs for the pushed commit. If fewer than two rows (`CI` and `Deploy`) appear, wait ~10 seconds and re-run it:

```bash
gh run list --commit "$(git rev-parse HEAD)" --json databaseId,workflowName,status
```

Then wait on each, using the `databaseId` values from that output. `gh run watch` takes the run id as a positional argument — called bare it opens an interactive picker and will hang or fail in a non-interactive shell, and without `--exit-status` it cannot signal failure:

```bash
gh run watch <CI databaseId> --exit-status
gh run watch <Deploy databaseId> --exit-status
gh run list --commit "$(git rev-parse HEAD)" --json workflowName,conclusion
```

Expected: both `gh run watch` commands exit 0, and both `CI` and `Deploy` show `"conclusion":"success"`.

- [ ] **Step 8: Verify live**

```bash
curl -s -o /dev/null -w '%{http_code}\n' https://rust.ristkari.dev/
curl -s -o /dev/null -w '%{http_code}\n' https://rust.ristkari.dev/lessons/26-testing/slides/
```

Expected: `200` from both.
