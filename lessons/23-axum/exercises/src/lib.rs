//! Lesson 23 — exercises.
//!
//! Implement `greet` (warm-up) and `greet_with_state` (main) so that
//! `cargo test --manifest-path lessons/23-axum/exercises/Cargo.toml`
//! passes. `AppState` and `router` are given. The tests live in
//! `tests/exercise.rs`.

use axum::Router;
use axum::extract::{Path, State};
use axum::routing::get;

/// Shared application state. Must be `Clone` for axum's `State`.
#[derive(Clone)]
pub struct AppState {
    pub greeting: String,
}

// The `_` prefixes keep the unfinished stubs compiling (unused variables
// are errors in this course). Rename to `name` / `state` when you use them.
pub async fn greet(Path(_name): Path<String>) -> String {
    todo!("return \"Hello, <name>!\"")
}

pub async fn greet_with_state(State(_state): State<AppState>, Path(_name): Path<String>) -> String {
    todo!("return \"<greeting>, <name>!\" using the state's greeting")
}

/// Build the router wiring both handlers, with shared state.
pub fn router(state: AppState) -> Router {
    Router::new()
        .route("/greet/{name}", get(greet))
        .route("/hello/{name}", get(greet_with_state))
        .with_state(state)
}
