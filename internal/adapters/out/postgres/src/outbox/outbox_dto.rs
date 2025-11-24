use std::time::SystemTime;

use diesel::pg::Pg;
use diesel::prelude::*;
use uuid::Uuid;

use super::outbox_schema::outbox;

#[derive(Queryable, Selectable, Identifiable, Insertable, AsChangeset, QueryableByName)]
#[diesel(table_name = outbox)]
#[diesel(treat_none_as_default_value = false)]
#[diesel(check_for_backend(Pg))]
pub struct OutboxDto {
    pub id: Uuid,
    pub name: String,
    pub payload: String,
    pub occured_at: SystemTime,
    pub processed_at: Option<SystemTime>,
}
