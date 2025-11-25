use diesel::PgConnection;
use diesel::dsl::insert_into;
use diesel::dsl::update;
use diesel::prelude::*;
use diesel::r2d2::ConnectionManager;
use domain::model::kernel::message::Message;
use ports::errors::RepositoryError;
use ports::outbox_repository::OutboxRepositoryPort;
use r2d2::Pool;
use r2d2::PooledConnection;

use crate::errors::postgres_error::PostgresError;
use crate::outbox::outbox_dto::OutboxDto;

use super::outbox_schema::outbox::dsl::*;

pub struct OutboxRepository {
    pool: Pool<ConnectionManager<PgConnection>>,
}

impl OutboxRepository {
    pub fn new(pool: Pool<ConnectionManager<PgConnection>>) -> Self {
        Self { pool }
    }

    fn get_conn(
        &mut self,
    ) -> Result<PooledConnection<ConnectionManager<PgConnection>>, RepositoryError> {
        self.pool
            .get()
            .map_err(PostgresError::from)
            .map_err(RepositoryError::from)
    }
}

impl OutboxRepositoryPort for OutboxRepository {
    fn add(&mut self, message: &Message) -> Result<(), RepositoryError> {
        let dto: OutboxDto = message.into();

        let mut conn = self.get_conn()?;

        insert_into(outbox)
            .values(&dto)
            .execute(&mut conn)
            .map_err(PostgresError::from)
            .map_err(RepositoryError::from)?;

        Ok(())
    }

    fn update(&mut self, message: &Message) -> Result<(), RepositoryError> {
        let dto: OutboxDto = message.into();

        let mut conn = self.get_conn()?;

        update(outbox.find(dto.id))
            .set(&dto)
            .execute(&mut conn)
            .map_err(PostgresError::from)
            .map_err(RepositoryError::from)?;

        Ok(())
    }

    fn get_not_published_messages(&mut self) -> Result<Vec<Message>, RepositoryError> {
        let mut conn = self.get_conn()?;

        let rows: Vec<OutboxDto> = outbox
            .filter(processed_at.is_null())
            .limit(10)
            .load(&mut conn)
            .map_err(PostgresError::from)
            .map_err(RepositoryError::from)?;

        Ok(rows.iter().map(Message::from).collect())
    }
}
