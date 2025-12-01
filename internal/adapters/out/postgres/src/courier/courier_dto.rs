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

impl std::fmt::Display for CourierDto {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "id: {}, name: {}", self.id, self.name)
    }
}
