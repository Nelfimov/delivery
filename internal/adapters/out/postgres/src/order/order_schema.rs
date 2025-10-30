use diesel::pg::Pg;
use diesel::prelude::*;
use uuid::Uuid;

diesel::table! {
    orders (id) {
        id -> diesel::sql_types::Uuid,
        courier_id -> diesel::sql_types::Nullable<diesel::sql_types::Uuid>,
        location_x -> diesel::sql_types::SmallInt,
        location_y -> diesel::sql_types::SmallInt,
        volume -> diesel::sql_types::SmallInt,
        status -> diesel::sql_types::Text,
    }
}
