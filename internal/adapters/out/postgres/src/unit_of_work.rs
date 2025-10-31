use crate::courier::courier_repository::CourierRepository;
use crate::order::order_repository::OrderRepository;
use diesel::PgConnection;
use diesel::prelude::*;
use ports::errors::RepositoryError;
use ports::unit_of_work_port::UnitOfWorkPort;

pub struct UnitOfWork<'a> {
    pub connection: &'a mut PgConnection,
}

impl<'a> UnitOfWork<'a> {
    pub fn new(conn: &'a mut PgConnection) -> Self {
        Self { connection: conn }
    }

    pub fn courier_repo(&mut self) -> CourierRepository<'_> {
        CourierRepository::new(self.connection)
    }

    pub fn order_repo(&mut self) -> OrderRepository<'_> {
        OrderRepository::new(self.connection)
    }
}

impl<'a> UnitOfWorkPort for UnitOfWork<'a> {
    type Uow<'tx> = UnitOfWork<'tx>;

    fn transaction<F, T>(&mut self, f: F) -> Result<T, RepositoryError>
    where
        F: for<'tx> FnOnce(&mut Self::Uow<'tx>) -> Result<T, RepositoryError>,
    {
        let mut captured: Option<RepositoryError> = None;

        let res = self
            .connection
            .transaction::<T, diesel::result::Error, _>(|tx| {
                let mut tx_uow = UnitOfWork { connection: tx };
                match f(&mut tx_uow) {
                    Ok(v) => Ok(v),
                    Err(e) => {
                        captured = Some(e);
                        Err(diesel::result::Error::RollbackTransaction)
                    }
                }
            });

        match res {
            Ok(v) => Ok(v),
            Err(diesel::result::Error::RollbackTransaction) => {
                Err(captured.unwrap_or_else(|| RepositoryError::DatabaseError("rollback".into())))
            }
            Err(e) => Err(RepositoryError::DatabaseError(e.to_string())),
        }
    }
}
