use domain::model::courier::courier_aggregate::Courier;
use domain::model::courier::courier_aggregate::CourierId;
use domain::model::courier::courier_aggregate::CourierName;
use domain::model::courier::courier_aggregate::CourierSpeed;
use domain::model::courier::storage_place::StoragePlace;
use domain::model::kernel::location::Location;
use ports::courier_repository_port::GetAllCouriersResponse;

use crate::courier::courier_dto::CourierDto;
use crate::storage_place::storage_place_dto::StoragePlaceDto;

impl From<&Courier> for CourierDto {
    fn from(order: &Courier) -> Self {
        Self {
            id: order.id().0,
            name: order.name().clone(),
            speed: *order.speed() as i16,
            location_x: order.location().x() as i16,
            location_y: order.location().y() as i16,
        }
    }
}

impl From<Courier> for CourierDto {
    fn from(order: Courier) -> Self {
        Self {
            id: order.id().0,
            name: order.name().clone(),
            speed: *order.speed() as i16,
            location_x: order.location().x() as i16,
            location_y: order.location().y() as i16,
        }
    }
}

pub struct CourierRecord(pub CourierDto, pub Vec<StoragePlaceDto>);

impl TryFrom<CourierRecord> for Courier {
    type Error = String;

    fn try_from(v: CourierRecord) -> Result<Self, Self::Error> {
        let courier_dto = v.0;
        let storage_places_dto = v.1;

        let location = Location::new(courier_dto.location_x as u8, courier_dto.location_y as u8)?;
        let storage_places = storage_places_dto
            .into_iter()
            .map(StoragePlace::try_from)
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Courier::restore(
            CourierId(courier_dto.id),
            CourierName(courier_dto.name),
            CourierSpeed(courier_dto.speed as u8),
            location,
            storage_places,
        ))
    }
}
