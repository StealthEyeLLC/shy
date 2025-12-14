//! OBS v0 — Observation SSPU
//! Record-only, append-only execution event sink.
//! No interpretation. No authority. No feedback.

use std::sync::Mutex;
use std::time::{SystemTime, UNIX_EPOCH};

/// Immutable execution event as observed.
#[derive(Clone, Debug)]
pub struct ObsEvent {
    pub timestamp_ms: u128,
    pub source: &'static str,
    pub message: String,
}

/// Append-only observation log (in-memory).
pub struct ObsLog {
    events: Mutex<Vec<ObsEvent>>,
}

impl ObsLog {
    /// Create a new empty observation log.
    pub fn new() -> Self {
        Self {
            events: Mutex::new(Vec::new()),
        }
    }

    /// Record an execution event.
    /// This MUST NOT fail silently.
    pub fn record(&self, source: &'static str, message: impl Into<String>) {
        let event = ObsEvent {
            timestamp_ms: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("time went backwards")
                .as_millis(),
            source,
            message: message.into(),
        };

        let mut guard = self.events.lock().expect("OBS mutex poisoned");
        guard.push(event);
    }

    /// Read-only snapshot of all observed events.
    pub fn snapshot(&self) -> Vec<ObsEvent> {
        let guard = self.events.lock().expect("OBS mutex poisoned");
        guard.clone()
    }
}
