use uuid::Uuid;

use crate::errors::domain_model_errors::DomainModelCreationError;

pub struct StoragePlace {
    id: Uuid,
    name: String,
    total_volume: u16,
    order_id: Option<Uuid>,
}

impl StoragePlace {
    pub fn new(
        name: String,
        total_volume: u16,
        order_id: Option<Uuid>,
    ) -> Result<Self, DomainModelCreationError> {
        if total_volume == 0 {
            return Err(DomainModelCreationError::ArgumentCannotBeZero(
                "total_volume".to_string(),
            ));
        }

        if name.is_empty() {
            return Err(DomainModelCreationError::ArgumentCannotBeEmpty(
                "name".to_string(),
            ));
        }

        Ok(Self {
            id: Uuid::new_v4(),
            name,
            total_volume,
            order_id,
        })
    }

    pub fn id(&self) -> &Uuid {
        &self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn total_volume(&self) -> &u16 {
        &self.total_volume
    }

    pub fn order_id(&self) -> &Option<Uuid> {
        &self.order_id
    }

    pub fn equals(&self, other: &StoragePlace) -> bool {
        self.id == other.id
    }

    pub fn can_place_order(&self, order_id: Uuid, volume: u16) -> bool {
        if self.order_id.is_some() {
            return false;
        }

        if order_id.is_nil() {
            return false;
        }

        if volume > self.total_volume {
            return false;
        }

        true
    }

    pub fn place_order(&mut self, order_id: Uuid, volume: u16) {
        if self.can_place_order(order_id, volume) {
            self.order_id = Some(order_id);
        }
    }

    pub fn remove_order(&mut self) -> bool {
        if self.order_id.is_none() {
            return false;
        }

        self.order_id = None;
        true
    }
}
