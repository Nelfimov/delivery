use async_trait::async_trait;
use domain::model::order::order_events::OrderEvent;
use ports::events_producer_port::Events;
use ports::events_producer_port::EventsProducerPort;

use crate::errors::command_errors::CommandError;
use crate::usecases::Handler;

pub struct OrderCompletedEventHandler<EP>
where
    EP: EventsProducerPort + Send + Sync,
{
    producer: EP,
}

impl<EP> OrderCompletedEventHandler<EP>
where
    EP: EventsProducerPort + Send + Sync,
{
    pub fn new(producer: EP) -> Self {
        Self { producer }
    }
}

#[async_trait]
impl<EP> Handler for OrderCompletedEventHandler<EP>
where
    EP: EventsProducerPort + Send + Sync,
{
    async fn execute(&self, event: OrderEvent) -> Result<(), CommandError> {
        self.producer.publish(Events::Order(event));
        Ok(())
    }
}
