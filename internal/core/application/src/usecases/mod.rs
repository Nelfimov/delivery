pub mod commands;
pub mod queries;

pub trait CommandHandler<C, R> {
    type Error;

    fn execute(&mut self, command: C) -> Result<R, Self::Error>;
}
