use core::DomainId;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DomainMeta {
    pub id: DomainId,
    pub name: String,
}

pub trait DomainRegistry {
    fn list(&self) -> Vec<DomainMeta>;
}
