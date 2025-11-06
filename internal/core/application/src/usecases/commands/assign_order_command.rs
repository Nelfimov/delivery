use crate::errors::command_errors::CommandError;

pub struct AssignOrderCommand;

impl AssignOrderCommand {
    pub fn new() -> Result<Self, CommandError> {
        Ok(Self {})
    }
}
