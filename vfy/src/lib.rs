//! VFY v0 — Verification SSPU
//!
//! Purpose:
//! - Prove claims about observed events.
//! - No execution authority.
//! - No policy.
//! - No interpretation beyond explicit checks.
//!
//! Inputs: OBS events (facts)
//! Outputs: verification results (pass/fail + issues)

use obs::ObsEvent;

#[derive(Clone, Debug)]
pub struct VfyIssue {
    pub code: &'static str,
    pub detail: String,
}

#[derive(Clone, Debug)]
pub struct VfyResult {
    pub ok: bool,
    pub issues: Vec<VfyIssue>,
}

impl VfyResult {
    pub fn ok() -> Self {
        Self { ok: true, issues: Vec::new() }
    }

    pub fn fail(mut self, code: &'static str, detail: impl Into<String>) -> Self {
        self.ok = false;
        self.issues.push(VfyIssue { code, detail: detail.into() });
        self
    }
}

/// VFY v0 verifier: deterministic, rule-only checks.
pub struct Verifier;

impl Verifier {
    /// Verify a snapshot of OBS events.
    ///
    /// Checks (v0, minimal):
    /// - timestamps are non-decreasing
    /// - messages are non-empty
    /// - sources are non-empty
    pub fn verify(events: &[ObsEvent]) -> VfyResult {
        let mut res = VfyResult::ok();

        let mut last_ts: Option<u128> = None;

        for (i, e) in events.iter().enumerate() {
            if e.source.is_empty() {
                res = res.fail(
                    "VFY_EMPTY_SOURCE",
                    format!("event[{i}] has empty source"),
                );
            }

            if e.message.trim().is_empty() {
                res = res.fail(
                    "VFY_EMPTY_MESSAGE",
                    format!("event[{i}] has empty/blank message"),
                );
            }

            if let Some(prev) = last_ts {
                if e.timestamp_ms < prev {
                    res = res.fail(
                        "VFY_TIME_ORDER",
                        format!(
                            "event[{i}] timestamp decreased: {} < {}",
                            e.timestamp_ms, prev
                        ),
                    );
                }
            }
            last_ts = Some(e.timestamp_ms);
        }

        res
    }
}
