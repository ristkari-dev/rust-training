//! Lesson 23 — reference solutions.

use axum::Router;
use axum::extract::{Path, State};
use axum::routing::get;

/// Shared application state. Must be `Clone` for axum's `State`.
#[derive(Clone)]
pub struct AppState {
    pub greeting: String,
}

pub async fn greet(Path(name): Path<String>) -> String {
    format!("Hello, {name}!")
}

pub async fn greet_with_state(State(state): State<AppState>, Path(name): Path<String>) -> String {
    format!("{}, {name}!", state.greeting)
}

/// Build the router wiring both handlers, with shared state.
pub fn router(state: AppState) -> Router {
    Router::new()
        .route("/greet/{name}", get(greet))
        .route("/hello/{name}", get(greet_with_state))
        .with_state(state)
}
