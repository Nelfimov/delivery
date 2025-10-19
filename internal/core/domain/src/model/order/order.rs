use uuid::Uuid;

use crate::errors::domain_model_errors::DomainModelError;
use crate::model::kernel::location::Location;
use crate::model::kernel::volume::Volume;

pub enum OrderStatus {
    Created,
    Assigned,
    Completed,
}

#[derive(PartialEq, Eq)]
pub struct OrderId(pub Uuid);

pub struct CourierId(pub Uuid);

pub struct Order {
    id: OrderId,
    courier_id: Option<CourierId>,
    location: Location,
    volume: Volume,
    status: OrderStatus,
}

impl PartialEq for Order {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for Order {}

impl Order {
    fn new(id: OrderId, location: Location, volume: u16) -> Self {
        Self {
            id,
            location,
            volume: Volume::new(volume).unwrap(),
            status: OrderStatus::Created,
            courier_id: None,
        }
    }

    fn assign(&mut self, courier_id: CourierId) -> Result<(), DomainModelError> {
        if self.courier_id.is_some() {
            return Err(DomainModelError::ArgumentAlreadyExists(
                "courier_id".to_string(),
            ));
        }
        self.courier_id = Some(courier_id);
        Ok(())
    }

    fn complete(&mut self) -> Result<(), DomainModelError> {
        match self.status {
            OrderStatus::Completed => Err(DomainModelError::ArgumentAlreadyExists(
                "status".to_string(),
            )),
            _ => {
                self.status = OrderStatus::Completed;
                Ok(())
            }
        }
    }

    fn id(&self) -> Uuid {
        self.id.0
    }

    fn courier_id(&self) -> &Option<CourierId> {
        &self.courier_id
    }

    fn location(&self) -> &Location {
        &self.location
    }

    fn volume(&self) -> u16 {
        self.volume.0
    }

    fn status(&self) -> &OrderStatus {
        &self.status
    }
}
