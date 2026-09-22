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
    assert!(
        !one.message.is_empty(),
        "the event needs a message - a constant one, with the values in fields"
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
