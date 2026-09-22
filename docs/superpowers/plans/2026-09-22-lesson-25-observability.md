# Lesson 25 — Observability — Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Author the last lesson of Phase 6: observability with `tracing`. The production skill: a log line is data, not prose — the message stays constant and the values go in named fields, so the output can be filtered and aggregated. Warm-up: `record_request` (one structured event). Main: `Metrics::record` (`AtomicU64` counters behind `&self`). Compile-fail: a span guard borrowing a temporary (E0716).

**Architecture:** Scaffold the lesson, add `tracing`, `tracing-subscriber` (with `json`) and `serde_json` to `[workspace.dependencies]`, and wire both lesson crates. A given `src/testkit.rs` installs a JSON subscriber scoped to one closure with `tracing::subscriber::with_default` and returns `(value, Vec<Event>)`, so tests assert on logs without a global subscriber — which `cargo test`'s parallelism could not share. `Metrics`, its fields and `render_prometheus` ship as given; students write `record_request` and `Metrics::record`. No external service, no C toolchain, no sqlx.

**Tech Stack:** Rust 2024 edition (toolchain 1.98), `tracing` 0.1.44, `tracing-subscriber` 0.3.23 with the `json` feature, `serde_json` 1 (an implementation detail of the given harness), existing tools (`new-lesson`, `compile-fails`, `slides-dev`, `build-index`), reveal.js (vendored), GNU Make.

**Spec:** [`docs/superpowers/specs/2026-09-22-lesson-25-observability-design.md`](../specs/2026-09-22-lesson-25-observability-design.md).

**Working directory:** `/Users/ristkari/code/private/rust-training`.

**Commit convention:** Plain commit messages only — no `Co-Authored-By` trailer or any AI attribution. If a commit fails with a GPG/pinentry error, simply retry the same `git commit` command once or twice.

## Global Constraints

- Lesson directory: `lessons/25-observability/`; Cargo package names `observability-exercises` and `observability-solutions` (import idents `observability_exercises` / `observability_solutions`).
- Root `Cargo.toml` `[workspace.dependencies]` gains exactly: `serde_json = "1"`, `tracing = "0.1"`, `tracing-subscriber = { version = "0.3", features = ["json"] }`. `tracing`'s default features stay on (`#[instrument]` lives behind its `attributes` feature); `tracing-subscriber`'s defaults already provide `fmt`.
- Both lesson crates declare all three as `{ workspace = true }` in a `[dependencies]` section after `[lints]`, and both carry a byte-identical `src/testkit.rs`.
- Workspace lints deny `clippy::all` + `clippy::pedantic`, and `unused` is denied (an unused variable is a compile error, which is why the stub's parameters are `_`-prefixed). No `#[allow]` attributes anywhere.
- `render_prometheus` is GIVEN, not an exercise: it is the only reader of the two counter fields, and with it stubbed out the denied `unused` turns the fields and the `Ordering` import into compile errors, so the stub would not build (verified during planning).
- Compile-fail files are std-only (the tool runs bare `rustc` with no `--extern`, so it cannot resolve `tracing`).
- Out of scope — do not add: OpenTelemetry, distributed tracing or propagation, log shipping, `EnvFilter`/`RUST_LOG` beyond a mention, custom `Layer` implementations, span sampling, metric labels or histograms, the `metrics` crate as a dependency, sqlx.
- All code in this plan was verified on rustc 1.98.1: solutions 8/8 tests pass, the stub compiles with all 8 tests panicking, both are clippy- and rustfmt-clean. If clippy or rustfmt fires, do NOT add an `#[allow]` and do NOT change the code; STOP and report the exact output.

## Deviations from the spec

- **Slides 4, 6 and 8 gain code blocks** the spec's prose-only slides did not have (one call per level; the three subscriber forms; the `capture` call), and slide 7 is titled "Other crates talk too".
- **On a firing lint, STOP instead of fixing.** The spec says to fix the code and report. This plan's code was verified clippy- and rustfmt-clean on rustc 1.98.1 against tracing 0.1.44 / tracing-subscriber 0.3.23, so a firing lint means toolchain or dependency drift worth reporting, not patching.

---

## Task 1: Scaffold lessons/25-observability

**Files (all created by the scaffolder):**
- `lessons/25-observability/README.md` (placeholder, replaced in Task 5)
- `lessons/25-observability/slides/index.html` (final — no edit needed)
- `lessons/25-observability/slides/slides.md` (placeholder, replaced in Task 6)
- `lessons/25-observability/exercises/Cargo.toml` (dependencies added in Task 2)
- `lessons/25-observability/exercises/src/lib.rs` (placeholder, replaced in Task 3)
- `lessons/25-observability/exercises/tests/exercise.rs` (placeholder, replaced in Task 3)
- `lessons/25-observability/solutions/Cargo.toml` (dependencies added in Task 2)
- `lessons/25-observability/solutions/src/lib.rs` (placeholder, replaced in Task 4)
- `lessons/25-observability/solutions/tests/exercise.rs` (placeholder, replaced in Task 4)

**Interfaces:**
- Consumes: nothing.
- Produces: workspace members `observability-exercises` and `observability-solutions`.

- [ ] **Step 1: Run the scaffolder**

```bash
make new-lesson NAME=25-observability
```

Expected: `scaffolded ./lessons/25-observability`.

- [ ] **Step 2: Verify directory structure**

```bash
ls lessons/25-observability/
ls lessons/25-observability/slides/ lessons/25-observability/exercises/ lessons/25-observability/solutions/
```

Expected: top-level `README.md`, `slides/`, `exercises/`, `solutions/` populated from templates.

- [ ] **Step 3: Verify Cargo package names**

```bash
grep '^name' lessons/25-observability/exercises/Cargo.toml lessons/25-observability/solutions/Cargo.toml
```

Expected:
```
lessons/25-observability/exercises/Cargo.toml:name = "observability-exercises"
lessons/25-observability/solutions/Cargo.toml:name = "observability-solutions"
```

- [ ] **Step 4: Verify the workspace picks up the new crates**

```bash
cargo metadata --no-deps --format-version 1 | grep -o '"name":"observability-[^"]*"' | sort -u
```

Expected output:
```
"name":"observability-exercises"
"name":"observability-solutions"
```

- [ ] **Step 5: Verify the scaffolded workspace builds clean**

```bash
cargo build --workspace
```

Expected: warning-free build.

- [ ] **Step 6: Commit**

```bash
git add lessons/25-observability
git commit -m "chore: scaffold lessons/25-observability"
```

---

## Task 2: Add the tracing dependencies and wire the lesson crates

**Files:**
- Modify: `Cargo.toml` (root)
- Modify: `lessons/25-observability/exercises/Cargo.toml`
- Modify: `lessons/25-observability/solutions/Cargo.toml`

**Interfaces:**
- Consumes: the two scaffolded manifests from Task 1.
- Produces: `tracing`, `tracing-subscriber` and `serde_json` available to both lesson crates.

- [ ] **Step 1: Add the three entries to the root `[workspace.dependencies]`**

Insert `serde_json` after `clap`, and `tracing` / `tracing-subscriber` after `toml_edit`, so the table reads:

```toml
[workspace.dependencies]
anyhow = "1"
axum = "0.8"
clap = { version = "4", features = ["derive"] }
serde_json = "1"
sqlx = { version = "0.8", default-features = false, features = ["runtime-tokio", "sqlite", "migrate", "macros"] }
thiserror = "2"
tokio = { version = "1", features = ["rt", "macros"] }
toml_edit = "0.22"
tracing = "0.1"
tracing-subscriber = { version = "0.3", features = ["json"] }
walkdir = "2"
tempfile = "3"
tiny_http = "0.12"
```

Change nothing else in the root `Cargo.toml`.

- [ ] **Step 2: Add the dependencies to `lessons/25-observability/exercises/Cargo.toml`**

Append a `[dependencies]` section after `[lints]`, so the whole file reads:

```toml
[package]
name = "observability-exercises"
version = "0.1.0"
edition.workspace = true
rust-version.workspace = true
license.workspace = true
publish.workspace = true

[lints]
workspace = true

[dependencies]
serde_json = { workspace = true }
tracing = { workspace = true }
tracing-subscriber = { workspace = true }
```

- [ ] **Step 3: Add the dependencies to `lessons/25-observability/solutions/Cargo.toml`**

Same change for the solutions crate:

```toml
[package]
name = "observability-solutions"
version = "0.1.0"
edition.workspace = true
rust-version.workspace = true
license.workspace = true
publish.workspace = true

[lints]
workspace = true

[dependencies]
serde_json = { workspace = true }
tracing = { workspace = true }
tracing-subscriber = { workspace = true }
```

- [ ] **Step 4: Build to confirm the dependencies resolve**

```bash
cargo build --workspace
```

Expected: a warning-free build, exit 0, in a few seconds. Only 7 packages are new — cargo prints `Locking 7 packages`, adding `tracing-subscriber` and its tree (`tracing-log`, `tracing-serde`, `sharded-slab`, `thread_local`, `nu-ansi-term`, `valuable`); `serde_json` and `tracing` are already in the workspace graph via sqlx and axum, so adding them to `[workspace.dependencies]` locks nothing new. No C toolchain is involved.

- [ ] **Step 5: Verify the resolved versions**

```bash
cargo tree --package observability-solutions --depth 1
```

Expected: lists `serde_json v1.x`, `tracing v0.1.x` and `tracing-subscriber v0.3.x`.

- [ ] **Step 6: Commit**

```bash
git add Cargo.toml lessons/25-observability/exercises/Cargo.toml lessons/25-observability/solutions/Cargo.toml
git commit -m "build(lesson-25): add tracing dependencies and wire the lesson crates"
```

---

## Task 3: Exercise content (testkit, stub, tests, compile-fail)

**Files:**
- Create: `lessons/25-observability/exercises/src/testkit.rs`
- Overwrite: `lessons/25-observability/exercises/src/lib.rs`
- Overwrite: `lessons/25-observability/exercises/tests/exercise.rs`
- Create: `lessons/25-observability/exercises/compile_fails/25-span-guard-temporary.rs`

**Interfaces:**
- Consumes: the dependencies from Task 2.
- Produces (the public API of `observability_exercises`, mirrored exactly by `observability_solutions` in Task 4):
  - `pub mod testkit` with `pub struct Event { pub level: String, pub message: String, pub fields: BTreeMap<String, String> }`, `Event::field(&self, &str) -> Option<&str>`, and `pub fn capture<T>(f: impl FnOnce() -> T) -> (T, Vec<Event>)`
  - `pub fn record_request(path: &str, status: u64, elapsed_ms: u64)`
  - `#[derive(Debug, Default)] pub struct Metrics` with `pub fn record(&self, status: u64)` and `pub fn render_prometheus(&self) -> String`

- [ ] **Step 1: Create `lessons/25-observability/exercises/src/testkit.rs`**

The given capture harness. Write EXACTLY as shown:

```rust
//! GIVEN: capture what your code logs, so a test can assert on it.
//!
//! `capture` installs a JSON subscriber for the duration of one closure and
//! nothing else — not globally — so tests stay independent even though
//! `cargo test` runs them in parallel.
//!
//! You only ever call `capture`. The rest of this file is plumbing you are
//! not expected to follow: `Buffer` is a `Vec<u8>` behind an `Arc<Mutex<_>>`
//! that the subscriber writes into, and `parse` turns each JSON line back
//! into an `Event`.

use std::collections::BTreeMap;
use std::io;
use std::sync::{Arc, Mutex};

use tracing_subscriber::fmt::MakeWriter;

/// One captured event.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Event {
    /// `INFO`, `WARN`, `ERROR`, …
    pub level: String,
    /// The event's constant message.
    pub message: String,
    /// Every other field, as text.
    pub fields: BTreeMap<String, String>,
}

impl Event {
    /// The value of one field, or `None` if the event doesn't carry it.
    #[must_use]
    pub fn field(&self, name: &str) -> Option<&str> {
        self.fields.get(name).map(String::as_str)
    }
}

#[derive(Clone, Default)]
struct Buffer(Arc<Mutex<Vec<u8>>>);

impl io::Write for Buffer {
    fn write(&mut self, data: &[u8]) -> io::Result<usize> {
        self.0.lock().unwrap().extend_from_slice(data);
        Ok(data.len())
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

impl<'a> MakeWriter<'a> for Buffer {
    type Writer = Self;

    fn make_writer(&'a self) -> Self::Writer {
        self.clone()
    }
}

fn parse(raw: &str) -> Vec<Event> {
    raw.lines()
        .map(|line| {
            let value: serde_json::Value =
                serde_json::from_str(line).expect("captured line was not JSON");
            let mut fields: BTreeMap<String, String> = value["fields"]
                .as_object()
                .expect("event had no fields")
                .iter()
                .map(|(k, v)| (k.clone(), text(v)))
                .collect();
            let message = fields.remove("message").unwrap_or_default();
            Event {
                level: value["level"].as_str().unwrap_or_default().to_string(),
                message,
                fields,
            }
        })
        .collect()
}

fn text(value: &serde_json::Value) -> String {
    match value {
        serde_json::Value::String(s) => s.clone(),
        other => other.to_string(),
    }
}

/// Run `f` with a private subscriber; return its value and what it logged.
pub fn capture<T>(f: impl FnOnce() -> T) -> (T, Vec<Event>) {
    let buffer = Buffer::default();
    let subscriber = tracing_subscriber::fmt()
        .json()
        .with_writer(buffer.clone())
        .without_time()
        .with_max_level(tracing::Level::TRACE)
        .finish();
    let out = tracing::subscriber::with_default(subscriber, f);
    let raw = String::from_utf8(buffer.0.lock().unwrap().clone()).expect("log was not UTF-8");
    (out, parse(&raw))
}
```

- [ ] **Step 2: Overwrite `lessons/25-observability/exercises/src/lib.rs`**

`Metrics`, its fields and `render_prometheus` ship complete; the two exercises are `todo!()`. Write EXACTLY as shown:

```rust
//! Lesson 25 — exercises.
//!
//! Implement `record_request` (warm-up) and `Metrics::record` (main) so that
//! `cargo test --manifest-path lessons/25-observability/exercises/Cargo.toml`
//! passes. `testkit::capture` and `Metrics::render_prometheus` are given.
//! The tests live in `tests/exercise.rs`.
//!
//! You add `use tracing::info;` yourself — the stub ships without it,
//! because an unused import is a compile error in this course.

pub mod testkit;

use std::sync::atomic::{AtomicU64, Ordering};

/// Record that a request finished.
///
/// One event: a constant message, with the values as named fields.
// The `_` prefixes keep the unfinished stubs compiling (unused variables
// are errors in this course). Rename them when you write the body.
pub fn record_request(_path: &str, _status: u64, _elapsed_ms: u64) {
    todo!("emit ONE event: a constant message, with the values as named fields")
}

/// Counters a service exposes for scraping.
#[derive(Debug, Default)]
pub struct Metrics {
    requests: AtomicU64,
    errors: AtomicU64,
}

impl Metrics {
    /// Count one finished request, and one error if the status is 5xx.
    pub fn record(&self, _status: u64) {
        todo!("count the request, and the error too when the status is 5xx")
    }

    /// GIVEN: render the counters in Prometheus' text exposition format.
    ///
    /// This is what a `/metrics` endpoint serves; a scraper reads it every
    /// few seconds and keeps the numbers over time.
    #[must_use]
    pub fn render_prometheus(&self) -> String {
        let requests = self.requests.load(Ordering::Relaxed);
        let errors = self.errors.load(Ordering::Relaxed);
        format!(
            "# HELP requests_total Requests served.\n\
             # TYPE requests_total counter\n\
             requests_total {requests}\n\
             # HELP errors_total Requests that failed.\n\
             # TYPE errors_total counter\n\
             errors_total {errors}\n"
        )
    }
}
```

- [ ] **Step 3: Overwrite `lessons/25-observability/exercises/tests/exercise.rs`**

```rust
use std::sync::Arc;
use std::thread;

use observability_exercises::testkit::capture;
use observability_exercises::{Metrics, record_request};

// Warm-up: record_request (one event, values as named fields)

#[test]
fn warmup_emits_exactly_one_event() {
    let ((), events) = capture(|| record_request("/health", 200, 3));
    assert_eq!(events.len(), 1, "expected one event, got {events:?}");
    assert_eq!(events[0].level, "INFO");
}

#[test]
fn warmup_values_are_fields() {
    let ((), events) = capture(|| record_request("/health", 200, 3));
    let event = events
        .first()
        .expect("no event was recorded - record_request must emit one");
    assert_eq!(
        event.field("path"),
        Some("/health"),
        "None means the event carries no field called `path` - the values belong beside the message, not inside it"
    );
    assert_eq!(event.field("status"), Some("200"));
    assert_eq!(event.field("elapsed_ms"), Some("3"));
}

#[test]
fn warmup_message_is_constant() {
    let ((), one) = capture(|| record_request("/health", 200, 3));
    let ((), two) = capture(|| record_request("/orders", 500, 41));
    let (one, two) = (
        one.first().expect("no event recorded for /health"),
        two.first().expect("no event recorded for /orders"),
    );
    assert_eq!(
        one.message, two.message,
        "the message must not change with the values - put them in fields, not in the text"
    );
}

#[test]
fn warmup_fields_survive_different_values() {
    let ((), events) = capture(|| record_request("/orders", 503, 41));
    let event = events
        .first()
        .expect("no event was recorded - record_request must emit one");
    assert_eq!(
        event.field("path"),
        Some("/orders"),
        "the same fields must appear whatever the values are"
    );
    assert_eq!(event.field("status"), Some("503"));
    assert_eq!(event.field("elapsed_ms"), Some("41"));
}

// Main: Metrics (counters you expose, not prose you log)

#[test]
fn main_counts_requests() {
    let metrics = Metrics::default();
    metrics.record(200);
    metrics.record(404);
    assert!(metrics.render_prometheus().contains("requests_total 2"));
}

#[test]
fn main_counts_server_errors_only() {
    let metrics = Metrics::default();
    metrics.record(200);
    metrics.record(404);
    metrics.record(500);
    metrics.record(503);
    let rendered = metrics.render_prometheus();
    assert!(
        rendered.contains("errors_total 2"),
        "only 5xx counts as an error: {rendered}"
    );
}

#[test]
fn main_counters_are_shared_across_threads() {
    let metrics = Arc::new(Metrics::default());
    let handles: Vec<_> = (0..8)
        .map(|_| {
            let metrics = Arc::clone(&metrics);
            thread::spawn(move || {
                for _ in 0..1000 {
                    metrics.record(200);
                }
            })
        })
        .collect();
    for handle in handles {
        handle.join().unwrap();
    }
    assert!(
        metrics.render_prometheus().contains("requests_total 8000"),
        "8 threads x 1000 requests must all be counted"
    );
}

#[test]
fn main_renders_the_exposition_format() {
    let metrics = Metrics::default();
    metrics.record(200);
    metrics.record(500);
    assert_eq!(
        metrics.render_prometheus(),
        "# HELP requests_total Requests served.\n\
         # TYPE requests_total counter\n\
         requests_total 2\n\
         # HELP errors_total Requests that failed.\n\
         # TYPE errors_total counter\n\
         errors_total 1\n"
    );
}
```

- [ ] **Step 4: Create `lessons/25-observability/exercises/compile_fails/25-span-guard-temporary.rs`**

The `compile_fails/` directory does not exist yet — create it. This file is self-contained and std-only (it must NOT use `tracing`).

```rust
// Compile-fail exercise: this file MUST NOT compile until you fix it.
//
// Entering a span with `Span::enter` hands you a GUARD that borrows the
// span, and the span stays open until the guard drops. So the span itself
// has to outlive the guard — if you create it and enter it in one
// expression, the span is a temporary that dies at the end of that
// statement, while the guard is still holding a borrow of it.
//
// This file reproduces that with plain structs (`Span::enter` here stands in
// for tracing's). rustc reports E0716: "temporary value dropped while
// borrowed".
//
// The fix: give the span a name first, so it lives as long as the guard —
//     let span = span("handle_request");
//     let guard = span.enter();

struct Span {
    name: String,
}

struct Entered<'a> {
    span: &'a Span,
}

impl Span {
    fn enter(&self) -> Entered<'_> {
        Entered { span: self }
    }
}

fn span(name: &str) -> Span {
    Span {
        name: name.to_string(),
    }
}

fn main() {
    let guard = span("handle_request").enter();
    println!("inside the span for {}", guard.span.name);
}
```

- [ ] **Step 5: Verify exercise tests compile and fail with `todo!()` panics (intentional)**

```bash
cargo test --manifest-path lessons/25-observability/exercises/Cargo.toml
```

Expected: the crate COMPILES, then ALL 8 tests FAIL. Seven panic directly with `not yet implemented`; `main_counters_are_shared_across_threads` panics the same way inside its worker threads and then fails on `join().unwrap()` with `called \`Result::unwrap()\` on an \`Err\` value: Any { .. }`. Result line: `test result: FAILED. 0 passed; 8 failed`. This is the correct undone state.

- [ ] **Step 6: Verify the exercises crate builds cleanly**

```bash
cargo build --package observability-exercises
```

Expected: warning-free build.

- [ ] **Step 7: Verify compile-fail ships broken**

```bash
cargo run --package compile-fails -- --expect broken lessons/25-observability
```

Expected: prints `ok:   lessons/25-observability/exercises/compile_fails/25-span-guard-temporary.rs` and exits 0. (The tool printing the rustc E0716 error text is expected — what matters is the final `ok:` line and exit 0.)

- [ ] **Step 8: Verify compile-fail's student-mode check fires**

```bash
cargo run --package compile-fails -- --expect compiles lessons/25-observability
```

Expected: non-zero exit with a `FAIL: ...` message naming the file. (Correct — it ships broken on purpose.)

- [ ] **Step 9: Verify lint passes on the exercises crate**

```bash
cargo clippy --package observability-exercises --all-targets -- -D warnings
cargo fmt --check --package observability-exercises
```

Expected: both exit 0.

- [ ] **Step 10: Commit**

```bash
git add lessons/25-observability/exercises
git commit -m "feat(lesson-25): add the capture testkit, exercise stubs, tests, and compile-fail"
```

---

## Task 4: Reference solutions

**Files:**
- Create: `lessons/25-observability/solutions/src/testkit.rs`
- Overwrite: `lessons/25-observability/solutions/src/lib.rs`
- Overwrite: `lessons/25-observability/solutions/tests/exercise.rs`

**Interfaces:**
- Consumes: the dependencies from Task 2.
- Produces: `observability_solutions` with the identical public API listed in Task 3, with real bodies for `record_request` and `Metrics::record`.

- [ ] **Step 1: Create `lessons/25-observability/solutions/src/testkit.rs`**

Copy the exercises version, so byte-identity is structural rather than aspirational:

```bash
cp lessons/25-observability/exercises/src/testkit.rs lessons/25-observability/solutions/src/testkit.rs
```

Step 4 verifies it with `diff`.

- [ ] **Step 2: Overwrite `lessons/25-observability/solutions/src/lib.rs`**

Write EXACTLY as shown:

```rust
//! Lesson 25 — reference solutions.

pub mod testkit;

use std::sync::atomic::{AtomicU64, Ordering};

use tracing::info;

/// Record that a request finished.
///
/// One event: a constant message, with the values as named fields.
pub fn record_request(path: &str, status: u64, elapsed_ms: u64) {
    info!(path, status, elapsed_ms, "request finished");
}

/// Counters a service exposes for scraping.
#[derive(Debug, Default)]
pub struct Metrics {
    requests: AtomicU64,
    errors: AtomicU64,
}

impl Metrics {
    /// Count one finished request, and one error if the status is 5xx.
    pub fn record(&self, status: u64) {
        self.requests.fetch_add(1, Ordering::Relaxed);
        if status >= 500 {
            self.errors.fetch_add(1, Ordering::Relaxed);
        }
    }

    /// GIVEN: render the counters in Prometheus' text exposition format.
    ///
    /// This is what a `/metrics` endpoint serves; a scraper reads it every
    /// few seconds and keeps the numbers over time.
    #[must_use]
    pub fn render_prometheus(&self) -> String {
        let requests = self.requests.load(Ordering::Relaxed);
        let errors = self.errors.load(Ordering::Relaxed);
        format!(
            "# HELP requests_total Requests served.\n\
             # TYPE requests_total counter\n\
             requests_total {requests}\n\
             # HELP errors_total Requests that failed.\n\
             # TYPE errors_total counter\n\
             errors_total {errors}\n"
        )
    }
}
```

> Pedagogical notes:
> - `record_request` is the whole structured-logging idea in one line. The field shorthand (`path` rather than `path = path`) is the idiom students meet everywhere, and the constant message is what makes the event aggregatable. `warmup_message_is_constant` is the test that enforces it: an interpolated message makes the two captured messages differ.
> - `Metrics::record` takes `&self`, not `&mut self` — `AtomicU64::fetch_add` mutates through a shared reference, which is why one `Arc<Metrics>` can be counted into from eight threads with no `Mutex`. That is Lesson 16's `Send`/`Sync` payoff, and the course's only appearance of atomics.
> - `Ordering::Relaxed` is correct because these counters synchronize nothing else; they only need to not lose increments. The README says that in one sentence rather than opening the memory-ordering rabbit hole.
> - `render_prometheus` carries `#[must_use]` (it returns a `String` and has no side effect); `record` and `record_request` return `()`, so they do not.

- [ ] **Step 3: Overwrite `lessons/25-observability/solutions/tests/exercise.rs`**

Identical to the exercises copy except the crate ident in the `use` lines:

```rust
use std::sync::Arc;
use std::thread;

use observability_solutions::testkit::capture;
use observability_solutions::{Metrics, record_request};

// Warm-up: record_request (one event, values as named fields)

#[test]
fn warmup_emits_exactly_one_event() {
    let ((), events) = capture(|| record_request("/health", 200, 3));
    assert_eq!(events.len(), 1, "expected one event, got {events:?}");
    assert_eq!(events[0].level, "INFO");
}

#[test]
fn warmup_values_are_fields() {
    let ((), events) = capture(|| record_request("/health", 200, 3));
    let event = events
        .first()
        .expect("no event was recorded - record_request must emit one");
    assert_eq!(
        event.field("path"),
        Some("/health"),
        "None means the event carries no field called `path` - the values belong beside the message, not inside it"
    );
    assert_eq!(event.field("status"), Some("200"));
    assert_eq!(event.field("elapsed_ms"), Some("3"));
}

#[test]
fn warmup_message_is_constant() {
    let ((), one) = capture(|| record_request("/health", 200, 3));
    let ((), two) = capture(|| record_request("/orders", 500, 41));
    let (one, two) = (
        one.first().expect("no event recorded for /health"),
        two.first().expect("no event recorded for /orders"),
    );
    assert_eq!(
        one.message, two.message,
        "the message must not change with the values - put them in fields, not in the text"
    );
}

#[test]
fn warmup_fields_survive_different_values() {
    let ((), events) = capture(|| record_request("/orders", 503, 41));
    let event = events
        .first()
        .expect("no event was recorded - record_request must emit one");
    assert_eq!(
        event.field("path"),
        Some("/orders"),
        "the same fields must appear whatever the values are"
    );
    assert_eq!(event.field("status"), Some("503"));
    assert_eq!(event.field("elapsed_ms"), Some("41"));
}

// Main: Metrics (counters you expose, not prose you log)

#[test]
fn main_counts_requests() {
    let metrics = Metrics::default();
    metrics.record(200);
    metrics.record(404);
    assert!(metrics.render_prometheus().contains("requests_total 2"));
}

#[test]
fn main_counts_server_errors_only() {
    let metrics = Metrics::default();
    metrics.record(200);
    metrics.record(404);
    metrics.record(500);
    metrics.record(503);
    let rendered = metrics.render_prometheus();
    assert!(
        rendered.contains("errors_total 2"),
        "only 5xx counts as an error: {rendered}"
    );
}

#[test]
fn main_counters_are_shared_across_threads() {
    let metrics = Arc::new(Metrics::default());
    let handles: Vec<_> = (0..8)
        .map(|_| {
            let metrics = Arc::clone(&metrics);
            thread::spawn(move || {
                for _ in 0..1000 {
                    metrics.record(200);
                }
            })
        })
        .collect();
    for handle in handles {
        handle.join().unwrap();
    }
    assert!(
        metrics.render_prometheus().contains("requests_total 8000"),
        "8 threads x 1000 requests must all be counted"
    );
}

#[test]
fn main_renders_the_exposition_format() {
    let metrics = Metrics::default();
    metrics.record(200);
    metrics.record(500);
    assert_eq!(
        metrics.render_prometheus(),
        "# HELP requests_total Requests served.\n\
         # TYPE requests_total counter\n\
         requests_total 2\n\
         # HELP errors_total Requests that failed.\n\
         # TYPE errors_total counter\n\
         errors_total 1\n"
    );
}
```

- [ ] **Step 4: Verify the duplicated testkit is byte-identical**

```bash
diff lessons/25-observability/exercises/src/testkit.rs lessons/25-observability/solutions/src/testkit.rs
```

Expected: prints nothing and exits 0.

- [ ] **Step 5: Verify solution tests pass**

```bash
cargo test --package observability-solutions
```

Expected: `test result: ok. 8 passed`.

- [ ] **Step 6: Verify lint passes on the solutions crate**

```bash
cargo clippy --package observability-solutions --all-targets -- -D warnings
cargo fmt --check --package observability-solutions
```

Expected: both exit 0. The code above is exactly correct as written — do NOT modify it. If clippy or rustfmt fires, do NOT add an `#[allow]` and do NOT change the code; STOP and report the exact output.

- [ ] **Step 7: Commit**

```bash
git add lessons/25-observability/solutions
git commit -m "feat(lesson-25): add reference solutions"
```

---

## Task 5: Lesson README

**Files:**
- Overwrite: `lessons/25-observability/README.md`

**Interfaces:**
- Consumes: the names from Tasks 3-4.
- Produces: nothing code-facing.

- [ ] **Step 1: Overwrite `lessons/25-observability/README.md`**

The complete file content is below, delimited by an OUTER quadruple-backtick fence (` ```` `). That outer fence is ONLY a delimiter for this plan — do NOT write it into the file. The file must start with `# Lesson 25` on line 1 and contain only PLAIN triple-backtick (` ``` `) code fences.

````markdown
# Lesson 25 — Observability

A service you cannot see into is a service you cannot operate. The question
is never "did it log something?" — it is "can I find the one request that
failed, among a million that didn't?" `tracing` turns what your code did
into structured events you can filter, and counters turn what it did a lot
of into numbers you can graph. The production skill: **a log line is data,
not prose**.

## Learning goals

- Emit a structured event with `tracing::info!`, keeping the message
  constant and putting the values in named fields
- Explain why `info!(path, status, "request finished")` is worth more than
  `info!("request {path} finished with {status}")`
- Test what your code logs, with a subscriber scoped to one closure rather
  than a global one
- Count events with `AtomicU64` counters behind a shared `&self`, and know
  why a counter needs no `Mutex`
- Tell a log from a metric, and read Prometheus' text exposition format

## Self-study notes

### Events, and why fields beat interpolation

An event is one record of something that happened. Write the message as a
constant and put the values beside it, as named fields:

```rust
use tracing::info;

info!(path, status, elapsed_ms, "request finished");
// as testkit::capture records it (a plain .json() subscriber also emits
// a "timestamp"):
// {"level":"INFO","fields":{"message":"request finished","path":"/health",
//  "status":200,"elapsed_ms":3},"target":"observability_exercises"}
```

`info!(path, ...)` is shorthand for `info!(path = path, ...)`, and the
message always comes **last** — fields first, message at the end. Put it
first and rustc offers to add `{}` placeholders for your fields; that
suggestion is the anti-pattern this lesson exists to prevent. The
interpolated version — `info!("request {path} finished with {status}")` —
reads just as well and throws the structure away: you
can grep it, but you cannot ask "how many requests returned 500 on
`/orders` last hour?" without parsing English back into data. Fields are
what make that question answerable.

### Levels and spans

Every event has a level: `error!`, `warn!`, `info!`, `debug!`, `trace!`.
Levels are the reader's filter, not a mood — `error!` means someone should
look, `info!` is the story of what happened.

An event is a point in time; a **span** is an interval with a name and
fields, and everything logged inside it inherits them:

```rust
#[tracing::instrument]
fn handle(path: &str) {
    info!("started");   // printed inside handle{path=…}, because the span carries it
}
```

`#[instrument]` opens a span for the whole function and records its
arguments as fields. This lesson doesn't exercise spans; add
`#[tracing::instrument]` to a function of your own and the subscriber
starts printing `handle{path=…}` around everything it logs.

### Subscribers — what records, and where

Nothing is recorded until something subscribes. Without a subscriber the
macros are no-ops, which is why a library can log freely and leave the
decision to the binary:

```rust
fn main() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG) // the default is INFO
        .init();                               // human-readable lines
    // tracing_subscriber::fmt().json().init(); // one JSON object per event
}
```

The subscriber decides the format, the destination and the minimum level.
It is also where you get other crates' telemetry for free: install one, and
every instrumented dependency starts talking. Lesson 24's database layer,
unchanged, emits lines like `DEBUG sqlx::query: summary="INSERT INTO
accounts (name, …" rows_affected=1 elapsed=52.041µs …` (abridged — sqlx
also records `db.statement`, `rows_returned` and `elapsed_secs`). You wrote none of that.
Note the level: sqlx logs queries at DEBUG, so the default INFO subscriber
hides them — raise the level as above, or turn on `tracing-subscriber`'s
`env-filter` feature and filter with `EnvFilter` and `RUST_LOG=sqlx=debug`.

### Testing what you log

If logs are data, you can assert on them. The catch: `init()` installs a
subscriber globally, once per process, and `cargo test` runs your tests in
parallel in one process — so tests would fight over it. Scope one to a
closure instead:

```rust
let ((), events) = capture(|| record_request("/health", 200, 3));
assert_eq!(events[0].field("status"), Some("200"));
```

`testkit::capture` is given. It installs a JSON subscriber for the duration
of the closure with `tracing::subscriber::with_default`, then hands back
your return value and the events it captured. It switches off timestamps,
because a test asserting on output needs that output to be the same every
run.

The scoping is per *thread*: `with_default` sets the subscriber for the
thread that calls it, and `cargo test` gives each test its own thread —
that is what makes parallel tests independent. The flip side: `capture`
does not see events logged from threads you spawn inside the closure.

### Metrics — counters you expose

A log is one record per event. A metric is aggregated state: how many, how
long, right now.

`AtomicU64` is a `u64` from `std::sync::atomic` that several threads may
touch at once. You don't read and write it with `=`; you call methods —
`fetch_add(n, ordering)` adds and returns the previous value, `load(ordering)`
reads it — and each one is indivisible, so two threads incrementing at the
same moment cannot lose an increment between them:

```rust
pub struct Counter {
    hits: AtomicU64,
}

impl Counter {
    pub fn hit(&self) {
        self.hits.fetch_add(1, Ordering::Relaxed);
    }
}
```

`fetch_add` mutates through `&self`, so eight threads can count at once
with no `Mutex` (Lesson 16's `Send`/`Sync` paying off). `Ordering::Relaxed` is right here
because these counters guard nothing else — they only need to not lose
increments. What a scraper reads from a `/metrics` endpoint is three lines per
counter — a `# HELP` description, a `# TYPE` (`counter` means it only ever
goes up), then the name and the value:

```text
# HELP requests_total Requests served.
# TYPE requests_total counter
requests_total 2
```

It reads that every few seconds and keeps the numbers over time; that is
all Prometheus is, at the bottom. Real services reach for the `metrics`
crate or OpenTelemetry rather than hand-rolling — this lesson hand-rolls so
the mechanics stay visible: one `AtomicU64`, no extra dependency.

## Exercises

### Warm-up: `record_request`

Implement `record_request` so it emits exactly one event, with the values
as named fields:

```rust
pub fn record_request(_path: &str, _status: u64, _elapsed_ms: u64) {
    todo!("emit ONE event: a constant message, with the values as named fields")
}
```

The stub's parameters start with `_` because this course's lints make an
unused variable a compile error — rename them in the same edit where you
replace `todo!()`. You add `use tracing::info;` yourself too (or call
`tracing::info!` in full, as in the Events section above); rustc will only
say `cannot find macro`, with no suggestion. One test calls the function twice with different values
and asserts the two messages are equal: an interpolated message fails it,
which is the whole point.

### Main: `Metrics::record`

Implement `Metrics::record` so it counts every request, and counts an error
as well when the status is 5xx:

```rust
pub fn record(&self, _status: u64) {
    todo!("count the request, and the error too when the status is 5xx")
}
```

Note the signature: `&self`, not `&mut self` — see the Metrics note above.
One of the tests counts into one `Arc<Metrics>` from eight threads at once. If you write `&mut self` instead, every error rustc prints will
point at `tests/exercise.rs` rather than your code, and several will
suggest adding `mut` to a binding in the test — don't: the tests are right,
the signature is the thing to change. `Metrics`, its fields and
`render_prometheus` are given; read `render_prometheus` to see what your
counters turn into.

### Compile-fail

`exercises/compile_fails/25-span-guard-temporary.rs` enters a span in the
same expression that creates it, so the span dies at the end of the
statement while the guard still borrows it (E0716 — temporary value dropped
while borrowed). Fix it by binding the span to a name first; rustc's own
suggestion shows the shape. You can check this one without waiting for the
tests:
`cargo run --package compile-fails -- --expect compiles lessons/25-observability`.

### Run

```bash
make verify LESSON=25-observability
```

This runs your exercise tests and asserts the compile-fail file now
compiles.

## Solutions

See `solutions/src/lib.rs` for the reference implementations. Try the
exercises before peeking.
````

- [ ] **Step 2: Spot-check the README**

```bash
head -1 lessons/25-observability/README.md
grep -c '^### ' lessons/25-observability/README.md
grep -c '^```' lessons/25-observability/README.md
```

Expected:
- First line: `# Lesson 25 — Observability`
- `grep -c '^### '` returns `9` (five subsections under self-study + four under exercises)
- `grep -c '^```'` returns `18` (9 code blocks × 2 fence lines — the "Metrics" self-study subsection carries two, and the "Compile-fail" exercise subsection is prose only)

If either count is wrong, the file content is off — re-check it against the content above and fix before committing.

- [ ] **Step 3: Commit**

```bash
git add lessons/25-observability/README.md
git commit -m "docs(lesson-25): write self-study notes"
```

---

## Task 6: Slide deck

**Files:**
- Overwrite: `lessons/25-observability/slides/slides.md`

**Interfaces:**
- Consumes: the names from Tasks 3-4.
- Produces: nothing code-facing.

- [ ] **Step 1: Overwrite `lessons/25-observability/slides/slides.md`**

The complete file content is below, delimited by an OUTER quadruple-backtick fence (` ```` `). That outer fence is ONLY a delimiter for this plan — do NOT write it into the file. The file must start with `# Observability` on line 1 and contain only PLAIN triple-backtick (` ``` `) code fences.

````markdown
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
tracing_subscriber::fmt().with_max_level(Level::DEBUG).init();  // lower the bar
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
````

- [ ] **Step 2: Verify `make slides-build` succeeds and includes lesson 25**

```bash
make slides-build
test -f dist/lessons/25-observability/slides/slides.md
test -f dist/lessons/25-observability/slides/index.html
grep -c "25-observability" dist/index.html
```

Expected: both files copied into dist; `grep -c "25-observability"` returns at least 1. (The build-index registry already has lesson 25 registered with slug `observability`, matching this directory, so it renders as a clickable link.)

- [ ] **Step 3: Spot-check slide separators**

```bash
grep -c '^---$' lessons/25-observability/slides/slides.md
```

Expected: `9` (between 10 slides).

- [ ] **Step 4: Commit**

```bash
git add lessons/25-observability/slides/slides.md
git commit -m "feat(lesson-25): write slide deck"
```

---

## Task 7: End-to-end verification + push

**Interfaces:**
- Consumes: everything from Tasks 1-6.
- Produces: lesson 25 live on the deployed site.

- [ ] **Step 1: `make ci` is green**

```bash
make ci
```

Expected: exit 0. Clippy clean, fmt clean, workspace builds, default-members tests pass (now including the 8 tests in `observability-solutions`), compile-fail `--expect broken` passes for lesson 25.

- [ ] **Step 2: `make verify LESSON=25-observability` fails (the exercise is undone — intentional)**

```bash
make verify LESSON=25-observability || echo "expected: exercise tests fail with todo!() panic"
```

Expected: `make` reports an error from the `cargo test` recipe — `test result: FAILED. 0 passed; 8 failed` (seven `not yet implemented` panics, plus `main_counters_are_shared_across_threads` failing on `join().unwrap()` after its worker threads panicked) — then the `expected: ...` echo line prints, so the combined command itself exits 0.

- [ ] **Step 3: `make slides-build` final state**

```bash
make slides-build
ls dist/lessons/
grep -c "25-observability" dist/index.html
```

Expected: `dist/lessons/` contains all twenty-five lessons. `grep -c "25-observability"` ≥ 1.

- [ ] **Step 4: Push**

```bash
git push
```

Expected: push succeeds. The CI cache key hashes `**/Cargo.toml`, which changed, so the first run refetches on both the stable and beta legs.

- [ ] **Step 5: Smoke-test the deployed site**

After the push, list the runs for the pushed commit. If fewer than two rows (`CI` and `Deploy`) appear, wait ~10 seconds and re-run it:

```bash
gh run list --commit "$(git rev-parse HEAD)" --json databaseId,workflowName,status
```

Wait for each run using the `databaseId` values from that output:

```bash
gh run watch <CI databaseId> --exit-status
gh run watch <Deploy databaseId> --exit-status
gh run list --commit "$(git rev-parse HEAD)" --json workflowName,conclusion
```

Expected: both `gh run watch` commands exit 0, and both `CI` and `Deploy` show `"conclusion":"success"`. Then:

```bash
curl -sS -o /dev/null -w "%{http_code}\n" https://rust.ristkari.dev/
curl -sS -o /dev/null -w "%{http_code}\n" https://rust.ristkari.dev/lessons/25-observability/slides/
```

Expected: both return `200`.

---

## Done criteria

- `lessons/25-observability/` exists with all four parts, plus `src/testkit.rs` in both crates
- Root `Cargo.toml` `[workspace.dependencies]` includes `serde_json`, `tracing` and `tracing-subscriber` (with the `json` feature); both lesson `Cargo.toml`s declare all three as workspace dependencies
- Cargo manifests use the package names `observability-exercises` and `observability-solutions`
- Both crates' `src/testkit.rs` are byte-identical
- `exercises/src/lib.rs` and `solutions/src/lib.rs` define the same `record_request`, `Metrics`, `Metrics::record` and `Metrics::render_prometheus`; the exercise ships `todo!()` bodies for the first two, the solution ships real ones
- `cargo test --package observability-solutions` → 8 passing tests
- `cargo test --manifest-path lessons/25-observability/exercises/Cargo.toml` → compiles, 8 panicking tests (intentional)
- `cargo run --package compile-fails -- --expect broken lessons/25-observability` → ok
- `cargo run --package compile-fails -- --expect compiles lessons/25-observability` → fails (intentional)
- `make ci` → green
- `make slides-build` → produces `dist/lessons/25-observability/slides/index.html`
- `dist/index.html` lists lesson 25 as a clickable link
- All changes committed and pushed (plain commit messages, no co-author trailer)
- The push triggers a green CI run (stable + beta) and a green Deploy run
- Deployed site returns HTTP 200 for `/` and `/lessons/25-observability/slides/`
