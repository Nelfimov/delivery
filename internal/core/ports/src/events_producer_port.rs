use domain::model::order::order_events::OrderEvent;

pub enum Events {
    Order(OrderEvent)
}

impl From<OrderEvent> for Events {
    fn from(v: OrderEvent) -> Self {
        Events::Order(v)
    }
}

pub trait EventsProducerPort {
    fn publish(&self, e: Events);
}
