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
