use core::{Request, RequestId};
use echo_domain::EchoDomain;
use whitehat::Decision;

struct TestRequest;

impl Request for TestRequest {
    fn id(&self) -> RequestId {
        RequestId("echo-test".into())
    }
}

#[test]
fn echo_domain_integrates_cleanly() {
    let domain = EchoDomain::new();
    let request = TestRequest;

    let decision = domain.handle(&request);

    assert_eq!(decision, Decision::Allow);
}
