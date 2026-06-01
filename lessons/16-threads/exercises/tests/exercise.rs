use threads_exercises::{double_in_thread, parallel_sum_of_squares};

// Warm-up: double_in_thread (spawn / move / join)

#[test]
fn warmup_double_positive() {
    assert_eq!(double_in_thread(5), 10);
}

#[test]
fn warmup_double_zero() {
    assert_eq!(double_in_thread(0), 0);
}

#[test]
fn warmup_double_negative() {
    assert_eq!(double_in_thread(-3), -6);
}

#[test]
fn warmup_double_large() {
    assert_eq!(double_in_thread(21), 42);
}

// Main: parallel_sum_of_squares (mpsc fan-out / collect)

#[test]
fn main_sum_empty() {
    assert_eq!(parallel_sum_of_squares(vec![]), 0);
}

#[test]
fn main_sum_one() {
    assert_eq!(parallel_sum_of_squares(vec![3]), 9);
}

#[test]
fn main_sum_many() {
    assert_eq!(parallel_sum_of_squares(vec![1, 2, 3]), 14);
}

#[test]
fn main_sum_negatives() {
    assert_eq!(parallel_sum_of_squares(vec![-2, 4]), 20);
}
