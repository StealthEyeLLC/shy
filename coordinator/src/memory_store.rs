use crate::{FlowState, FlowStore};
use core::{DomainId, RequestId};
use std::cell::RefCell;

/// PUBLIC API FROZEN — changes require version bump
pub struct InMemoryFlowStore {
    records: RefCell<Vec<(DomainId, RequestId, FlowState)>>,
}

impl InMemoryFlowStore {
    pub fn new() -> Self {
        InMemoryFlowStore {
            records: RefCell::new(Vec::new()),
        }
    }

    pub fn records_len(&self) -> usize {
        self.records.borrow().len()
    }
}

impl FlowStore for InMemoryFlowStore {
    fn record_flow(
        &self,
        domain: &DomainId,
        request: &RequestId,
        state: FlowState,
    ) {
        self.records
            .borrow_mut()
            .push((domain.clone(), request.clone(), state));
    }
}
