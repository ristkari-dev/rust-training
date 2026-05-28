use iterators_collections_exercises::{evens_squared, most_frequent, word_frequencies};

// Warm-up: evens_squared

#[test]
fn warmup_empty() {
    assert_eq!(evens_squared(&[]), Vec::<i32>::new());
}

#[test]
fn warmup_all_odd() {
    assert_eq!(evens_squared(&[1, 3, 5]), Vec::<i32>::new());
}

#[test]
fn warmup_mixed() {
    assert_eq!(evens_squared(&[1, 2, 3, 4]), vec![4, 16]);
}

#[test]
fn warmup_all_even() {
    assert_eq!(evens_squared(&[2, 4, 6]), vec![4, 16, 36]);
}

// Main: word_frequencies + most_frequent

#[test]
fn main_frequencies_empty() {
    assert!(word_frequencies("").is_empty());
}

#[test]
fn main_frequencies_counts() {
    let counts = word_frequencies("the cat the dog the");
    assert_eq!(counts.get("the"), Some(&3));
    assert_eq!(counts.get("cat"), Some(&1));
    assert_eq!(counts.get("dog"), Some(&1));
    assert_eq!(counts.len(), 3);
}

#[test]
fn main_most_frequent_none_for_empty() {
    assert_eq!(most_frequent(""), None);
}

#[test]
fn main_most_frequent_unique_winner() {
    assert_eq!(
        most_frequent("apple apple banana"),
        Some("apple".to_string())
    );
}
