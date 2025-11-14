use async_trait::async_trait;
use domain::model::order::order_completed_event::OrderCompletedEvent;
use domain::model::order::order_created_event::OrderCreatedEvent;
use ports::events_producer_port::Events;

use crate::errors::command_errors::CommandError;
use crate::usecases::CommandHandler;

type OrderCreatedBox = Box<dyn OrderCreatedSubscriber>;
type OrderCompletedBox = Box<dyn OrderCompletedSubscriber>;

#[derive(Default)]
pub struct OrdersEventBus {
    order_created_subscribers: Vec<OrderCreatedBox>,
    order_completed_subscribers: Vec<OrderCompletedBox>,
}

impl OrdersEventBus {
    pub fn new() -> Self {
        Self {
            order_created_subscribers: Vec::new(),
            order_completed_subscribers: Vec::new(),
        }
    }

    pub fn register_order_created<S>(&mut self, subscriber: S)
    where
        S: OrderCreatedSubscriber + 'static,
    {
        self.order_created_subscribers.push(Box::new(subscriber));
    }

    pub fn register_order_completed<S>(&mut self, subscriber: S)
    where
        S: OrderCompletedSubscriber + 'static,
    {
        self.order_completed_subscribers.push(Box::new(subscriber));
    }

    pub async fn publish(&mut self, event: Events) -> Result<(), CommandError> {
        match event {
            Events::OrderCreated(event) => {
                for subscriber in &mut self.order_created_subscribers {
                    subscriber.on_order_created(event.clone()).await?;
                }
            }
            Events::OrderCompleted(event) => {
                for subscriber in &mut self.order_completed_subscribers {
                    subscriber.on_order_completed(event.clone()).await?;
                }
            }
        };

        Ok(())
    }
}

#[async_trait(?Send)]
pub trait OrderCreatedSubscriber: Send {
    async fn on_order_created(&mut self, event: OrderCreatedEvent) -> Result<(), CommandError>;
}

#[async_trait(?Send)]
impl<T> OrderCreatedSubscriber for T
where
    T: CommandHandler<OrderCreatedEvent, (), Error = CommandError> + Send,
{
    async fn on_order_created(&mut self, event: OrderCreatedEvent) -> Result<(), CommandError> {
        self.execute(event).await
    }
}

#[async_trait(?Send)]
pub trait OrderCompletedSubscriber: Send {
    async fn on_order_completed(&mut self, event: OrderCompletedEvent) -> Result<(), CommandError>;
}

#[async_trait(?Send)]
impl<T> OrderCompletedSubscriber for T
where
    T: CommandHandler<OrderCompletedEvent, (), Error = CommandError> + Send,
{
    async fn on_order_completed(&mut self, event: OrderCompletedEvent) -> Result<(), CommandError> {
        self.execute(event).await
    }
}
