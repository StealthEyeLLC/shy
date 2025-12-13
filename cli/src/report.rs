use coordinator::FlowState;
use whitehat::RefusalReason;

/// PUBLIC API FROZEN — changes require version bump
pub struct CliReport;

impl CliReport {
    pub fn new() -> Self {
        CliReport
    }

    /// Maps internal flow state into a CLI-visible result.
    ///
    /// Invariants:
    /// - CLI must never panic
    /// - All FlowState variants are handled explicitly
    /// - Initialized is surfaced as a refusal, not a crash
    pub fn map_flow_state(&self, state: FlowState) -> Result<(), RefusalReason> {
        match state {
            FlowState::Completed => Ok(()),

            FlowState::Rejected => Err(RefusalReason::PolicyDenied),

            FlowState::Initialized => Err(RefusalReason::PolicyDenied),
        }
    }
}
