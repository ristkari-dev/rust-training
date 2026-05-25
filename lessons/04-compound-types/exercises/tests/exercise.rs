use compound_types_exercises::{divmod, join_with_dashes};

// Warm-up: divmod

#[test]
fn warmup_typical() {
    assert_eq!(divmod(10, 3), (3, 1));
}

#[test]
fn warmup_exact() {
    assert_eq!(divmod(20, 4), (5, 0));
}

#[test]
fn warmup_divide_by_one() {
    assert_eq!(divmod(7, 1), (7, 0));
}

#[test]
fn warmup_zero_dividend() {
    assert_eq!(divmod(0, 5), (0, 0));
}

// Main: join_with_dashes

#[test]
fn main_empty() {
    assert_eq!(join_with_dashes(&[]), "");
}

#[test]
fn main_single() {
    assert_eq!(join_with_dashes(&["solo"]), "solo");
}

#[test]
fn main_two() {
    assert_eq!(join_with_dashes(&["a", "b"]), "a-b");
}

#[test]
fn main_three() {
    assert_eq!(
        join_with_dashes(&["red", "green", "blue"]),
        "red-green-blue"
    );
}
