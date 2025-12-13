use coordinator::{FlowStore, FlowState};
use core::{DomainId, RequestId};

struct TestStore;

impl FlowStore for TestStore {
    fn record_flow(
        &self,
        _domain: &DomainId,
        _request: &RequestId,
        _state: FlowState,
    ) {}
}

#[test]
fn flow_store_trait_is_implementable() {
    let _store = TestStore;
}
