use std::error::Error;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result;

#[derive(Debug)]
pub enum RepositoryError {
    DatabaseError(String),
    MapError(String),
    NotFound(String),
}

impl Error for RepositoryError {}

impl Display for RepositoryError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            RepositoryError::DatabaseError(msg) => {
                write!(f, "Database error: {}", msg)
            }
            RepositoryError::MapError(msg) => {
                write!(f, "Error while mapping domain model: {}", msg)
            }
            RepositoryError::NotFound(msg) => {
                write!(f, "Could not find: {}", msg)
            }
        }
    }
}

impl From<String> for RepositoryError {
    fn from(value: String) -> Self {
        Self::MapError(value)
    }
}
