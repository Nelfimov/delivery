use domain::model::order::order_aggregate::Order;
use domain::model::order::order_aggregate::OrderId;

use crate::errors::RepositoryError;

pub trait OrderRepositoryPort {
    fn add(&mut self, order: &Order) -> Result<(), RepositoryError>;
    fn update(&mut self, order: &Order) -> Result<(), RepositoryError>;
    fn get_by_id(&mut self, id: OrderId) -> Result<Order, RepositoryError>;
    fn get_any_new(&mut self) -> Result<Order, RepositoryError>;
    fn get_all_assigned(&mut self) -> Result<Vec<Order>, RepositoryError>;
}
