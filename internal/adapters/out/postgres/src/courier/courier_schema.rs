diesel::table! {
    couriers {
        id -> Uuid,
        name -> Text,
        speed -> SmallInt,
        location_x -> SmallInt,
        location_y -> SmallInt,
    }
}
