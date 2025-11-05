use diesel::result::Error as DieselError;
use ports::errors::RepositoryError;
use r2d2::Error as R2D2Error;

#[derive(Debug)]
pub enum PostgresError {
    Diesel(DieselError),
    R2D2(R2D2Error),
    Map(String),
}

impl From<DieselError> for PostgresError {
    fn from(err: DieselError) -> Self {
        Self::Diesel(err)
    }
}

impl From<R2D2Error> for PostgresError {
    fn from(err: R2D2Error) -> Self {
        Self::R2D2(err)
    }
}

impl From<PostgresError> for RepositoryError {
    fn from(err: PostgresError) -> Self {
        match err {
            PostgresError::Diesel(e) => RepositoryError::DatabaseError(e.to_string()),
            PostgresError::Map(msg) => RepositoryError::MapError(msg),
            PostgresError::R2D2(e) => RepositoryError::DatabaseError(e.to_string()),
        }
    }
}
