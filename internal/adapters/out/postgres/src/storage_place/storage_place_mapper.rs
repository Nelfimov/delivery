use domain::errors::domain_model_errors::DomainModelError;
use domain::model::courier::courier_aggregate::CourierId;
use domain::model::courier::storage_place::StoragePlace;
use domain::model::kernel::volume::Volume;
use domain::model::order::order_aggregate::OrderId;

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
