use diesel::PgConnection;
use diesel::prelude::*;
use diesel::r2d2::ConnectionManager;
use domain::model::kernel::message::Message;
use ports::errors::RepositoryError;
use ports::outbox_repository::OutboxRepositoryPort;
use r2d2::Pool;

use super::outbox_schema::outbox::dsl::*;

pub struct OutboxRepository {
    pool: Pool<ConnectionManager<PgConnection>>,
}

impl OutboxRepository {
    pub fn new(pool: Pool<ConnectionManager<PgConnection>>) -> Self {
        Self { pool }
    }
}

impl OutboxRepositoryPort for OutboxRepository {
    fn update(&mut self, message: &Message) -> Result<(), RepositoryError> {
        todo!()
    }

    fn get_not_published_messages(&mut self) -> Result<Vec<Message>, RepositoryError> {
        todo!()
    }
}
