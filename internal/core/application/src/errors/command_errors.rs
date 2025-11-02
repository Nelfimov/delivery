use std::error::Error;
use std::fmt::Display;
use std::fmt::Result;

use domain::errors::domain_model_errors::DomainModelError;

#[derive(Debug)]
pub enum CommandError {
    ArgumentError(String),
    ExecutionError(String),
}

impl Error for CommandError {}

impl Display for CommandError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result {
        match self {
            Self::ArgumentError(msg) => {
                write!(f, "Command arguments error: {}", msg)
            }
            Self::ExecutionError(msg) => {
                write!(f, "Command execution failure: {}", msg)
            }
        }
    }
}

impl From<DomainModelError> for CommandError {
    fn from(value: DomainModelError) -> Self {
        Self::ArgumentError(value.to_string())
    }
}
