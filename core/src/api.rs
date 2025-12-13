// PUBLIC API FROZEN — changes require version bump
use crate::{CoreError, DomainId, RequestId};

pub trait Domain {
    fn id(&self) -> DomainId;
}

pub trait Request {
    fn id(&self) -> RequestId;
}

pub fn validate_domain(domain: &dyn Domain) -> Result<(), CoreError> {
    let id = domain.id();

    if id.0.is_empty() {
        Err(CoreError::InvalidInput)
    } else {
        Ok(())
    }
}
