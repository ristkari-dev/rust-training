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
