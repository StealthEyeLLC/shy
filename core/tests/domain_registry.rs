use core::{Domain, DomainId, DomainRegistry, RegistryError};

struct TestDomain {
    id: DomainId,
}

impl TestDomain {
    fn new(id: &str) -> Self {
        Self {
            id: DomainId(id.to_string()),
        }
    }
}

impl Domain for TestDomain {
    fn id(&self) -> DomainId {
        self.id.clone()
    }
}

#[test]
fn registry_accepts_unique_domains() {
    let mut registry = DomainRegistry::new();

    let domain_a = Box::new(TestDomain::new("alpha"));
    let domain_b = Box::new(TestDomain::new("beta"));

    assert!(registry.register(domain_a).is_ok());
    assert!(registry.register(domain_b).is_ok());

    assert!(registry.contains(&DomainId("alpha".into())));
    assert!(registry.contains(&DomainId("beta".into())));
}

#[test]
fn registry_rejects_duplicate_domain_ids() {
    let mut registry = DomainRegistry::new();

    let domain_a1 = Box::new(TestDomain::new("alpha"));
    let domain_a2 = Box::new(TestDomain::new("alpha"));

    assert!(registry.register(domain_a1).is_ok());

    let err = registry.register(domain_a2).unwrap_err();
    assert_eq!(err, RegistryError::DuplicateDomain(DomainId("alpha".into())));
}
