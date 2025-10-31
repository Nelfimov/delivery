use diesel::PgConnection;
use diesel::insert_into;
use diesel::prelude::*;
use diesel::update;
use domain::model::order::order_aggregate::Order;
use domain::model::order::order_aggregate::OrderId;
use domain::model::order::order_aggregate::OrderStatus;
use ports::errors::RepositoryError;
use ports::order_repository_port::OrderRepositoryPort;

use crate::errors::postgres_error::PostgresError;

use super::order_dto::OrderDto;
use super::order_schema::orders::dsl::*;

pub struct OrderRepository<'a> {
    connection: &'a mut PgConnection,
}

impl<'a> OrderRepository<'a> {
    pub fn new(connection: &'a mut PgConnection) -> Self {
        Self { connection }
    }
}

impl<'a> OrderRepositoryPort for OrderRepository<'a> {
    fn add(&mut self, order: &Order) -> Result<(), RepositoryError> {
        let dto: OrderDto = order.into();
        let _ = insert_into(orders)
            .values(&dto)
            .execute(self.connection)
            .map_err(PostgresError::from)
            .map_err(RepositoryError::from)?;
        Ok(())
    }

    fn update(&mut self, order: &Order) -> Result<(), RepositoryError> {
        let dto: OrderDto = order.into();

        update(orders.find(dto.id))
            .set(&dto)
            .execute(self.connection)
            .map_err(PostgresError::from)
            .map_err(RepositoryError::from)?;
        Ok(())
    }

    fn get_by_id(&mut self, order_id: OrderId) -> Result<Order, RepositoryError> {
        let order: OrderDto = orders
            .find(order_id.value())
            .first(self.connection)
            .map_err(PostgresError::from)
            .map_err(RepositoryError::from)?;

        order.try_into().map_err(RepositoryError::MapError)
    }

    fn get_any_new(&mut self) -> Result<Order, RepositoryError> {
        let row: OrderDto = orders
            .filter(status.eq(OrderStatus::Created.to_string()))
            .first(self.connection)
            .map_err(PostgresError::from)
            .map_err(RepositoryError::from)?;

        let result: Order = row.try_into().map_err(RepositoryError::MapError)?;

        Ok(result)
    }

    fn get_all_assigned(&mut self) -> Result<Vec<Order>, RepositoryError> {
        let s: String = OrderStatus::Assigned.into();

        let rows: Vec<OrderDto> = orders
            .filter(status.eq(s))
            .load(self.connection)
            .map_err(PostgresError::from)
            .map_err(RepositoryError::from)?;

        let result: Result<Vec<Order>, RepositoryError> = rows
            .into_iter()
            .map(|dto| dto.try_into().map_err(RepositoryError::MapError))
            .collect();

        result
    }
}
