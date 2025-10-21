use uuid::Uuid;

use crate::model::courier::storage_place::StoragePlace;
use crate::model::kernel::location::Location;
use crate::model::kernel::volume::Volume;

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
        let detault_volume = Volume::new(10).unwrap();
        let default_storage_place =
            StoragePlace::new("bag".to_string(), detault_volume, None).unwrap();
        let storage_places: Vec<StoragePlace> = vec![default_storage_place];

        Self {
            id: CourierId(Uuid::new_v4()),
            location,
            name,
            speed,
            storage_places,
        }
    }

    pub fn add_storage_place(&mut self, name: String, volume: Volume) -> () {
        self.storage_places
            .push(StoragePlace::new(name, volume, None).unwrap());
    }
}
