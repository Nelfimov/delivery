use domain::model::kernel::message::Message;

use crate::errors::RepositoryError;

pub trait OutboxRepositoryPort {
    fn update(&mut self, message: &Message) -> Result<(), RepositoryError>;
    fn get_not_published_messages(&mut self) -> Result<Vec<Message>, RepositoryError>;
}
