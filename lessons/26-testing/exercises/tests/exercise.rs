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
    assert!(
        src.matches("/// ```").count() >= 4,
        "both doc-test examples on `encode` must stay - deleting a test is not passing it"
    );
}
