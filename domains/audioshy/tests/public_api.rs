use audioshy::AudioDomain;
use core::Domain;

#[test]
fn audioshy_domain_has_stable_id() {
    let domain = AudioDomain::new();
    assert_eq!(domain.id().0, "audioshy");
}
