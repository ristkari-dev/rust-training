//! Lesson 18 — reference solutions.

pub async fn sum_doubled(a: i32, b: i32) -> i32 {
    let doubled_a = tokio::spawn(async move { a * 2 }).await.unwrap();
    let doubled_b = tokio::spawn(async move { b * 2 }).await.unwrap();
    doubled_a + doubled_b
}

pub async fn concurrent_sum_of_squares(values: Vec<i32>) -> i32 {
    let mut handles = Vec::new();
    for v in values {
        handles.push(tokio::spawn(async move { v * v }));
    }
    let mut total = 0;
    for handle in handles {
        total += handle.await.unwrap();
    }
    total
}
