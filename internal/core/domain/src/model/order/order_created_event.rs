use uuid::Uuid;

use crate::model::kernel::event::DomainEvent;

#[derive(Clone, Debug)]
pub struct OrderCreatedEvent {
    id: String,
    name: String,
    order_id: String,
}

impl OrderCreatedEvent {
    pub fn new(order_id: Uuid) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            name: "order_created_event".to_string(),
            order_id: order_id.to_string(),
        }
    }

    pub fn order_id(&self) -> String {
        self.order_id.clone()
    }
}

impl DomainEvent for OrderCreatedEvent {
    fn id(&self) -> String {
        self.id.clone()
    }

    fn name(&self) -> String {
        self.name.clone()
    }
}
