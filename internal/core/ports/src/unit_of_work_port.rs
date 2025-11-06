use crate::courier_repository_port::CourierRepositoryPort;
use crate::errors::RepositoryError;
use crate::order_repository_port::OrderRepositoryPort;

pub trait UnitOfWorkPort {
    type Uow<'tx>: UnitOfWorkPort + 'tx;
    type CourierRepo<'tx>: CourierRepositoryPort
    where
        Self: 'tx;
    type OrderRepo<'tx>: OrderRepositoryPort
    where
        Self: 'tx;

    fn transaction<F, T>(&mut self, f: F) -> Result<T, RepositoryError>
    where
        F: for<'tx> FnOnce(&mut Self::Uow<'tx>) -> Result<T, RepositoryError>;

    fn courier_repo(&mut self) -> Self::CourierRepo<'_>;

    fn order_repo(&mut self) -> Self::OrderRepo<'_>;
}
