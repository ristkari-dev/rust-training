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
