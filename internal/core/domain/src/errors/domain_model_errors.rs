use std::error::Error;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result;

#[derive(Debug)]
pub enum DomainModelCreationError {
    ArgumentCannotBeZero(String),
    ArgumentCannotBeEmpty(String),
}

impl Error for DomainModelCreationError {}

impl Display for DomainModelCreationError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            DomainModelCreationError::ArgumentCannotBeZero(msg) => {
                write!(f, "Argument cannot be zero: {}", msg)
            }
            DomainModelCreationError::ArgumentCannotBeEmpty(msg) => {
                write!(f, "Argument cannot be empty string: {}", msg)
            }
        }
    }
}
