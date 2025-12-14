use crate::{Domain, DomainId};

/// PUBLIC API FROZEN — changes require version bump
///
/// Central registry for all domains known to the system.
/// This enforces explicit domain registration and prevents
/// implicit or ad-hoc domain discovery.
pub struct DomainRegistry {
    domains: Vec<Box<dyn Domain>>,
}

impl DomainRegistry {
    /// Create an empty domain registry.
    pub fn new() -> Self {
        Self { domains: Vec::new() }
    }

    /// Register a domain.
    ///
    /// Rules:
    /// - Domain IDs must be unique
    /// - Registration order is deterministic
    pub fn register(&mut self, domain: Box<dyn Domain>) -> Result<(), RegistryError> {
        let id = domain.id();

        if self.domains.iter().any(|d| d.id() == id) {
            return Err(RegistryError::DuplicateDomain(id));
        }

        self.domains.push(domain);
        Ok(())
    }

    /// Return all registered domains.
    pub fn domains(&self) -> &[Box<dyn Domain>] {
        &self.domains
    }

    /// Check whether a domain with the given ID exists.
    pub fn contains(&self, id: &DomainId) -> bool {
        self.domains.iter().any(|d| d.id() == *id)
    }
}

/// Errors related to domain registration.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RegistryError {
    DuplicateDomain(DomainId),
}
