use std::time::SystemTime;

use uuid::Uuid;

pub struct Message {
    pub id: Uuid,
    pub name: String,
    pub payload: String,
    pub occured_at: SystemTime,
    pub processed_at: Option<SystemTime>,
}

impl Message {
    pub fn new(name: String, payload: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            name,
            payload,
            occured_at: SystemTime::now(),
            processed_at: None,
        }
    }
}
