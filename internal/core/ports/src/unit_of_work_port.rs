use crate::errors::RepositoryError;

pub trait UnitOfWorkPort {
    type Uow<'tx>: UnitOfWorkPort + 'tx;

    fn transaction<F, T>(&mut self, f: F) -> Result<T, RepositoryError>
    where
        F: for<'tx> FnOnce(&mut Self::Uow<'tx>) -> Result<T, RepositoryError>;
}
