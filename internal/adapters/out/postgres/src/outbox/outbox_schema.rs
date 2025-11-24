diesel::table! {
    outbox {
        id -> Uuid,
        name -> Text,
        payload -> Text,
        occured_at -> Timestamp,
        processed_at -> Nullable<Timestamp>,
    }
}
