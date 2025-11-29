use ports::events_producer_port::Events;
use ports::events_producer_port::EventsProducerPort;
use ports::outbox_repository::OutboxRepositoryPort;

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
        let messages = self
            .outbox_repo
            .get_not_published_messages()
            .map_err(CommandError::from)?;

        let events: Vec<Events> = messages
            .iter()
            .filter_map(|f| {
                let event = Events::try_from(f);
                event.ok()
            })
            .collect();

        for event in events {
            self.event_producer.publish(event);
        }

        Ok(())
    }
}
