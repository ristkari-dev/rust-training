//! Lesson 11 — reference solutions.

use std::collections::HashMap;

#[must_use]
pub fn evens_squared(nums: &[i32]) -> Vec<i32> {
    nums.iter()
        .copied()
        .filter(|&n| n % 2 == 0)
        .map(|n| n * n)
        .collect()
}

#[must_use]
pub fn word_frequencies(text: &str) -> HashMap<String, usize> {
    let mut counts = HashMap::new();
    for word in text.split_whitespace() {
        *counts.entry(word.to_string()).or_insert(0) += 1;
    }
    counts
}

#[must_use]
pub fn most_frequent(text: &str) -> Option<String> {
    word_frequencies(text)
        .into_iter()
        .max_by_key(|(_word, count)| *count)
        .map(|(word, _count)| word)
}
