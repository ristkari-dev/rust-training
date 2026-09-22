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
