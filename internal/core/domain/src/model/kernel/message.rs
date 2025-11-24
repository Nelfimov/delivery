use std::time::SystemTime;

use uuid::Uuid;

pub struct Message {
    pub id: Uuid,
    pub name: String,
    // TODO: use byte
    pub payload: String,
    pub occured_at: SystemTime,
    pub processed_at: Option<SystemTime>,
}
