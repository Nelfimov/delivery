use diesel::PgConnection;
use diesel::dsl::insert_into;
use diesel::dsl::update;
use diesel::prelude::*;
use diesel::result::Error;
use domain::model::order::order_aggregate::Order;
use domain::model::order::order_aggregate::OrderId;
use domain::model::order::order_aggregate::OrderStatus;

use super::order_dto::OrderDto;
use super::order_schema::orders::dsl::*;

pub struct OrderRepository {
    pub connection: PgConnection,
}

impl OrderRepository {
    pub fn add(&mut self, order: &Order) -> Result<(), Error> {
        insert_into(orders::table)
            .values(order.try_into())
            .load(self.connection)?
    }

    pub fn update(order: &Order) -> Result<(), Error> {
        let dto: OrderDto = order.into();
        update(orders.find(dto.id)).set(&dto).execute(connection)?;
        Ok(())
    }

    pub fn get_by_id(&mut self, order_id: OrderId) -> Result<Order, Error> {
        let order: OrderDto = orders.find(order_id.value()).first(&mut self.connection)?;

        Ok(order
            .try_into()
            .map_err(|e| Error::DeserializationError(Box::new(e)))?)
    }

    pub fn get_any_new(&mut self) {
        let row: OrderDto = orders
            .filter(status.eq(OrderStatus::Created.into()))
            .limit(1)
            .load(&mut self.connection)?;

        let result: OrderDto = row
            .try_into()
            .map_err(|e| Error::DeserializationError(Box::new(e)))?;

        Ok(result)
    }

    pub fn get_all_assigned(&mut self) -> Result<Vec<Order>, Error> {
        let rows: Vec<OrderDto> = orders
            .filter(status.eq(OrderStatus::Assigned.into()))
            .load(&mut self.connection)?;

        let result: Result<Vec<Order>, Error> = rows
            .into_iter()
            .map(|dto| {
                dto.try_into()
                    .map_err(|e| Error::DeserializationError(Box::new(e)))
            })
            .collect();

        result
    }
}
