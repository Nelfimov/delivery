use domain::model::order::order_events::OrderEvent;

pub enum Events {
    Order(OrderEvent)
}

pub trait EventsProducerPort {
    fn publish(&self, e: Events);
}
