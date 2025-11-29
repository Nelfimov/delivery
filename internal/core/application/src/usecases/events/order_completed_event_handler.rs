use async_trait::async_trait;
use domain::model::kernel::message::Message;
use domain::model::order::order_events::OrderEvent;
use ports::outbox_repository::OutboxRepositoryPort;

use crate::errors::command_errors::CommandError;
use crate::usecases::Handler;

pub struct OrderCompletedEventHandler<OR>
where
    OR: OutboxRepositoryPort + Send + Sync,
{
    outbox_repo: OR,
}

impl<OR> OrderCompletedEventHandler<OR>
where
    OR: OutboxRepositoryPort + Send + Sync,
{
    pub fn new(outbox_repo: OR) -> Self {
        Self { outbox_repo }
    }
}

#[async_trait]
impl<OR> Handler for OrderCompletedEventHandler<OR>
where
    OR: OutboxRepositoryPort + Send + Sync,
{
    async fn execute(&mut self, event: OrderEvent) -> Result<(), CommandError> {
        match event {
            OrderEvent::Completed { 0: e } => {
                let message = Message::new(
                    e.name.clone(),
                    serde_json::to_string(&e).map_err(|_| {
                        CommandError::ExecutionError("could not serialize event".to_string())
                    })?,
                );
                self.outbox_repo.add(&message).map_err(CommandError::from)
            }
            _ => Ok(()),
        }
    }
}
