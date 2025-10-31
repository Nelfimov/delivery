use diesel::prelude::*;
use uuid::Uuid;

use super::storage_place_schema::storage_places;
use crate::courier::courier_dto::CourierDto;

#[derive(Queryable, Identifiable, Insertable, Associations, Debug, Clone)]
#[diesel(belongs_to(CourierDto, foreign_key = courier_id))]
#[diesel(table_name = storage_places)]
#[diesel(check_for_backend(Pg))]
pub struct StoragePlaceDto {
    pub id: Uuid,
    pub courier_id: Uuid,
    pub name: String,
    pub total_volume: i16,
    pub order_id: Option<Uuid>,
}
