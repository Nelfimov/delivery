use uuid::Uuid;

use crate::model::courier::storage_place::StoragePlace;
use crate::model::kernel::location::Location;

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub struct CourierId(pub Uuid);

pub struct CourierName(pub String);

pub struct CourierSpeed(pub u16);

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
    pub fn new(name: CourierName, speed: CourierSpeed, location: Location) -> Self {
        let default_storage_place = StoragePlace::new("bag".to_string(), 10, None);

        let storage_places: Vec<StoragePlace> = vec![default_storage_place.unwrap()];

        Self {
            id: CourierId(Uuid::new_v4()),
            location,
            name,
            speed,
            storage_places,
        }
    }
}
