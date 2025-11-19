pub trait DomainEvent {
    fn id(&self) -> String;
    fn name(&self) -> String;
}
