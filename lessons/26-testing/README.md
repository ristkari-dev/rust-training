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

Any Rust ```` ``` ```` block in a `///` comment is compiled and run by
`cargo test` (unless you tell it otherwise — see the fence attributes
below):

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
example; write `##` at the start of a line when the rendered line must
begin with a literal `#`. An attribute needs no escape — the hiding rule
requires the space, so `#[derive(Debug)]` shows as written. And the
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
cargo test -p testing-exercises --no-fail-fast   # all of them, all failures
cargo test -p testing-exercises --lib            # the #[cfg(test)] mod tests
cargo test -p testing-exercises --test exercise  # tests/exercise.rs
cargo test -p testing-exercises --doc            # the doc-comment examples
cargo test -p testing-exercises encode           # names containing "encode"
```

Note the `-p`: without it you are asking the whole workspace, and this
crate is not part of what `cargo test` picks by default — you would get a
green bar belonging to somebody else's code. That last command has a
wrinkle too: a bare name filter skips doc tests entirely, so use
`cargo test -p testing-exercises --doc encode` to filter those.

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
