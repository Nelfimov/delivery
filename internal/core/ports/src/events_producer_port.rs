use domain::model::order::order_completed_event::OrderCompletedEvent;
use domain::model::order::order_created_event::OrderCreatedEvent;

pub enum Events {
    OrderCreated(OrderCreatedEvent),
    OrderCompleted(OrderCompletedEvent),
}

pub trait EventsProducerPort {
    fn publish(&self, e: Events);
}
