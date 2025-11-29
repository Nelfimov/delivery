use domain::errors::domain_model_errors::DomainModelError;
use domain::model::courier::courier_aggregate::CourierId;
use domain::model::courier::storage_place::StoragePlace;
use domain::model::kernel::volume::Volume;
use domain::model::order::order_aggregate::OrderId;
use uuid::Uuid;

use super::storage_place_dto::StoragePlaceDto;

impl From<(&StoragePlace, CourierId)> for StoragePlaceDto {
    fn from((sp, courier_id): (&StoragePlace, CourierId)) -> Self {
        Self {
            id: *sp.id(),
            courier_id: courier_id.0,
            name: sp.name().to_string(),
            total_volume: sp.total_volume() as i16,
            order_id: sp.order_id().map(|id| id.value()),
        }
    }
}

impl StoragePlaceDto {
    pub fn from_dto(v: StoragePlace, courier_id: Uuid) -> Self {
        Self {
            id: v.id().to_owned(),
            name: v.name().to_string(),
            total_volume: v.total_volume() as i16,
            order_id: v.order_id().map(|f| f.0).to_owned(),
            courier_id,
        }
    }
}

impl TryFrom<StoragePlaceDto> for StoragePlace {
    type Error = DomainModelError;

    fn try_from(dto: StoragePlaceDto) -> Result<Self, Self::Error> {
        Ok(Self::restore(
            dto.id,
            dto.name,
            Volume::new(dto.total_volume as u16)?,
            dto.order_id.map(OrderId::new),
        ))
    }
}
