pub mod commands;
pub mod events;
pub mod queries;

#[trait_variant::make(HttpService: Send)]
pub trait CommandHandler<C, R> {
    type Error;

    async fn execute(&mut self, command: C) -> Result<R, Self::Error>;
}
