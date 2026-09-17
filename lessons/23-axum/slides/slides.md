# HTTP services with Axum

> An HTTP service maps requests to responses. axum lets you write each endpoint as an ordinary async function — it pulls what you need out of the request for you, and routes each request to the right handler.

---

## What is axum

A modern async web framework built on **tokio**, **tower**, and **hyper** — the most popular Rust web framework.

You describe your endpoints as **handler** functions and wire them into a **router**; axum does the networking, parsing, and dispatch.

---

## Handlers

```rust
async fn hello() -> String {
    "Hello!".to_string()
}
```

A handler is an `async fn` whose return type implements `IntoResponse` — `String`, `(StatusCode, String)`, `Json<T>`, and many more. axum needs a handler to return a future, so it's written as an `async fn` — even one like this that never awaits.

---

## The `Router`

```rust
use axum::{Router, routing::get};

let app: Router = Router::new()
    .route("/", get(hello))
    .route("/greet/{name}", get(greet));
```

A `Router` maps a method + path to a handler. `get(hello)` says "on a GET to this path, call `hello`". `{name}` is a path parameter (axum 0.8 syntax — the old `:name` form panics).

---

## Extractors

A handler's parameters are **extractors** — each pulls a piece of the request:

- `Path<T>` — segments from the URL path
- `Query<T>` — the query string
- `State<T>` — shared application state
- `Json<T>` — a JSON request body

axum builds them from the request before calling your handler.

---

## The `Path` extractor

```rust
use axum::extract::Path;

async fn greet(Path(name): Path<String>) -> String {
    format!("Hello, {name}!")
}
```

With the route `/greet/{name}`, a request to `/greet/Alice` extracts `name = "Alice"`. The pattern `Path(name)` destructures the extractor to get the inner value.

---

## `State` — shared data

```rust
use axum::extract::State;

#[derive(Clone)]
struct AppState { greeting: String }

async fn greet_with_state(State(state): State<AppState>, Path(name): Path<String>) -> String {
    format!("{}, {name}!", state.greeting)
}
// ...Router::new().route(...).with_state(AppState { greeting: ... })
```

`State<T>` carries shared application data (config, a database pool, …). You attach it with `.with_state(...)`, and **the state type must be `Clone`** — axum clones it per request.

---

## Testing handlers

```rust
#[tokio::test]
async fn greets() {
    assert_eq!(greet(Path("Alice".to_string())).await, "Hello, Alice!");
}
```

A handler is just an `async fn`, so construct the extractors and call it directly — no TCP, no running server. (To actually serve it you'd bind a listener: `axum::serve(TcpListener::bind(addr).await?, app).await?`.)

---

## Putting it together

Today's exercises (`AppState` and `router()` are given):

- **Warm-up** `greet` — read a `Path<String>`, return a greeting
- **Main** `greet_with_state` — add `State<AppState>` to use the shared greeting

`router` wires both with `get` + `.with_state` — it compiling proves the handlers are valid axum handlers. The compile-fail forgets `#[derive(Clone)]` on the state.

---

## Wrap — services in Rust

- a handler is an `async fn` whose return type implements `IntoResponse`
- a `Router` maps method + path to handlers
- extractors (`Path`, `State`, …) pull request data
- the `State` type must be `Clone`
- test handlers by calling them directly

Next: **Lesson 24 — Persistence** (`sqlx`, migrations, transactions).
