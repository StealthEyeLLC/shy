// PASS 1 — structural only

use crate::FlowState;
use core::{DomainId, RequestId};

pub trait FlowStore {
    fn record_flow(
        &self,
        domain: &DomainId,
        request: &RequestId,
        state: FlowState,
    );
}
