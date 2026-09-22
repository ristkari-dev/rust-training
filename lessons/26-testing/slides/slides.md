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
