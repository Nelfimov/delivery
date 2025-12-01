use diesel::prelude::*;
use uuid::Uuid;

use super::storage_place_schema::storage_places;
use crate::courier::courier_dto::CourierDto;

#[derive(Queryable, Identifiable, Insertable, Associations, Debug, Clone, AsChangeset)]
#[diesel(belongs_to(CourierDto, foreign_key = courier_id))]
#[diesel(table_name = storage_places)]
#[diesel(check_for_backend(Pg))]
#[diesel(treat_none_as_null = true)]
pub struct StoragePlaceDto {
    pub id: Uuid,
    pub courier_id: Uuid,
    pub name: String,
    pub total_volume: i16,
    pub order_id: Option<Uuid>,
}

impl std::fmt::Display for StoragePlaceDto {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "id: {}, courier_id: {}", self.id, self.courier_id)
    }
}
