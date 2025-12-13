use photoshy::PhotoDomain;
use core::{Domain, DomainId};

#[test]
fn photoshy_domain_has_stable_id() {
    let domain = PhotoDomain::new();

    let id = domain.id();
    assert_eq!(id, DomainId("photoshy".into()));
}
