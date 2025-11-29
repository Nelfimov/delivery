use serde::Deserialize;
use serde::Serialize;
use uuid::Uuid;

use crate::errors::domain_model_errors::DomainModelError;
use crate::model::courier::storage_place::StoragePlace;
use crate::model::kernel::location::Location;
use crate::model::kernel::volume::Volume;
use crate::model::order::order_aggregate::OrderId;

#[derive(PartialEq, Eq, Debug, Clone, Copy, Serialize, Deserialize)]
pub struct CourierId(pub Uuid);

#[derive(Clone, Debug)]
pub struct CourierName(pub String);

#[derive(Clone, Debug)]
pub struct CourierSpeed(pub u8);

#[derive(Clone, Debug)]
pub struct Courier {
    id: CourierId,
    name: CourierName,
    speed: CourierSpeed,
    location: Location,
    storage_places: Vec<StoragePlace>,
}

impl PartialEq for Courier {
    fn eq(&self, other: &Courier) -> bool {
        self.id == other.id
    }
}

impl Courier {
    pub fn new(
        name: CourierName,
        speed: CourierSpeed,
        location: Location,
    ) -> Result<Self, DomainModelError> {
        let detault_volume = Volume::new(10)?;
        let default_storage_place = StoragePlace::new("bag".to_string(), detault_volume, None)?;
        let storage_places: Vec<StoragePlace> = vec![default_storage_place];

        Ok(Self {
            id: CourierId(Uuid::new_v4()),
            location,
            name,
            speed,
            storage_places,
        })
    }

    pub fn restore(
        id: CourierId,
        name: CourierName,
        speed: CourierSpeed,
        location: Location,
        storage_places: Vec<StoragePlace>,
    ) -> Self {
        Self {
            id,
            name,
            speed,
            location,
            storage_places,
        }
    }

    pub fn id(&self) -> &CourierId {
        &self.id
    }
    pub fn name(&self) -> &String {
        &self.name.0
    }
    pub fn speed(&self) -> &u8 {
        &self.speed.0
    }
    pub fn location(&self) -> &Location {
        &self.location
    }
    pub fn storage_places(&self) -> &Vec<StoragePlace> {
        &self.storage_places
    }

    pub fn add_storage_place(
        &mut self,
        name: String,
        volume: Volume,
    ) -> Result<(), DomainModelError> {
        let new_storage_place = StoragePlace::new(name, volume, None)?;
        self.storage_places.push(new_storage_place);
        Ok(())
    }

    pub fn can_take_order(&self, order_volume: &Volume) -> Option<usize> {
        self.storage_places
            .iter()
            .enumerate()
            .filter(|(_, sp)| sp.can_place_order(order_volume))
            .min_by_key(|(_, sp)| sp.total_volume())
            .map(|(index, _)| index)
    }

    pub fn take_order(
        &mut self,
        order_id: OrderId,
        order_volume: Volume,
    ) -> Result<(), DomainModelError> {
        if let Some(index) = self.can_take_order(&order_volume)
            && let Some(storage) = self.storage_places.get_mut(index)
        {
            storage.place_order(order_id, order_volume);
            return Ok(());
        }

        Err(DomainModelError::UnmetRequirement(format!(
            "could not assign order, {:?}",
            self
        )))
    }

    pub fn complete_order(&mut self, order_id: OrderId) {
        if let Some(storage) = self
            .storage_places
            .iter_mut()
            .find(|sp| sp.order_id() == &Some(order_id))
        {
            storage.remove_order();
        }
    }

    pub fn get_traverse_length(&self, destination: &Location) -> u8 {
        let distance = self.location.get_distance(destination) as f64;
        let speed = self.speed.0 as f64;

        (distance / speed).ceil() as u8
    }

    pub fn move_to_location(&mut self, location: &Location) -> Result<&Location, DomainModelError> {
        let speed = self.speed.0;
        let x_distance = self.location.x().abs_diff(location.x());
        let y_distance = self.location.y().abs_diff(location.y());

        if x_distance >= y_distance {
            if self.location.x() < location.x() {
                let preliminary_action = self.location.x() + speed;
                let x = if preliminary_action > location.x() {
                    location.x()
                } else {
                    preliminary_action
                };
                self.location = Location::new(x, self.location.y())?
            }
            if self.location.x() > location.x() {
                let preliminary_action = self.location.x() - speed;
                let x = if preliminary_action < location.x() {
                    location.x()
                } else {
                    preliminary_action
                };
                self.location = Location::new(x, self.location.y())?
            }
        } else {
            if self.location.y() < location.y() {
                let preliminary_action = self.location.y() + speed;
                let y = if preliminary_action > location.y() {
                    location.y()
                } else {
                    preliminary_action
                };
                self.location = Location::new(self.location.x(), y)?
            }
            if self.location.y() > location.y() {
                let preliminary_action = self.location.y() - speed;
                let y = if preliminary_action < location.y() {
                    location.y()
                } else {
                    preliminary_action
                };
                self.location = Location::new(self.location.x(), y)?
            }
        }

        Ok(&self.location)
    }
}
