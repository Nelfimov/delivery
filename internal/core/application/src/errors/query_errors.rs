use std::error::Error;
use std::fmt::Display;
use std::fmt::Result;

use domain::errors::domain_model_errors::DomainModelError;
use ports::errors::RepositoryError;

#[derive(Debug)]
pub enum QueryError {
    ArgumentError(String),
    ExecutionError(String),
}

impl Error for QueryError {}

impl Display for QueryError {
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

impl From<DomainModelError> for QueryError {
    fn from(value: DomainModelError) -> Self {
        Self::ArgumentError(value.to_string())
    }
}

impl From<RepositoryError> for QueryError {
    fn from(value: RepositoryError) -> Self {
        Self::ExecutionError(value.to_string())
    }
}
