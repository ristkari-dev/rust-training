use async_tokio_exercises::{concurrent_sum_of_squares, sum_doubled};

// Warm-up: sum_doubled (async fn + spawn + .await)

#[tokio::test]
async fn warmup_sum_doubled_basic() {
    assert_eq!(sum_doubled(3, 4).await, 14);
}

#[tokio::test]
async fn warmup_sum_doubled_zero() {
    assert_eq!(sum_doubled(0, 0).await, 0);
}

#[tokio::test]
async fn warmup_sum_doubled_negative() {
    assert_eq!(sum_doubled(-2, 5).await, 6);
}

#[tokio::test]
async fn warmup_sum_doubled_large() {
    assert_eq!(sum_doubled(10, 10).await, 40);
}

// Main: concurrent_sum_of_squares (spawn-per-value fan-out)

#[tokio::test]
async fn main_sum_empty() {
    assert_eq!(concurrent_sum_of_squares(vec![]).await, 0);
}

#[tokio::test]
async fn main_sum_one() {
    assert_eq!(concurrent_sum_of_squares(vec![3]).await, 9);
}

#[tokio::test]
async fn main_sum_many() {
    assert_eq!(concurrent_sum_of_squares(vec![1, 2, 3]).await, 14);
}

#[tokio::test]
async fn main_sum_negatives() {
    assert_eq!(concurrent_sum_of_squares(vec![-2, 4]).await, 20);
}
