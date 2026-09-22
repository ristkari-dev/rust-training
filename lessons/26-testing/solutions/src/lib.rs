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
