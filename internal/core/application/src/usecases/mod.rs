use async_trait::async_trait;
use domain::model::order::order_events::OrderEvent;
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
pub trait Handler: Send + Sync {
    async fn execute(&mut self, event: OrderEvent) -> Result<(), CommandError>;
}

#[allow(async_fn_in_trait)]
pub trait EventBus {
    fn register_order_created(&mut self, subscriber: impl Handler + 'static);
    fn register_order_completed(&mut self, subscriber: impl Handler + 'static);
    async fn commit(&mut self, e: Events) -> Result<(), CommandError>;
}
