//! Lesson 18 — exercises.
//!
//! Implement `sum_doubled` (warm-up) and `concurrent_sum_of_squares`
//! (main) so that `cargo test --manifest-path
//! lessons/18-async-tokio/exercises/Cargo.toml` passes. Both are async;
//! the tests `.await` them. The tests live in `tests/exercise.rs`.

// `#[allow(clippy::unused_async)]`: these functions MUST be `async` — the
// tests `.await` them — but the unfinished `todo!()` bodies contain no
// `.await` yet, which would otherwise trip clippy's `unused_async`. Your
// finished implementation will contain `.await`, so the allow is only
// needed while the body is a stub.

#[allow(clippy::unused_async)]
pub async fn sum_doubled(_a: i32, _b: i32) -> i32 {
    todo!("spawn a task to double each argument, await both, and return their sum")
}

#[allow(clippy::unused_async)]
pub async fn concurrent_sum_of_squares(_values: Vec<i32>) -> i32 {
    todo!("spawn a task per value to square it, await all handles, and sum the results")
}
