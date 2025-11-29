use serde::Deserialize;
use serde::Serialize;

use crate::model::courier::courier_aggregate::CourierId;
use crate::model::kernel::event::DomainEvent;
use crate::model::kernel::event::EventId;
use crate::model::order::order_aggregate::OrderId;

#[derive(Clone, Serialize, Deserialize)]
pub enum OrderEvent {
    Created {
        id: EventId,
        name: String,
        order_id: OrderId,
    },
    Completed {
        id: EventId,
        name: String,
        order_id: OrderId,
        courier_id: CourierId,
    },
}

impl DomainEvent for OrderEvent {
    fn id(&self) -> String {
        match self {
            Self::Created { id, .. } => id.0.to_string(),
            Self::Completed { id, .. } => id.0.to_string(),
        }
    }

    fn name(&self) -> String {
        match self {
            Self::Created { name, .. } => name.clone(),
            Self::Completed { name, .. } => name.clone(),
        }
    }
}

impl OrderEvent {
    pub fn created(order_id: OrderId) -> Self {
        Self::Created {
            id: EventId::default(),
            name: "created".to_string(),
            order_id,
        }
    }

    pub fn completed(order_id: OrderId, courier_id: CourierId) -> Self {
        Self::Completed {
            id: EventId::default(),
            name: "completed".to_string(),
            order_id,
            courier_id,
        }
    }
}
