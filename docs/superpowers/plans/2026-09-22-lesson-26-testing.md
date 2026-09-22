# Lesson 26 — Testing — Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Author the first lesson of Phase 7: testing. Rust puts a test in one of three homes — `#[cfg(test)] mod tests` (sees private items), `tests/*.rs` (a separate crate, public API only), and `///` examples (documentation that is compiled and run) — and the lesson's thesis is that examples and properties catch different bugs. Warm-up (inverted: the *tests* are the deliverable): two unit tests on a private `run_len`, two doc tests on `encode`. Main: implement `decode`, graded by three examples and one `proptest` property. Compile-fail: calling a `#[cfg(test)]` function from normal code (E0425).

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
- The proptest strategy is `"[a-z]{0,30}"`, never `".*"` — the round-trip property is false for inputs containing digits, and the README explains that rather than shipping an unsatisfiable test.
- Compile-fail files are std-only (the tool runs bare `rustc` with no `--extern`).
- Out of scope — do not add: clippy/rustfmt/miri content (Lesson 27), benchmarking or `criterion` (Lesson 28), coverage, fuzzing, mutation testing, `insta`, `rstest`, mocking frameworks, `cargo-nextest`, custom harnesses.
- All code in this plan was verified on rustc 1.98.1 in a scratch crate carrying the workspace's exact `[lints]`: solutions 8/8 pass (2 unit + 4 integration + 2 doc), the stub compiles clippy- and rustfmt-clean with all 8 failing, and no `.proptest-regressions` file is produced. If clippy or rustfmt fires, do NOT add an `#[allow]` and do NOT change the code; STOP and report the exact output.

## Deviations from the spec

- **The spec's "8 tests in `tests/exercise.rs`" house shape is split 4/4 across two files.** `tests/exercise.rs` holds the four main tests; the four warm-up tests live in `src/lib.rs` because they are the deliverable, not the grader. The spec already describes this; it is called out here so a reviewer does not read it as drift.
- **`cargo test` stops at the first failing target.** Verified during planning: with the shipped stub, `cargo test` reports only the two unit-test failures and never reaches the integration or doc tests. The student therefore meets the work in cargo's fixed target order — unit tests, then `decode`, then the doc tests — which is NOT the order the README introduces them in. The README must say so explicitly and name `--lib` / `--test exercise` / `--doc` for picking one target. This behaviour was not anticipated in the spec.
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
- Produces: `proptest` resolvable as `{ workspace = true }` from both crates' `[dev-dependencies]`, i.e. from `tests/exercise.rs` but not from `src/lib.rs`.

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
    // Add `use super::*;` yourself. A unit test module can reach its
    // parent's private items, but only once they are in scope — and the stub
    // cannot ship with the import, because an unused import is a compile
    // error in this course.

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
fn main_expands_an_empty_string() {
    assert_eq!(decode(""), "");
}

#[test]
fn main_reads_multi_digit_counts() {
    assert_eq!(
        decode("12a"),
        "aaaaaaaaaaaa",
        "a count can run to more than one digit - keep reading digits until the character"
    );
}

proptest! {
    #![proptest_config(ProptestConfig { failure_persistence: None, ..ProptestConfig::default() })]

    /// Whatever `encode` produced, `decode` must turn back into the original.
    #[test]
    fn main_round_trips_whatever_encode_produced(s in "[a-z]{0,30}") {
        prop_assert_eq!(decode(&encode(&s)), s);
    }
}
```

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

Expected: `2 failed`, `4 failed`, `2 failed` respectively — 8 in total, 0 passed anywhere.

- [ ] **Step 6: Verify the compile-fail file is broken as shipped**

```bash
cargo run --package compile-fails -- --expect broken lessons/26-testing
```

Expected: pass (the file does not compile, which is what "broken" asserts).

- [ ] **Step 7: Verify the compile-fail file compiles once fixed**

```bash
cp lessons/26-testing/exercises/compile_fails/26-cfg-test-not-compiled.rs /tmp/cf26.bak
sed -i '' '/^#\[cfg(test)\]$/d' lessons/26-testing/exercises/compile_fails/26-cfg-test-not-compiled.rs
cargo run --package compile-fails -- --expect compiles lessons/26-testing
cp /tmp/cf26.bak lessons/26-testing/exercises/compile_fails/26-cfg-test-not-compiled.rs
```

Expected: the middle command passes. The last command restores the broken file — verify with `git diff --stat lessons/26-testing/exercises/compile_fails/` that it reports no change before committing.

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
fn main_expands_an_empty_string() {
    assert_eq!(decode(""), "");
}

#[test]
fn main_reads_multi_digit_counts() {
    assert_eq!(
        decode("12a"),
        "aaaaaaaaaaaa",
        "a count can run to more than one digit - keep reading digits until the character"
    );
}

proptest! {
    #![proptest_config(ProptestConfig { failure_persistence: None, ..ProptestConfig::default() })]

    /// Whatever `encode` produced, `decode` must turn back into the original.
    #[test]
    fn main_round_trips_whatever_encode_produced(s in "[a-z]{0,30}") {
        prop_assert_eq!(decode(&encode(&s)), s);
    }
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

Wrap prose at 75 characters. Five self-study `###` sections and four exercise `###` sections — the house shape.

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
- Write a unit test that exercises a private item, and explain why
  `use super::*;` is what gives it that access
- Write a doc test, and explain why an example that is compiled and run is
  the only documentation that cannot rot
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
| doc | a `///` example on a public item | the public API only |

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

`use super::*;` brings in everything from the parent module, private items
included. A child module may always see its parent's private items; the
import is what puts them in scope. Leave it out and `run_len` is simply not
a name this module knows.

This lesson's compile-fail file shows the other edge of `#[cfg(test)]`:
call a `#[cfg(test)]` function from ordinary code and it does not fail with
"unused" — it fails with *cannot find function*, because in that build the
item was never created.

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

A doc test is compiled as its own tiny crate that depends on yours, which
is why the example needs a `use` naming your crate — exactly as a reader
of your documentation would have to write. Each ```` ``` ```` block is a
separate test.

A comment can lie about the code it sits next to, and given enough commits
it will. An example that is compiled and run cannot: rename the function
and the doc test stops compiling. That is the whole argument for doc tests
— they are the only documentation the compiler checks.

Two details worth knowing. A line starting with `#` runs but is hidden
from the rendered page, so setup does not clutter the example. And the
fence takes attributes: ```` ```no_run ```` compiles without running,
```` ```ignore ```` does neither, and ```` ```should_panic ```` expects the
example to panic.

### Reading `cargo test` output

`cargo test` runs your tests as several separate binaries and prints a
block for each:

```text
running 2 tests          <- the unit tests, from src/lib.rs
running 4 tests          <- tests/exercise.rs
   Doc-tests testing_solutions
running 2 tests          <- the examples in your doc comments
```

Two things about that order matter in practice. It is fixed — library
tests, then `tests/*.rs`, then doc tests — and **the run stops at the first
binary that fails**. If your unit tests are red you will not even see
whether the doc tests pass. To work on one at a time:

```bash
cargo test --lib              # just the #[cfg(test)] mod tests
cargo test --test exercise    # just tests/exercise.rs
cargo test --doc              # just the examples in doc comments
cargo test run_len            # any test whose name contains "run_len"
```

Also useful: `#[should_panic]` for a test that must panic, `#[ignore]` for
one that only runs when asked (`cargo test -- --ignored`), and
`-- --nocapture` when you want to see the `println!`s from a passing test,
which cargo otherwise swallows.

### Properties, and what shrinking gives you

Every example test above has the same weakness: you can only write the
cases you thought of. Every one in this lesson uses letters, because
letters are what came to mind.

A **property** test inverts that. You state something that must hold for
*all* inputs, and the machine generates them:

```rust
use proptest::prelude::*;

proptest! {
    #[test]
    fn round_trip(s in "[a-z]{0,30}") {
        prop_assert_eq!(decode(&encode(&s)), s);
    }
}
```

`s in "[a-z]{0,30}"` is a **strategy**: it describes the inputs to draw
from — here, a regex. `prop_assert_eq!` is the property-test flavour of
`assert_eq!`. proptest runs the body a few hundred times with different
values.

The payoff is what happens when it fails. proptest does not just hand you
the random 27-character string that broke: it **shrinks** it, retrying
smaller and simpler inputs until it finds the smallest one that still
fails, and reports that:

```text
minimal failing input: s = "0"
```

Try it. Widen the strategy from `"[a-z]{0,30}"` to `".*"`, run the test,
and you get exactly that counterexample — one character. It is telling you
something real: `<count><char>` cannot survive a digit in the input,
because `encode("1")` is `"11"` and `decode` reads that as eleven of
something that never arrives. The round-trip property is simply *not true*
for all strings, and naming the domain where a property does hold is part
of stating it. Then put the strategy back.

Note what each kind caught. The property found the digit case, which no
hand-written example was ever going to try. But a random draw from `[a-z]`
will not produce ten identical letters in a row in a few hundred tries, so
the multi-digit count in `decode("12a")` is caught only by an example
somebody wrote on purpose. Neither kind subsumes the other.

Two housekeeping notes. `proptest` is declared under
`[dev-dependencies]`, not `[dependencies]` — it is used only from
`tests/`, so it is not part of what this crate ships, and anyone depending
on the crate never builds it. And by default proptest saves a failing case
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

Replace each `todo!()` with a real assertion. You must also add
`use super::*;` yourself — the stub cannot ship with it, because an unused
import is a compile error in this course, and it is the line the exercise
is about.

**The two doc tests** are stubbed on `encode`:

```rust
/// ```
/// use testing_exercises::encode;
/// let expected: &str = todo!("what does encode(\"aaab\") return?");
/// assert_eq!(encode("aaab"), expected);
/// ```
```

Replace each `todo!()` with what `encode` actually returns. These are
graded properly — a wrong guess fails — so work it out from the format
rather than running the function first. The second one, a run of twelve,
is worth getting right before you start on `decode`.

Being straight with you: the two unit tests are on your honour. A test
body of `assert!(true)` passes them, and no test harness can tell the
difference between a real assertion and an empty one. That is worth
knowing about testing in general, not just about this exercise.

### Main: `decode`

Implement `decode` so it expands what `encode` produced:

```rust
pub fn decode(_input: &str) -> String {
    todo!("expand each `<count><char>` run back into characters")
}
```

The parameter starts with `_` because unused variables are compile errors
here — rename it in the same edit where you replace the `todo!()`. Four
given tests in `tests/exercise.rs` grade it, including a proptest
round-trip. The one that catches most first attempts is `decode("12a")`: a
count is not always a single digit.

### Compile-fail

`exercises/compile_fails/26-cfg-test-not-compiled.rs` calls a
`#[cfg(test)]` function from ordinary code. In a normal build that function
is never compiled, so the call cannot resolve — E0425, cannot find
function, with a note pointing at the gate that configured it out. Fix it
by deciding the function is not test-only. Check this one without waiting
for the tests:
`cargo run --package compile-fails -- --expect compiles lessons/26-testing`.

### Run

```bash
make verify LESSON=26-testing
```

This runs all three kinds of test and asserts the compile-fail file now
compiles. Remember it stops at the first failing binary, so you will meet
the work in this order: the unit tests, then `decode`, then the doc tests.

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

`#[cfg(test)]` means *compile this only when building tests* — in a normal build the module does not exist. `use super::*;` is what reaches the parent's private items.

---

## Integration tests

```rust
use testing_solutions::decode;   // your crate, from outside
```

`tests/exercise.rs` is its own crate. It sees only what is `pub`, which is the point: it tests the API you ship, not the code you happen to have.

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
running 2 tests          <- unit, from src/
running 4 tests          <- tests/exercise.rs
   Doc-tests testing_solutions
running 2 tests          <- your doc comments
```

Fixed order, and it **stops at the first one that fails**. Pick one with `--lib`, `--test exercise`, `--doc`.

---

## Examples run out

You can only write the cases you thought of.

Every example in this lesson uses letters — because letters are what came to mind.

---

## Properties, and shrinking

```rust
proptest! {
    #[test]
    fn round_trip(s in ".*") {
        prop_assert_eq!(decode(&encode(&s)), s);
    }
}
```

```text
minimal failing input: s = "0"
```

You name an invariant; the machine hunts. When it finds a failure it **shrinks** it to the smallest input that still breaks.

---

## Each kind caught what the other missed

The **property** found `"0"` — a digit makes `<count><char>` ambiguous, and no hand-written example was going to try that.

The **example** found `decode("12a")` — a random draw from `[a-z]` will not produce ten identical letters in a few hundred tries.

Neither subsumes the other.

---

## Wrap — testing in Rust

- three homes: beside the code, outside the crate, inside the docs
- `#[cfg(test)]` code does not exist in a normal build
- a doc test is the only documentation the compiler checks
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

- [ ] **Step 2: Verify the student path from a clean tree**

```bash
make verify LESSON=26-testing 2>&1 | tail -20
```

Expected: FAILS — this is the shipped exercise and it must be red. Confirm the failure is test failures, not compile errors.

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

- [ ] **Step 6: Push and watch**

```bash
git push origin main
gh run watch
```

Expected: CI (stable + beta) and Deploy both conclude `success`.

- [ ] **Step 7: Verify live**

```bash
curl -s -o /dev/null -w '%{http_code}\n' https://rust.ristkari.dev/
curl -s -o /dev/null -w '%{http_code}\n' https://rust.ristkari.dev/lessons/26-testing/slides/
```

Expected: `200` from both.
