use domain::model::order::order_aggregate::OrderId;

use crate::order::order_dto::OrderDto;

pub struct OrderRepository {}

impl OrderRepository {
    pub fn add(order: &OrderDto) {}

    pub fn update(order: &OrderDto) {}

    pub fn get_by_id(id: OrderId) {}

    pub fn get_any_new() {}

    pub fn get_all_assigned() {}
}
