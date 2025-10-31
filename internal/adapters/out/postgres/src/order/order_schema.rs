diesel::table! {
    orders {
        id -> Uuid,
        courier_id -> Nullable<Uuid>,
        location_x -> SmallInt,
        location_y -> SmallInt,
        volume -> SmallInt,
        status -> Text,
    }
}
