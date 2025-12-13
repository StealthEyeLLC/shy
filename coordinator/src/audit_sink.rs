use crate::AuditEvent;
use core::{DomainId, RequestId};

/// PUBLIC API FROZEN — changes require version bump
pub trait AuditSink {
    fn emit(
        &self,
        domain: &DomainId,
        request: &RequestId,
        event: AuditEvent,
    );
}
