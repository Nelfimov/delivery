use diesel::PgConnection;
use diesel::prelude::*;
use diesel::r2d2::ConnectionManager;
use diesel::r2d2::Pool;
use ports::errors::RepositoryError;
use ports::unit_of_work_port::UnitOfWorkPort;

use crate::courier::courier_repository::CourierRepository;
use crate::errors::postgres_error::PostgresError;
use crate::order::order_repository::OrderRepository;

pub struct UnitOfWork {
    pub pool: Pool<ConnectionManager<PgConnection>>,
}

impl UnitOfWork {
    pub fn new(pool: Pool<ConnectionManager<PgConnection>>) -> Self {
        Self { pool }
    }
}

impl UnitOfWorkPort for UnitOfWork {
    type Uow = UnitOfWork;
    type CourierRepo = CourierRepository;
    type OrderRepo = OrderRepository;

    fn courier_repo(&mut self) -> Self::CourierRepo {
        CourierRepository::new(self.pool.clone())
    }

    fn order_repo(&mut self) -> Self::OrderRepo {
        OrderRepository::new(self.pool.clone())
    }

    fn transaction<F, T>(&mut self, f: F) -> Result<T, RepositoryError>
    where
        F: for<'tx> FnOnce(&mut Self::Uow) -> Result<T, RepositoryError>,
    {
        let mut captured: Option<RepositoryError> = None;
        let mut connection = self
            .pool
            .get()
            .map_err(PostgresError::from)
            .map_err(RepositoryError::from)?;

        let res = connection.transaction::<T, diesel::result::Error, _>(|tx| {
            let mut tx_uow = UnitOfWork {
                pool: self.pool.clone(),
            };

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
