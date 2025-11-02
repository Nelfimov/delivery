use super::courier_schema::couriers;
use diesel::pg::Pg;
use diesel::prelude::*;
use uuid::Uuid;

#[derive(Queryable, Selectable, Identifiable, Insertable, AsChangeset, Clone)]
#[diesel(table_name = couriers)]
#[diesel(treat_none_as_default_value = false)]
#[diesel(check_for_backend(Pg))]
pub struct CourierDto {
    pub id: Uuid,
    pub name: String,
    pub speed: i16,
    pub location_x: i16,
    pub location_y: i16,
}
