use domain::model::order::order_events::OrderEvent;
use ports::events_producer_port::Events;
use ports::events_producer_port::EventsProducerPort;

use crate::errors::command_errors::CommandError;
use crate::usecases::EventHandler;

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

impl<EP> EventHandler<OrderEvent, ()> for OrderCompletedEventHandler<EP>
where
    EP: EventsProducerPort + Send + Sync,
{
    type Error = CommandError;

    async fn execute(&mut self, event: OrderEvent) -> Result<(), Self::Error> {
        self.producer.publish(Events::Order(event));
        Ok(())
    }
}
