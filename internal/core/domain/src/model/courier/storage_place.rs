use uuid::Uuid;

use crate::errors::domain_model_errors::DomainModelError;
use crate::model::kernel::volume::Volume;
use crate::model::order::order_aggregate::OrderId;

#[derive(Clone, Debug)]
pub struct StoragePlace {
    id: Uuid,
    name: String,
    total_volume: Volume,
    order_id: Option<OrderId>,
}

impl PartialEq for StoragePlace {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for StoragePlace {}

impl StoragePlace {
    pub fn new(
        name: String,
        total_volume: Volume,
        order_id: Option<OrderId>,
    ) -> Result<Self, DomainModelError> {
        if name.is_empty() {
            return Err(DomainModelError::ArgumentCannotBeEmpty("name".to_string()));
        }

        Ok(Self {
            id: Uuid::new_v4(),
            name,
            total_volume,
            order_id,
        })
    }

    pub fn restore(
        id: Uuid,
        name: String,
        total_volume: Volume,
        order_id: Option<OrderId>,
    ) -> Self {
        Self {
            id,
            name,
            total_volume,
            order_id,
        }
    }

    pub fn id(&self) -> &Uuid {
        &self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn total_volume(&self) -> u16 {
        self.total_volume.value()
    }

    pub fn order_id(&self) -> &Option<OrderId> {
        &self.order_id
    }

    pub fn can_place_order(&self, volume: &Volume) -> bool {
        if volume.value() > self.total_volume.value() {
            return false;
        }

        if self.order_id.is_some() {
            return false;
        }

        true
    }

    pub fn place_order(&mut self, order_id: OrderId, volume: Volume) -> bool {
        if self.can_place_order(&volume) {
            self.order_id = Some(order_id);
            return true;
        }

        false
    }

    pub fn remove_order(&mut self) -> bool {
        if self.order_id.is_none() {
            return false;
        }

        self.order_id = None;
        true
    }
}
