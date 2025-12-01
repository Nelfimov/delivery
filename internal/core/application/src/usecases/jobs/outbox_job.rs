use std::time::SystemTime;

use domain::model::kernel::message::Message;
use ports::events_producer_port::Events;
use ports::events_producer_port::EventsProducerPort;
use ports::outbox_repository::OutboxRepositoryPort;
use tracing::debug;
use tracing::warn;

use crate::errors::command_errors::CommandError;
use crate::usecases::JobHandler;

pub struct OutboxJob<OR, EP>
where
    OR: OutboxRepositoryPort + Send + Sync,
    EP: EventsProducerPort + Send + Sync,
{
    outbox_repo: OR,
    event_producer: EP,
}

impl<OR, EP> OutboxJob<OR, EP>
where
    OR: OutboxRepositoryPort + Send + Sync,
    EP: EventsProducerPort + Send + Sync,
{
    pub fn new(outbox_repo: OR, event_producer: EP) -> Self {
        Self {
            outbox_repo,
            event_producer,
        }
    }
}

#[async_trait::async_trait]
impl<OR, EP> JobHandler for OutboxJob<OR, EP>
where
    OR: OutboxRepositoryPort + Send + Sync,
    EP: EventsProducerPort + Send + Sync,
{
    async fn execute(&mut self) -> Result<(), CommandError> {
        debug!("looking for unprocessed events");

        let messages = self
            .outbox_repo
            .get_not_published_messages()
            .map_err(CommandError::from)?;

        if messages.is_empty() {
            debug!("no unprocessed messages");
            return Ok(());
        }

        debug!("unprocessed messages: {}", messages.len());

        for mut message in messages {
            debug!("publishing message: {:?}", message);
            let event = Events::try_from(&message).ok();
            if let Some(e) = event {
                self.event_producer.publish(e);
                message.processed_at = Some(SystemTime::now());
                if let Err(e) = self.outbox_repo.update(&message) {
                    warn!("error while updating outbox repo: {}", e);
                }
            }
        }

        Ok(())
    }
}
