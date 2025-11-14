use uuid::Uuid;

pub struct OrderCreatedEvent {
    id: Uuid,
    order_id: Uuid,
}

impl OrderCreatedEvent {
    pub fn new(order_id: Uuid) -> Self {
        Self {
            id: Uuid::new_v4(),
            order_id,
        }
    }

    pub fn id(&self) -> Uuid {
        self.id
    }

    pub fn order_id(&self) -> Uuid {
        self.order_id
    }
}
