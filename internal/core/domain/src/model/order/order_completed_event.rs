use uuid::Uuid;

use crate::model::kernel::event::DomainEvent;

#[derive(Clone, Debug)]
pub struct OrderCompletedEvent {
    id: String,
    name: String,
    order_id: String,
    courier_id: String,
}

impl OrderCompletedEvent {
    pub fn new(order_id: Uuid, courier_id: Uuid) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            name: "order_completed_event".to_string(),
            order_id: order_id.to_string(),
            courier_id: courier_id.to_string(),
        }
    }

    pub fn order_id(&self) -> String {
        self.order_id.clone()
    }

    pub fn courier_id(&self) -> String {
        self.courier_id.clone()
    }
}

impl DomainEvent for OrderCompletedEvent {
    fn id(&self) -> String {
        self.id.clone()
    }

    fn name(&self) -> String {
        self.name.clone()
    }
}
