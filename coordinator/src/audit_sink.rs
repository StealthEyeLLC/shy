// PASS 1 — structural only

use crate::AuditEvent;
use core::{DomainId, RequestId};

pub trait AuditSink {
    fn emit(
        &self,
        domain: &DomainId,
        request: &RequestId,
        event: AuditEvent,
    );
}
