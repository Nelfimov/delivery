use crate::errors::command_errors::CommandError;

pub struct MoveCouriersCommand;

impl MoveCouriersCommand {
    pub fn new() -> Result<Self, CommandError> {
        Ok(Self {})
    }
}
