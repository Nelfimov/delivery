use async_trait::async_trait;
use domain::model::order::order_completed_event::OrderCompletedEvent;
use domain::model::order::order_created_event::OrderCreatedEvent;
use ports::events_producer_port::Events;

use crate::errors::command_errors::CommandError;

pub mod commands;
pub mod events;
pub mod queries;

#[trait_variant::make(HttpService: Send)]
pub trait CommandHandler<C, R> {
    type Error;

    async fn execute(&mut self, command: C) -> Result<R, Self::Error>;
}

#[async_trait]
pub trait OrderCreatedSubscriber: Send {
    async fn on_order_created(&mut self, event: OrderCreatedEvent) -> Result<(), CommandError>;
}

#[trait_variant::make(Send)]
pub trait EventHandler<C, R>: Send {
    type Error;

    async fn execute(&mut self, event: C) -> Result<R, Self::Error>;
}

#[trait_variant::make(Send)]
pub trait EventBus: Send {
    fn register_order_created<S>(&mut self, subscriber: S)
    where
        S: OrderCreatedSubscriber + 'static;
    fn register_order_completed<S>(&mut self, subscriber: S)
    where
        S: OrderCompletedSubscriber + 'static;
    async fn commit(&mut self, e: Events) -> Result<(), CommandError>;
}

#[async_trait]
pub trait OrderCompletedSubscriber: Send {
    async fn on_order_completed(&mut self, event: OrderCompletedEvent) -> Result<(), CommandError>;
}
