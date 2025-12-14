//! OBS Sink (Bridge) — v0
//!
//! Purpose:
//! - Provide a one-way, record-only event sink that SHY (or any executor) can call
//!   without OBS gaining any authority.
//!
//! Constraints:
//! - No interpretation.
//! - No branching logic.
//! - No feedback path into execution.
//! - Append-only recording via OBS v0.

use obs::ObsLog;

/// Minimal one-way sink interface for emitting execution events.
/// This is intentionally tiny to prevent authority creep.
pub trait ObsSink: Send + Sync {
    fn emit(&self, source: &'static str, message: &str);
}

/// OBS v0 sink implementation backed by an in-memory append-only ObsLog.
pub struct ObsSinkV0 {
    log: ObsLog,
}

impl ObsSinkV0 {
    /// Create a new OBS v0 sink.
    pub fn new() -> Self {
        Self { log: ObsLog::new() }
    }

    /// Access the underlying log (read-only via snapshot methods on ObsLog).
    /// NOTE: This does not grant authority—only visibility for tests/verification.
    pub fn log(&self) -> &ObsLog {
        &self.log
    }
}

impl Default for ObsSinkV0 {
    fn default() -> Self {
        Self::new()
    }
}

impl ObsSink for ObsSinkV0 {
    fn emit(&self, source: &'static str, message: &str) {
        self.log.record(source, message.to_string());
    }
}
