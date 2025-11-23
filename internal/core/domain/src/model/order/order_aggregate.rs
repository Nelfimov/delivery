use std::fmt::Display;
use uuid::Uuid;

use crate::errors::domain_model_errors::DomainModelError;
use crate::model::courier::courier_aggregate::CourierId;
use crate::model::kernel::location::Location;
use crate::model::kernel::volume::Volume;
use crate::model::order::order_events::OrderEvent;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum OrderStatus {
    Created,
    Assigned,
    Completed,
}

impl From<OrderStatus> for String {
    fn from(value: OrderStatus) -> Self {
        match value {
            OrderStatus::Created => "created".into(),
            OrderStatus::Assigned => "assigned".into(),
            OrderStatus::Completed => "completed".into(),
        }
    }
}

impl From<&OrderStatus> for String {
    fn from(value: &OrderStatus) -> Self {
        match value {
            OrderStatus::Created => "created".into(),
            OrderStatus::Assigned => "assigned".into(),
            OrderStatus::Completed => "completed".into(),
        }
    }
}

impl Display for OrderStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let v: String = self.into();
        write!(f, "{}", v)
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct OrderId(pub Uuid);

impl OrderId {
    pub fn new(id: Uuid) -> Self {
        Self(id)
    }

    pub fn value(&self) -> Uuid {
        self.0
    }
}

pub struct Order {
    id: OrderId,
    courier_id: Option<CourierId>,
    location: Location,
    volume: Volume,
    status: OrderStatus,

    domain_events: Vec<OrderEvent>,
}

impl PartialEq for Order {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for Order {}

impl Order {
    pub fn new(id: OrderId, location: Location, volume: Volume) -> Result<Self, DomainModelError> {
        let mut order = Self {
            id,
            location,
            volume,
            status: OrderStatus::Created,
            courier_id: None,
            domain_events: Vec::new(),
        };
        order.raise_domain_event(OrderEvent::created(id));
        Ok(order)
    }

    pub fn restore(
        id: OrderId,
        courier_id: Option<CourierId>,
        location: Location,
        volume: Volume,
        status: OrderStatus,
    ) -> Self {
        Self {
            id,
            location,
            volume,
            status,
            courier_id,
            domain_events: Vec::new(),
        }
    }

    pub fn assign(&mut self, courier_id: &CourierId) -> Result<(), DomainModelError> {
        if self.courier_id.is_some() {
            return Err(DomainModelError::ArgumentAlreadyExists(
                "courier_id".to_string(),
            ));
        }
        match self.status {
            OrderStatus::Assigned | OrderStatus::Completed => Err(
                DomainModelError::UnmetRequirement(format!("status is already {}", self.status)),
            ),
            OrderStatus::Created => {
                self.courier_id = Some(*courier_id);
                self.status = OrderStatus::Assigned;
                Ok(())
            }
        }
    }

    pub fn complete(&mut self) -> Result<(), DomainModelError> {
        match self.courier_id {
            None => Err(DomainModelError::UnmetRequirement(
                "courier_id is not present".to_owned(),
            )),
            Some(courier_id) => match self.status {
                OrderStatus::Completed => Err(DomainModelError::UnmetRequirement(
                    "status is already set as Completed".to_string(),
                )),
                _ => {
                    self.status = OrderStatus::Completed;
                    self.raise_domain_event(OrderEvent::completed(self.id, courier_id));
                    Ok(())
                }
            },
        }
    }

    pub fn id(&self) -> OrderId {
        self.id
    }

    pub fn courier_id(&self) -> &Option<CourierId> {
        &self.courier_id
    }

    pub fn location(&self) -> &Location {
        &self.location
    }

    pub fn volume(&self) -> u16 {
        self.volume.value()
    }

    pub fn status(&self) -> &OrderStatus {
        &self.status
    }

    pub fn raise_domain_event(&mut self, event: OrderEvent) {
        self.domain_events.push(event);
    }

    pub fn get_domain_events(&self) -> &Vec<OrderEvent> {
        &self.domain_events
    }

    pub fn pop_domain_events(self) -> Vec<OrderEvent> {
        self.domain_events
    }

    pub fn clear_domain_events(&mut self) {
        self.domain_events.clear();
    }
}
