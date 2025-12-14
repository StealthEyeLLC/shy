use core::{Domain, DomainId};

struct TestDomain;

impl Domain for TestDomain {
    fn id(&self) -> DomainId {
        DomainId("test".into())
    }
}

#[test]
fn domain_has_id() {
    let d = TestDomain;
    let _ = d.id();
}
