use std::error::Error;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result;

#[derive(Debug)]
pub enum DomainModelError {
    ArgumentCannotBeZero(String),
    ArgumentCannotBeEmpty(String),
    ArgumentAlreadyExists(String),
}

impl Error for DomainModelError {}

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
        }
    }
}
