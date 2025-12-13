use domains::{DomainHandler};
use core::{Request, RequestId};
use whitehat::Decision;

struct TestRequest;
impl Request for TestRequest {
    fn id(&self) -> RequestId {
        RequestId("req".into())
    }
}

struct TestHandler;
impl DomainHandler for TestHandler {
    fn handle(&self, _request: &dyn Request) -> Decision {
        Decision::Allow
    }
}

#[test]
fn domain_handler_is_callable() {
    let h = TestHandler;
    let r = TestRequest;
    let _ = h.handle(&r);
}
