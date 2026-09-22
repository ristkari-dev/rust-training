# Observability

> A service you cannot see into is a service you cannot operate. The question is never "did it log something?" — it is "can I find the one request that failed, among a million that didn't?"

---

## Two kinds of signal

A **log** is one record per event: what happened, once, with detail.

A **metric** is aggregated state: how many, how long, right now.

You need both, and they answer different questions.

---

## Structured events

```rust
info!(path, status, elapsed_ms, "request finished");
```

The message is a constant; the values are named fields. `info!(path, ...)` is shorthand for `info!(path = path, ...)`.

Compare `info!("request {path} finished with {status}")` — just as readable for a human, but nothing can filter it by `status` afterwards.

---

## Levels

```rust
error!("payment provider unreachable");
warn!(retries, "retrying");
info!(path, "request finished");
debug!(rows, "executing");
trace!(offset, "loop tick");
```

Levels are the **reader's filter**, not your feelings: `error!` means someone should look, `info!` is the story of what happened.

---

## Spans

```rust
#[tracing::instrument]
fn handle(path: &str) {
    info!("started");   // carries path, because the span does
}
```

An event is a point; a **span** is an interval with a name and fields, and everything logged inside it inherits them.

---

## Subscribers

```rust
tracing_subscriber::fmt().init();                 // human-readable, INFO and up
tracing_subscriber::fmt().json().init();          // one JSON object per event
tracing_subscriber::fmt().with_max_level(tracing::Level::DEBUG).init();  // lower the bar
```

Nothing is recorded until something subscribes. With no subscriber the macros are **no-ops** — which is why a library can log freely and leave the choice to the binary.

---

## Other crates talk too

```text
DEBUG sqlx::query: summary="INSERT INTO accounts (name, …" rows_affected=1 elapsed=52.041µs
```

Install one subscriber and every instrumented dependency starts reporting. That is Lesson 24's database layer, unchanged — **you wrote none of that**. (sqlx logs at DEBUG, so the default INFO subscriber hides it.)

---

## Testing what you log

```rust
let ((), events) = capture(|| record_request("/health", 200, 3));
assert_eq!(events[0].field("status"), Some("200"));
```

`init()` is global and once per process, which parallel tests cannot share. The given `testkit::capture` scopes a subscriber to one closure and hands back your value **and** the events.

---

## Putting it together

Today's exercises (`testkit::capture`, `Metrics` and `render_prometheus` are given):

- **Warm-up** `record_request` — one event, values as named fields
- **Main** `Metrics::record` — count requests and 5xx errors with `AtomicU64`

`fetch_add` takes `&self`, so eight threads can count at once — no `Mutex`. The compile-fail enters a span that dies at the end of the statement.

---

## Wrap — observability in Rust

- the message is constant; the values are fields
- levels are the reader's filter
- a span gives every event inside it context
- a subscriber decides what is recorded, and where
- a counter is aggregated state you expose, not prose you log

Next: **Lesson 26 — Testing** (unit, integration, doc tests, property testing).
