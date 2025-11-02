diesel::table! {
    storage_places {
        id -> Uuid,
        courier_id -> Uuid,
        name -> Text,
        total_volume -> SmallInt,
        order_id -> Nullable<Uuid>,
    }
}

use crate::courier::courier_schema::couriers;

diesel::joinable!(storage_places -> couriers (courier_id));
diesel::allow_tables_to_appear_in_same_query!(couriers, storage_places);
