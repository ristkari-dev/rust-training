# Lesson 26 — Testing — design

The first lesson of Phase 7 (Tooling & quality). For twenty-five lessons the
student has *read* tests: every `tests/exercise.rs` was the specification
they implemented against. This lesson turns the tooling around and makes
them write tests. Rust puts a test in one of three homes — beside the code
(`#[cfg(test)] mod tests`), outside the crate (`tests/*.rs`), or inside the
documentation (a `///` example) — and each home can reach a different part
of your code. On top of that sits the lesson's real thesis: **examples and
properties catch different bugs, and you need both.** Builds on modules and
visibility (L15), `Option` and iterators (L07-L09), and every exercise
harness the course has shipped so far.

A deliberate inversion: in this lesson the *warm-up deliverable is tests*,
not a function. That is the only way to teach test authorship with a
test-graded harness, and it is confined to the warm-up — the main exercise
is ordinary code graded by given tests.

## Audience and prerequisites

- Has completed Lessons 01-25
- Comfortable with modules and `pub` (L15), `Option` (L07), iterators and
  `char` handling (L09), and adding a dependency (L14/L18/L22/L23/L24/L25)
- Has the toolchain installed and `make verify` working

## Learning goals

By the end of this lesson, the student can:

1. Put a test in the right of its three homes — `#[cfg(test)] mod tests`,
   `tests/*.rs`, or a `///` example — and say what each one can reach
2. Write a unit test that exercises a *private* item, and explain where
   that access comes from — being a child module, not the import
3. Write a doc test, and explain why an example the tooling compiles and
   runs is the part of the documentation least able to rot
4. State an invariant as a `proptest` property and read the *shrunk*
   counterexample it reports
5. Explain why `#[cfg(test)]` code does not exist in a normal build, and
   what `[dev-dependencies]` is for

## Scope

In scope: the three homes of a test and what each can see; `#[cfg(test)]`
and what it actually does to a normal build; `use super::*;` in a unit test
module; integration tests as a *separate crate* that sees only the public
API; doc tests — how they are compiled and run, that they need the crate
name in a `use`, and that a leading `# ` (hash, space) hides a setup line
from the rendered docs while `##` renders a literal `#` (an attribute needs no escape, since
the hiding rule requires the space); reading
`cargo test` output as three separate result blocks; the limits of
example-based tests; property testing with `proptest` — writing a strategy,
`prop_assert_eq!`, and reading a shrunk counterexample; `[dev-dependencies]`
as the place test-only crates go. Mentioned but not exercised:
`#[should_panic]`, `#[ignore]`, `--nocapture`, and proptest's
`.proptest-regressions` file. New infrastructure: `proptest` joins
`[workspace.dependencies]` and becomes the repo's first dev-dependency.

Out of scope (deferred or skipped): clippy, rustfmt and miri (Lesson 27);
benchmarking and `criterion` (Lesson 28); code coverage
(`cargo-llvm-cov`); fuzzing (`cargo-fuzz`); mutation testing; snapshot
testing (`insta`); parameterised-test crates (`rstest`); mocking frameworks
and trait-substitution test doubles; `cargo-nextest`; testing async code
beyond what Lesson 18 already showed; custom test harnesses and
`[[test]] harness = false`. Testing is introduced as *three places to put a
test, plus a machine that invents the inputs you did not think of*.

## New dependency infrastructure

One new entry in the root `[workspace.dependencies]`:

```toml
proptest = "1"
```

Verified during design on rustc 1.98.1: `proptest` resolves to 1.11.0 and
locks a tree of 22 crates (`rand` and its `rand_chacha`/`rand_core`/
`rand_xorshift`/`ppv-lite86` companions, `regex-syntax`, `bit-set`,
`bit-vec`, `unarray`, `quick-error`, `num-traits`, `zerocopy`, `fnv`,
`rusty-fork`, `wait-timeout`, `tempfile`, `rustix`, `errno`, `once_cell`,
`fastrand`, `getrandom`, `libc`, `cfg-if`, `bitflags`, `autocfg`) with no
MSRV trouble, and the whole tree is clean under the `-D warnings` CI sets.
No C toolchain and no external service.

Both lesson crates add, after `[lints]`:

```toml
[dev-dependencies]
proptest = { workspace = true }
```

This is the **first `[dev-dependencies]` section in the repository**, and
that is deliberate: it is a teaching point in its own right. `proptest` is
used only from tests, so it is not part of the library's public dependency
set and does not ship to anyone who depends on the crate. Note the precise
rule, which the README states and learning goal 5 asks for: it is the
*declaration* that keeps it out of what ships, not the usage, and a
dev-dependency is visible wherever the crate is compiled for testing —
`tests/`, `#[cfg(test)]` modules and doc tests alike.

`cargo build --workspace --all-targets` does build dev-dependencies, so CI
pays for proptest's tree once. Measured during design: a cold build of the
tree is a few seconds — comparable to the `tracing-subscriber` tree Lesson
25 added, and far below sqlx.

## Verified mechanics

Every mechanic below was confirmed empirically during design, in a scratch
crate carrying the workspace's exact `[lints]` (`clippy::all` +
`clippy::pedantic` + `rust_2018_idioms` + `unused`, all denied). They are
recorded here because three of them constrain the exercise shape and one of
them killed the first design.

1. **`todo!()` works inside a doc test.** `let expected: &str = todo!("…");`
   followed by `assert_eq!(…, expected);` compiles and panics at run time
   with `not yet implemented: …`. Clippy does **not** lint doc tests — it
   never compiles them — so this form is safe in a doc comment and only
   there.
2. **`todo!()` does *not* work in the same form inside `#[cfg(test)] mod
   tests`.** `cargo clippy --all-targets -- -D warnings` (which is what
   `make lint` runs) rejects it three times over: `unused_variables` on
   `expected`, `clippy::diverging_sub_expression` on the `todo!()`, and
   unreachable code after it. Unit-test stubs must therefore be a bare
   `todo!("…")` as the whole test body, which is lint-clean and is also the
   repo's established stub idiom.
3. **`assert_eq!(x, todo!())` does not even compile.** `assert_eq!` compares
   through references, so the `!` never coerces and rustc reports E0277
   `no implementation for `String == !`` — a confusing error to hand a
   student. This is why the doc-test stub binds `expected` with an explicit
   type first.
4. **A bare `todo!()` body makes `use super::*;` unused**, and
   `-D unused-imports` then fails `make lint`. The stub therefore ships
   *without* the import and the student adds it in the same edit as their
   first assertion — the README must say "same edit", because adding the
   import while a `todo!()` remains is a *build* error, not a test failure.
   Note the import is a convenience, not the access mechanism: with no
   import at all, `super::run_len("aaab")` compiles and passes (verified).
9. **A doc test is not compiled as its own crate.** Current rustdoc merges a
   crate's doc tests into one bundle, so a failing example panics at
   `…/doctest_bundle_2024.rs` and the run ends `all doctests ran in …;
   merged doctests compilation took …`. The student sees this on their first
   `make verify`, so the README must not claim one-crate-per-example. Each
   block is still reported and run as a separate test.
10. **The `#`-hiding rule needs the space.** Verified against rendered
   `cargo doc` HTML: `# let x = 1;` is hidden but runs, `##[derive(Debug)]`
   renders as a literal `#[derive(Debug)]`, and `#use std::fmt;` is a
   compile error (`expected one of ! or [, found keyword use`).
11. **An integration test cannot see `#[cfg(test)]` items either.** The
   library is compiled twice; `tests/*.rs` links the copy built *without*
   `cfg(test)`. Verified: calling a `#[cfg(test)] pub fn` from `tests/it.rs`
   gives the same E0425 + "configured out" note as the compile-fail file.
5. **`#[cfg(test)]` under the compile-fail harness produces a textbook
   error.** `rustc --edition=2024 --crate-type=lib --emit=metadata` (the
   bare invocation `tools/compile-fails` uses, with no `--test` and no
   `--extern`) reports E0425 `cannot find function `double` in this scope`
   and then `note: found an item that was configured out` pointing at the
   `#[cfg(test)]` line.
6. **`proptest` shrinks the round-trip failure to one character.** Run over
   `".*"`, `decode(encode(s)) == s` fails and reports
   `minimal failing input: s = "0"`.
7. **The reference `decode` is clean under `clippy::pedantic`**, including
   its `digit as usize` cast.
8. **Doc tests in `lessons/*/exercises` are not run by CI.** The root
   `Cargo.toml` sets `default-members = ["tools/*", "lessons/*/solutions"]`,
   so `make test`'s `cargo test` runs the *solutions* doc tests (verifying
   the reference answers) and never the exercise stubs. `make verify
   LESSON=26-testing` passes `--manifest-path lessons/26-testing/exercises/
   Cargo.toml` and therefore does run them. This split is what makes
   test-authoring exercises possible at all.

## Slide arc (10 slides)

1. **Title — Testing.** Hook: *"For twenty-five lessons the tests told you
   what to build. Now you write them — and find out that the tests you
   think of are never the ones that catch you."*
2. **Three homes for a test.**
   | where | file | can see |
   |---|---|---|
   | unit | `src/lib.rs`, in `#[cfg(test)] mod tests` | everything, private included |
   | integration | `tests/*.rs` | the public API only |
   | doc | a `///` example | the public API only — and it is *documentation* |
3. **Unit tests.**
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
   `#[cfg(test)]` means *compile this only when building tests* — in a
   normal build the module does not exist. `use super::*;` is what reaches
   the private items of the parent module.
4. **Integration tests.** `tests/exercise.rs` is its own crate. It does
   `use testing_solutions::decode;` like any other consumer, so it can only
   touch what is `pub`. That is the point: it tests the API you ship, not
   the code you happen to have. *You have been reading these all course.*
5. **Doc tests.**
   ```rust
   /// ```
   /// use testing_solutions::encode;
   /// assert_eq!(encode("aaab"), "3a1b");
   /// ```
   ```
   `cargo test` compiles and runs every example in your docs. A comment can
   lie about the code; an example that is run cannot.
6. **What `cargo test` actually runs.** Three result blocks, not one:
   ```text
   running 2 tests            <- unit tests, from src/
   running 4 tests            <- tests/exercise.rs
   running 2 tests            <- Doc-tests
   ```
   Also worth knowing: `#[should_panic]`, `#[ignore]`, and `--nocapture`
   when you want to see your `println!`s.
7. **Examples run out.** You can only write the cases you thought of. Every
   example in this lesson feeds `encode` nothing but letters — because
   letters are what came to mind.
8. **Properties, and shrinking.**
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
   You name an invariant; the machine hunts for inputs — 256 draws by
   default. When it finds one it **shrinks** it, from whatever random string
   broke first down to the smallest input that still breaks. One character.
9. **Each kind caught what the other missed.** The example `decode("1a12b")`
   catches a decoder that reads only the first digit of a count, and the
   property never will: the generator does not produce ten identical
   characters in a row (measured — in 1,000,000 draws the longest run was
   5). The property catches a `decode` that indexes bytes rather than
   characters, reporting `minimal failing input: s = "¡"`, and no example
   here would have thought to try a non-ASCII character. Neither kind
   subsumes the other.
10. **Wrap — testing in Rust.**
    - three homes: beside the code, outside the crate, inside the docs
    - `#[cfg(test)]` code does not exist in a normal build
    - a doc example is the one part of your docs the tooling checks
    - a property says what must always hold; shrinking says where it broke
    - examples find what you thought of, properties find what you didn't

    Next: **Lesson 27 — Clippy, rustfmt, miri** (the linter ecosystem & CI).

## Exercise spec

One domain for the whole lesson: **run-length encoding**. Each run of
identical characters becomes `<count><char>`, so `"aaab"` encodes to
`"3a1b"` and a run of twelve encodes to `"12a"`.

The split:

- **Given:** `encode` (public, complete) and `run_len` (private, complete).
- **Warm-up (4 tests, written by the student):** two unit tests on the
  private `run_len`, two doc tests on `encode`.
- **Main (4 tests, given):** the student implements `decode`; two example
  tests and one `proptest` property grade it, and a fourth test is a
  tripwire that fails only if the warm-up tests are deleted.

Eight graded tests, as the house shape requires — but distributed
differently from every prior lesson, because four of them are the
deliverable rather than the grader. `tests/exercise.rs` holds the four main
tests only; the four warm-up tests live in `src/lib.rs`, two in
`#[cfg(test)] mod tests` and two in `encode`'s doc comment.

### Given: `encode` and `run_len` (both crates)

```rust
/// Run-length encode: every run of identical characters becomes
/// `<count><char>`.
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

/// Length of the leading run of identical characters.
fn run_len(input: &str) -> usize {
    let Some(first) = input.chars().next() else {
        return 0;
    };
    input.chars().take_while(|c| *c == first).count()
}
```

`run_len` is private on purpose: it is the item only a unit test can reach,
and the warm-up exists to make the student feel that.

### Warm-up part 1: the unit tests

Ships in `exercises/src/lib.rs` as:

```rust
#[cfg(test)]
mod tests {
    // Add `use super::*;` yourself. A unit test module can see its
    // parent's private items, but only once they are in scope — and the
    // stub cannot ship with the import, because an unused import is a
    // compile error in this course.

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

Reference (`solutions/src/lib.rs`):

```rust
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

**Honesty note, to be carried into the README verbatim in substance:** a
student who replaces `todo!("…")` with a test that asserts nothing passes
this warm-up. There is no way to grade test *authorship* with a test
harness, and the README must say so rather than imply a rigour that is not
there. This applies to all four warm-up tests, the doc tests included — a
doc test can be made vacuous too (`let expected: &str = &encode("aaab");`
passes), and the first wrong guess prints the right answer in the
`assert_eq!` diff. What the harness *can* check is that the four tests still
exist, which `warmup_all_four_tests_are_still_there` in `tests/exercise.rs`
does. The main exercise is graded outright.

### Warm-up part 2: the doc tests

Ships on `encode` in `exercises/src/lib.rs`:

```rust
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
```

Reference (`solutions/src/lib.rs`, with the crate name changed):

```rust
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
```

The second doc test is not decoration: it is the student writing down, in
their own hand, the fact that counts can exceed one digit — right before the
main exercise asks them to parse one back.

### Main: `decode`

Stub (`exercises/src/lib.rs`):

```rust
/// Expand a run-length encoded string: `"3a1b"` becomes `"aaab"`.
///
/// Counts can be more than one digit — read digits until you reach the
/// character they belong to.
#[must_use]
pub fn decode(_input: &str) -> String {
    todo!("expand each `<count><char>` run back into characters")
}
```

Reference:

```rust
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
```

### Main tests (`tests/exercise.rs`)

Shipped in both crates, identical except for the crate named in the `use` —
`testing_exercises` in `exercises/`, `testing_solutions` in `solutions/`, as
every prior lesson does. The solutions copy is shown:

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
    assert!(
        src.matches("/// ```").count() >= 4,
        "both doc-test examples on `encode` must stay - deleting a test is not passing it"
    );
}
```

`failure_persistence: None` is deliberate and must be kept: by default
proptest writes a `.proptest-regressions` file next to the test when one
fails, which would leave an untracked file in the student's tree every time
`make verify` goes red. The README explains what the file is and why this
lesson switches it off, so the behaviour is taught rather than hidden.

The strategy is `"[^0-9]{0,30}"`, not `".*"` and not `"[a-z]"`. The
`<count><char>` format is ambiguous the moment the input contains a digit,
so the round-trip property is not true for all strings — and `[^0-9]` is
*exactly* the domain where it is true, which makes "naming the domain where
a property holds is part of stating it" a demonstrated point rather than an
asserted one. The README then invites the student to widen it to `".*"`,
watch it fail, and read `minimal failing input: s = "0"` — which costs them
nothing and is the most memorable thirty seconds in the lesson.

The three graded main tests are load-bearing in different directions, all
verified by attacking them with deliberately broken implementations:

- `decode("1a12b")` puts the multi-digit count in the *second* run. With the
  earlier `decode("12a")`, a decoder that parsed a multi-digit count only at
  the start of the string and single digits thereafter passed everything.
- The property over `[^0-9]` catches a `decode` that indexes bytes instead
  of characters, reporting `minimal failing input: s = "¡"`. Over `[a-z]` it
  could not: `encode` is UTF-8-correct, so the crate would have shipped an
  asymmetric encode/decode pair.
- Conversely the property cannot catch the single-digit-count bug, because a
  random draw does not produce ten identical characters in a row. Measured:
  in 1,000,000 draws from `[a-z]{0,30}` the longest run seen was 5. That
  asymmetry is the lesson's thesis, and it is now demonstrated *inside* the
  graded suite in both directions rather than asserted on a slide.

### Compile-fail: `26-cfg-test-not-compiled.rs`

```rust
// Compile-fail exercise: this file MUST NOT compile until you fix it.
//
// `#[cfg(test)]` means "compile this only when building tests". In a normal
// build the item is not merely unused - it does not exist at all, so
// anything that calls it fails to resolve. rustc reports E0425 ("cannot
// find function") and then points straight at the gate: "found an item that
// was configured out".
//
// That is the whole reason unit tests can live beside your code without
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

Std-only, as the harness requires — `tools/compile-fails` invokes bare
`rustc` with no `--extern`. Verified to fail with E0425 before the fix and
to compile cleanly after it.

## README structure

Five self-study `###` sections and four exercise `###` sections, matching
the house shape.

Self-study:

1. **Three homes for a test** — the table, and what each home can reach.
2. **Unit tests, and what `#[cfg(test)]` really means** — `use super::*;`,
   private access, and the fact that the module does not exist in a normal
   build (forward reference to the compile-fail).
3. **Doc tests — documentation that cannot rot** — the `use` line naming the
   crate, `#` to hide setup from the rendered page, and why `cargo test`
   running your examples is what keeps them honest.
4. **Reading `cargo test` output** — three result blocks, plus
   `#[should_panic]`, `#[ignore]` and `--nocapture`.
5. **Properties, and what shrinking gives you** — `proptest`, strategies,
   `prop_assert_eq!`, the shrunk counterexample, `[dev-dependencies]`, and
   the `.proptest-regressions` file this lesson switches off.

Exercises:

1. **Warm-up: write the tests** — both halves (unit and doc), including the
   honesty note that an empty assertion passes.
2. **Main: `decode`**
3. **Compile-fail**
4. **Run** — `make verify LESSON=26-testing`

## Lint expectations

- `#[must_use]` on `encode` and `decode` (`clippy::must_use_candidate` is
  pedantic and denied).
- The unit-test stub ships **without** `use super::*;` and with a bare
  `todo!()` body — any other form fails `make lint`, per Verified mechanics
  2 and 4.
- The `decode` stub takes `_input` so the unfinished exercise still
  compiles under `unused = "deny"`, and the README tells the student to
  rename it when they write the body — the same convention Lessons 24 and
  25 use.
- The doc-test stubs are not linted by clippy at all, which is what allows
  the richer `let expected: … = todo!(…)` form there.

## Done criteria

1. `make ci` is green.
2. `cargo test --manifest-path lessons/26-testing/solutions/Cargo.toml`
   passes 8 tests: 2 unit, 4 integration (one of them the property), 2 doc.
   (Not `make verify` — that target is hardcoded to `exercises/`.)
3. `make verify LESSON=26-testing` against the shipped `exercises/` fails 7
   of the 8 and compiles cleanly — no lint errors, no build errors, only
   test failures. The eighth, `warmup_all_four_tests_are_still_there`,
   passes: it is a tripwire, and nothing has been deleted yet.
4. `cargo run --package compile-fails -- --expect broken lessons` passes
   with `26-cfg-test-not-compiled.rs` present; `--expect compiles` passes
   once the `#[cfg(test)]` line is removed.
5. Running `make verify` twice in a row leaves no untracked files (proptest
   persistence is off).
6. The slide deck has 10 slides (9 `---` separators) and the README has 5
   self-study and 4 exercise `###` sections.
7. The root index lists Lesson 26 and its deck renders.

## Open questions

None. Three design choices were put to the course owner and settled:
the warm-up inverts the house pattern so the student writes tests
(accepted, with the honesty note above); `proptest` is added as a
dev-dependency rather than hand-rolling a generator or using `quickcheck`
(accepted); and the digit-breaks-the-format discovery is *told* in the
README with an invitation to reproduce it, rather than shipped as a red
test the student must repair (accepted — a shipped-red property would be
unsatisfiable through no fault of the student's own code).
