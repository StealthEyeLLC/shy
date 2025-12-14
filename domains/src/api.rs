use core::Request;

/// PUBLIC API FROZEN — changes require version bump
pub trait Domain {
    fn id(&self) -> String;
}

/// PUBLIC API FROZEN — changes require version bump
pub trait DomainRequest: Request {
    fn domain(&self) -> &dyn Domain;
}
