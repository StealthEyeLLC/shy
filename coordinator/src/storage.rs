use crate::FlowState;
use core::{DomainId, RequestId};

/// PUBLIC API FROZEN — changes require version bump
pub trait FlowStore {
    fn record_flow(
        &self,
        domain: &DomainId,
        request: &RequestId,
        state: FlowState,
    );
}
