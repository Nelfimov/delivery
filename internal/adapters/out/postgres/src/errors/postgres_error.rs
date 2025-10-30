use diesel::result::Error as DieselError;
use ports::errors::RepositoryError;

#[derive(Debug)]
pub enum PostgresError {
    Diesel(DieselError),
    Map(String),
}

impl From<DieselError> for PostgresError {
    fn from(err: DieselError) -> Self {
        Self::Diesel(err)
    }
}

impl From<PostgresError> for RepositoryError {
    fn from(err: PostgresError) -> Self {
        match err {
            PostgresError::Diesel(e) => RepositoryError::DatabaseError(e.to_string()),
            PostgresError::Map(msg) => RepositoryError::MapError(msg),
        }
    }
}
