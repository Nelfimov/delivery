use domain::model::order::order_completed_event::OrderCompletedEvent;
use domain::model::order::order_created_event::OrderCreatedEvent;
use prost_types::Timestamp;
use std::time::SystemTime;

use crate::order_event_gen::OrderCompletedIntegrationEvent;
use crate::order_event_gen::OrderCreatedIntegrationEvent;

impl From<OrderCompletedEvent> for OrderCompletedIntegrationEvent {
    fn from(v: OrderCompletedEvent) -> Self {
        Self {
            order_id: v.order_id(),
            event_id: v.id(),
            event_type: "completed".to_string(),
            occurred_at: Some(Timestamp::from(SystemTime::now())),
            courier_id: v.courier_id(),
        }
    }
}

impl From<OrderCreatedEvent> for OrderCreatedIntegrationEvent {
    fn from(v: OrderCreatedEvent) -> Self {
        Self {
            event_id: v.id(),
            event_type: "created".to_string(),
            occurred_at: Some(Timestamp::from(SystemTime::now())),
            order_id: v.order_id(),
        }
    }
}
