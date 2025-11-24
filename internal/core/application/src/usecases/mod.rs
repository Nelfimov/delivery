use async_trait::async_trait;
use domain::model::order::order_events::OrderEvent;

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
    async fn execute(&self, event: OrderEvent) -> Result<(), CommandError>;
}
