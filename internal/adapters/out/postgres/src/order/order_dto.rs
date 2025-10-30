use diesel::pg::Pg;
use diesel::prelude::*;
use uuid::Uuid;

use super::order_schema::orders;

#[derive(Queryable, Selectable, Identifiable, Insertable, AsChangeset)]
#[diesel(table_name = orders)]
#[diesel(treat_none_as_default_value = false)]
#[diesel(check_for_backend(Pg))]
pub struct OrderDto {
    pub id: Uuid,
    pub courier_id: Option<Uuid>,
    pub location_x: i16,
    pub location_y: i16,
    pub volume: i16,
    pub status: String,
}
