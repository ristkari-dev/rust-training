use axum::extract::{Path, State};
use axum_solutions::{AppState, greet, greet_with_state, router};

// Warm-up: greet (a handler with one extractor, called directly)

#[tokio::test]
async fn warmup_alice() {
    assert_eq!(greet(Path("Alice".to_string())).await, "Hello, Alice!");
}

#[tokio::test]
async fn warmup_bob() {
    assert_eq!(greet(Path("Bob".to_string())).await, "Hello, Bob!");
}

#[tokio::test]
async fn warmup_empty() {
    assert_eq!(greet(Path(String::new())).await, "Hello, !");
}

#[tokio::test]
async fn warmup_unicode() {
    assert_eq!(greet(Path("Zoë".to_string())).await, "Hello, Zoë!");
}

// Main: greet_with_state (State + Path extractors, called directly)

#[tokio::test]
async fn main_hi_bob() {
    let state = AppState {
        greeting: "Hi".to_string(),
    };
    assert_eq!(
        greet_with_state(State(state), Path("Bob".to_string())).await,
        "Hi, Bob!"
    );
}

#[tokio::test]
async fn main_welcome_sam() {
    let state = AppState {
        greeting: "Welcome".to_string(),
    };
    assert_eq!(
        greet_with_state(State(state), Path("Sam".to_string())).await,
        "Welcome, Sam!"
    );
}

#[tokio::test]
async fn main_hey_empty() {
    let state = AppState {
        greeting: "Hey".to_string(),
    };
    assert_eq!(
        greet_with_state(State(state), Path(String::new())).await,
        "Hey, !"
    );
}

#[tokio::test]
async fn main_router_builds() {
    // `router` wires the handlers into a Router; building it proves they
    // are valid axum handlers.
    let _router = router(AppState {
        greeting: "Hello".to_string(),
    });
}
