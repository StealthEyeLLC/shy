use crate::{AuditEvent, AuditSink};
use core::{DomainId, RequestId};
use std::cell::RefCell;

/// PUBLIC API FROZEN — changes require version bump
pub struct InMemoryAuditSink {
    events: RefCell<Vec<(DomainId, RequestId, AuditEvent)>>,
}

impl InMemoryAuditSink {
    pub fn new() -> Self {
        InMemoryAuditSink {
            events: RefCell::new(Vec::new()),
        }
    }

    pub fn events_len(&self) -> usize {
        self.events.borrow().len()
    }
}

impl AuditSink for InMemoryAuditSink {
    fn emit(
        &self,
        domain: &DomainId,
        request: &RequestId,
        event: AuditEvent,
    ) {
        self.events
            .borrow_mut()
            .push((domain.clone(), request.clone(), event));
    }
}
