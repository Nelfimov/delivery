use domain::model::kernel::message::Message;

use crate::outbox::outbox_dto::OutboxDto;

impl From<Message> for OutboxDto {
    fn from(m: Message) -> Self {
        Self {
            id: m.id,
            name: m.name,
            payload: m.payload,
            occured_at: m.occured_at,
            processed_at: m.processed_at,
        }
    }
}

impl From<&Message> for OutboxDto {
    fn from(m: &Message) -> Self {
        Self {
            id: m.id,
            name: m.name.clone(),
            payload: m.payload.clone(),
            occured_at: m.occured_at,
            processed_at: m.processed_at,
        }
    }
}

impl From<OutboxDto> for Message {
    fn from(row: OutboxDto) -> Self {
        Self {
            id: row.id,
            name: row.name,
            payload: row.payload,
            occured_at: row.occured_at,
            processed_at: row.processed_at,
        }
    }
}
