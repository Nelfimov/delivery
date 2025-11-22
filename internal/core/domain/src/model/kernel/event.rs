use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EventId(pub Uuid);

impl Default for EventId {
    fn default() -> Self {
        Self(Uuid::new_v4())
    }
}

pub trait DomainEvent {
    fn id(&self) -> String;
    fn name(&self) -> String;
}
