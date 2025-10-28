use diesel::pg::Pg;
use diesel::prelude::*;
use uuid::Uuid;

diesel::table! {
    orders {
        id -> diesel::sql_types::Uuid,
        courier_id -> diesel::sql_types::Nullable<diesel::sql_types::Uuid>,
        location_x -> diesel::sql_types::SmallInt,
        location_y -> diesel::sql_types::SmallInt,
        volume -> diesel::sql_types::SmallInt,
        status -> diesel::sql_types::Text,
    }
}

#[derive(Queryable, Selectable, Identifiable, Insertable)]
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
