use crate::errors::command_errors::CommandError;

#[derive(Debug)]
pub struct MoveCouriersCommand;

impl MoveCouriersCommand {
    pub fn new() -> Result<Self, CommandError> {
        Ok(Self {})
    }
}
