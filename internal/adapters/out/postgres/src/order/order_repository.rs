use diesel::insert_into;
use diesel::prelude::*;
use diesel::r2d2::ConnectionManager;
use diesel::r2d2::Pool;
use diesel::r2d2::PooledConnection;
use diesel::sql_query;
use diesel::update;
use diesel::PgConnection;
use domain::model::order::order_aggregate::Order;
use domain::model::order::order_aggregate::OrderId;
use domain::model::order::order_aggregate::OrderStatus;
use ports::errors::RepositoryError;
use ports::order_repository_port::OrderRepositoryPort;

use crate::errors::postgres_error::PostgresError;

use super::order_dto::OrderDto;
use super::order_schema::orders::dsl::*;
use std::ops::DerefMut;
use std::ptr::NonNull;

pub struct OrderRepository {
    pool: Pool<ConnectionManager<PgConnection>>,
    shared_connection: Option<NonNull<PgConnection>>,
}

// SAFETY: `shared_connection` is only accessed via `&mut self`, preventing cross-thread use.
unsafe impl Send for OrderRepository {}

impl OrderRepository {
    pub fn new(pool: Pool<ConnectionManager<PgConnection>>) -> Self {
        Self {
            pool,
            shared_connection: None,
        }
    }

    pub fn with_shared_connection(
        pool: Pool<ConnectionManager<PgConnection>>,
        connection: NonNull<PgConnection>,
    ) -> Self {
        Self {
            pool,
            shared_connection: Some(connection),
        }
    }

    fn connection(&mut self) -> Result<RepositoryConn<'_>, RepositoryError> {
        if let Some(conn_ptr) = self.shared_connection {
            // SAFETY: conn_ptr originates from an active transaction and remains valid
            // while the transaction closure executes.
            let conn = unsafe { &mut *conn_ptr.as_ptr() };
            return Ok(RepositoryConn::Borrowed(conn));
        }

        let conn = self
            .pool
            .get()
            .map_err(PostgresError::from)
            .map_err(RepositoryError::from)?;

        Ok(RepositoryConn::Pooled(conn))
    }
}

impl OrderRepositoryPort for OrderRepository {
    fn add(&mut self, order: &Order) -> Result<(), RepositoryError> {
        let dto: OrderDto = order.into();
        let mut connection = self.connection()?;

        let _ = insert_into(orders)
            .values(&dto)
            .execute(connection.as_mut())
            .map_err(PostgresError::from)
            .map_err(RepositoryError::from)?;
        Ok(())
    }

    fn update(&mut self, order: &Order) -> Result<(), RepositoryError> {
        let dto: OrderDto = order.into();
        let mut connection = self.connection()?;

        update(orders.find(dto.id))
            .set(&dto)
            .execute(connection.as_mut())
            .map_err(PostgresError::from)
            .map_err(RepositoryError::from)?;
        Ok(())
    }

    fn get_by_id(&mut self, order_id: OrderId) -> Result<Order, RepositoryError> {
        let mut connection = self.connection()?;

        let order: OrderDto = orders
            .find(order_id.value())
            .first(connection.as_mut())
            .map_err(PostgresError::from)
            .map_err(RepositoryError::from)?;

        order.try_into().map_err(RepositoryError::MapError)
    }

    fn get_any_new(&mut self) -> Result<Order, RepositoryError> {
        let mut connection = self.connection()?;

        let row: OrderDto = orders
            .filter(status.eq(OrderStatus::Created.to_string()))
            .first(connection.as_mut())
            .map_err(PostgresError::from)
            .map_err(RepositoryError::from)?;

        let result: Order = row.try_into().map_err(RepositoryError::MapError)?;

        Ok(result)
    }

    fn get_all_assigned(&mut self) -> Result<Vec<Order>, RepositoryError> {
        let s: String = OrderStatus::Assigned.into();
        let mut connection = self.connection()?;

        let rows: Vec<OrderDto> = orders
            .filter(status.eq(s))
            .load(connection.as_mut())
            .map_err(PostgresError::from)
            .map_err(RepositoryError::from)?;

        let result: Result<Vec<Order>, RepositoryError> = rows
            .into_iter()
            .map(|dto| dto.try_into().map_err(RepositoryError::MapError))
            .collect();

        result
    }

    fn raw(&mut self, query: String) -> Result<Vec<Order>, RepositoryError> {
        let mut connection = self.connection()?;

        let rows: Vec<OrderDto> = sql_query(query)
            .load(connection.as_mut())
            .map_err(PostgresError::from)
            .map_err(RepositoryError::from)?;

        rows.into_iter()
            .map(|dto| dto.try_into().map_err(RepositoryError::MapError))
            .collect()
    }
}

enum RepositoryConn<'a> {
    Borrowed(&'a mut PgConnection),
    Pooled(PooledConnection<ConnectionManager<PgConnection>>),
}

impl<'a> RepositoryConn<'a> {
    fn as_mut(&mut self) -> &mut PgConnection {
        match self {
            RepositoryConn::Borrowed(conn) => conn,
            RepositoryConn::Pooled(conn) => conn.deref_mut(),
        }
    }
}
