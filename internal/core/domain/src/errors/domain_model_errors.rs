use std::error::Error;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result;

#[derive(Debug)]
pub enum DomainModelError {
    ArgumentCannotBeZero(String),
    ArgumentCannotBeEmpty(String),
    ArgumentAlreadyExists(String),
    UnmetRequirement(String),
    MapError(String),
}

impl Error for DomainModelError {}

impl From<DomainModelError> for String {
    fn from(value: DomainModelError) -> Self {
        value.to_string()
    }
}

impl Display for DomainModelError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            DomainModelError::ArgumentCannotBeZero(msg) => {
                write!(f, "Argument cannot be zero: {}", msg)
            }
            DomainModelError::ArgumentCannotBeEmpty(msg) => {
                write!(f, "Argument cannot be empty string: {}", msg)
            }
            DomainModelError::ArgumentAlreadyExists(msg) => {
                write!(
                    f,
                    "Argument already exists and cannot be overwritten: {}",
                    msg
                )
            }
            DomainModelError::UnmetRequirement(msg) => {
                write!(f, "Requirement is not met: {}", msg)
            }
            DomainModelError::MapError(msg) => {
                write!(f, "Error while mapping domain model: {}", msg)
            }
        }
    }
}
