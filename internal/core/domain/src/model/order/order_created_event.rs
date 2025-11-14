use uuid::Uuid;

pub struct OrderCreatedEvent {
    id: String,
    order_id: String,
}

impl OrderCreatedEvent {
    pub fn new(order_id: Uuid) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            order_id: order_id.to_string(),
        }
    }

    pub fn id(&self) -> String {
        self.id.clone()
    }

    pub fn order_id(&self) -> String {
        self.order_id.clone()
    }
}
