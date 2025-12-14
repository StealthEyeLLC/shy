//! SHY → OBS Emission Adapter (v0)
//!
//! Purpose:
//! - Provide a zero-authority helper that SHY can call to emit execution events
//!   without OBS influencing execution.
//!
//! Guarantees:
//! - One-way only
//! - Record-only
//! - No branching, no policy, no interpretation
//! - Safe to call anywhere inside SHY execution paths

use crate::obs_sink::ObsSink;

/// Lightweight helper for SHY-side emission.
/// This does NOT wrap SHY and does NOT control execution.
pub struct ShyEmitter<'a> {
    sink: &'a dyn ObsSink,
}

impl<'a> ShyEmitter<'a> {
    /// Bind an emitter to an OBS sink.
    pub fn new(sink: &'a dyn ObsSink) -> Self {
        Self { sink }
    }

    /// Emit a deterministic execution event.
    #[inline]
    pub fn emit(&self, message: &'static str) {
        self.sink.emit("SHY", message);
    }
}
