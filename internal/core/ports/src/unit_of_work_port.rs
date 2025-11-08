use crate::courier_repository_port::CourierRepositoryPort;
use crate::errors::RepositoryError;
use crate::order_repository_port::OrderRepositoryPort;

pub trait UnitOfWorkPort {
    type Uow: UnitOfWorkPort;
    type CourierRepo: CourierRepositoryPort;
    type OrderRepo: OrderRepositoryPort;

    fn transaction<F, T>(&mut self, f: F) -> Result<T, RepositoryError>
    where
        F: for<'tx> FnOnce(&mut Self::Uow) -> Result<T, RepositoryError>;

    fn courier_repo(&mut self) -> Self::CourierRepo;

    fn order_repo(&mut self) -> Self::OrderRepo;
}
