use domain::model::order::order_created_event::OrderCreatedEvent;
use ports::events_producer_port::Events;
use ports::events_producer_port::EventsProducerPort;

use crate::errors::command_errors::CommandError;
use crate::usecases::EventHandler;

pub struct OrderCreatedEventHandler<EP>
where
    EP: EventsProducerPort,
{
    producer: EP,
}

impl<EP> OrderCreatedEventHandler<EP>
where
    EP: EventsProducerPort,
{
    pub fn new(producer: EP) -> Self {
        Self { producer }
    }
}

impl<EP> EventHandler<OrderCreatedEvent, ()> for OrderCreatedEventHandler<EP>
where
    EP: EventsProducerPort,
{
    type Error = CommandError;

    async fn execute(&mut self, event: OrderCreatedEvent) -> Result<(), Self::Error> {
        self.producer.publish(Events::OrderCreated(event));
        Ok(())
    }
}
