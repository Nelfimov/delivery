use serde::Deserialize;
use serde::Serialize;

use crate::model::courier::courier_aggregate::CourierId;
use crate::model::kernel::event::DomainEvent;
use crate::model::kernel::event::EventId;
use crate::model::order::order_aggregate::OrderId;

#[derive(Clone, Serialize, Deserialize, Debug)]
pub enum OrderEvent {
    Created(OrderCreatedEvent),
    Completed(OrderCompletedEvent),
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct OrderCreatedEvent {
    pub id: EventId,
    pub name: String,
    pub order_id: OrderId,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct OrderCompletedEvent {
    pub id: EventId,
    pub name: String,
    pub order_id: OrderId,
    pub courier_id: CourierId,
}

impl DomainEvent for OrderEvent {
    fn id(&self) -> String {
        match self {
            Self::Created(e) => e.id.0.to_string(),
            Self::Completed(e) => e.id.0.to_string(),
        }
    }

    fn name(&self) -> String {
        match self {
            Self::Created(e) => e.name.clone(),
            Self::Completed(e) => e.name.clone(),
        }
    }
}

impl OrderEvent {
    pub fn created(order_id: OrderId) -> Self {
        Self::Created(OrderCreatedEvent {
            id: EventId::default(),
            name: "created".to_string(),
            order_id,
        })
    }

    pub fn completed(order_id: OrderId, courier_id: CourierId) -> Self {
        Self::Completed(OrderCompletedEvent {
            id: EventId::default(),
            name: "completed".to_string(),
            order_id,
            courier_id,
        })
    }
}
