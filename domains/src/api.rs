// PUBLIC API FROZEN — changes require version bump
use core::{Domain, Request};
use whitehat::Decision;

pub trait DomainHandler {
    fn handle(&self, request: &dyn Request) -> Decision;
}

pub fn dispatch(
    handler: &dyn DomainHandler,
    request: &dyn Request,
) -> Decision {
    handler.handle(request)
}
