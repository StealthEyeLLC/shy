use coordinator::FlowState;
use whitehat::RefusalReason;

/// PUBLIC API FROZEN — changes require version bump
pub struct CliReport;

impl CliReport {
    pub fn new() -> Self {
        CliReport
    }

    pub fn map_flow_state(&self, state: FlowState) -> Result<(), RefusalReason> {
        match state {
            FlowState::Completed => Ok(()),
            FlowState::Rejected => Err(RefusalReason::PolicyDenied),
            FlowState::Initialized => {
                panic!("CLI attempted to report an uninitialized flow state")
            }
        }
    }
}
