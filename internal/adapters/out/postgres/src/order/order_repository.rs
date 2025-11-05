use diesel::PgConnection;
use diesel::insert_into;
use diesel::prelude::*;
use diesel::r2d2::ConnectionManager;
use diesel::r2d2::Pool;
use diesel::sql_query;
use diesel::update;
use domain::model::order::order_aggregate::Order;
use domain::model::order::order_aggregate::OrderId;
use domain::model::order::order_aggregate::OrderStatus;
use ports::errors::RepositoryError;
use ports::order_repository_port::OrderRepositoryPort;

use crate::errors::postgres_error::PostgresError;

use super::order_dto::OrderDto;
use super::order_schema::orders::dsl::*;

pub struct OrderRepository {
    pool: Pool<ConnectionManager<PgConnection>>,
}

impl OrderRepository {
    pub fn new(pool: Pool<ConnectionManager<PgConnection>>) -> Self {
        Self { pool }
    }
}

impl OrderRepositoryPort for OrderRepository {
    fn add(&mut self, order: &Order) -> Result<(), RepositoryError> {
        let dto: OrderDto = order.into();
        let mut connection = self
            .pool
            .get()
            .map_err(PostgresError::from)
            .map_err(RepositoryError::from)?;

        let _ = insert_into(orders)
            .values(&dto)
            .execute(&mut connection)
            .map_err(PostgresError::from)
            .map_err(RepositoryError::from)?;
        Ok(())
    }

    fn update(&mut self, order: &Order) -> Result<(), RepositoryError> {
        let dto: OrderDto = order.into();
        let mut connection = self
            .pool
            .get()
            .map_err(PostgresError::from)
            .map_err(RepositoryError::from)?;

        update(orders.find(dto.id))
            .set(&dto)
            .execute(&mut connection)
            .map_err(PostgresError::from)
            .map_err(RepositoryError::from)?;
        Ok(())
    }

    fn get_by_id(&mut self, order_id: OrderId) -> Result<Order, RepositoryError> {
        let mut connection = self
            .pool
            .get()
            .map_err(PostgresError::from)
            .map_err(RepositoryError::from)?;

        let order: OrderDto = orders
            .find(order_id.value())
            .first(&mut connection)
            .map_err(PostgresError::from)
            .map_err(RepositoryError::from)?;

        order.try_into().map_err(RepositoryError::MapError)
    }

    fn get_any_new(&mut self) -> Result<Order, RepositoryError> {
        let mut connection = self
            .pool
            .get()
            .map_err(PostgresError::from)
            .map_err(RepositoryError::from)?;

        let row: OrderDto = orders
            .filter(status.eq(OrderStatus::Created.to_string()))
            .first(&mut connection)
            .map_err(PostgresError::from)
            .map_err(RepositoryError::from)?;

        let result: Order = row.try_into().map_err(RepositoryError::MapError)?;

        Ok(result)
    }

    fn get_all_assigned(&mut self) -> Result<Vec<Order>, RepositoryError> {
        let s: String = OrderStatus::Assigned.into();
        let mut connection = self
            .pool
            .get()
            .map_err(PostgresError::from)
            .map_err(RepositoryError::from)?;

        let rows: Vec<OrderDto> = orders
            .filter(status.eq(s))
            .load(&mut connection)
            .map_err(PostgresError::from)
            .map_err(RepositoryError::from)?;

        let result: Result<Vec<Order>, RepositoryError> = rows
            .into_iter()
            .map(|dto| dto.try_into().map_err(RepositoryError::MapError))
            .collect();

        result
    }

    fn raw(&mut self, query: String) -> Result<Vec<Order>, RepositoryError> {
        let mut connection = self
            .pool
            .get()
            .map_err(PostgresError::from)
            .map_err(RepositoryError::from)?;

        let rows: Vec<OrderDto> = sql_query(query)
            .load(&mut connection)
            .map_err(PostgresError::from)
            .map_err(RepositoryError::from)?;

        rows.into_iter()
            .map(|dto| dto.try_into().map_err(RepositoryError::MapError))
            .collect()
    }
}
