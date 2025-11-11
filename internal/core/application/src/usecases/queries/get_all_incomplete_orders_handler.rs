use domain::model::order::order_aggregate::Order;
use ports::order_repository_port::OrderRepositoryPort;

use crate::errors::query_errors::QueryError;
use crate::usecases::CommandHandler;
use crate::usecases::queries::get_all_incomplete_orders_query::GetAllIncompleteOrders;

pub struct GetAllIncompleteOrdersHandler<OR>
where
    OR: OrderRepositoryPort,
{
    order_repository: OR,
}

impl<OR> GetAllIncompleteOrdersHandler<OR>
where
    OR: OrderRepositoryPort,
{
    pub fn new(order_repository: OR) -> Self {
        Self { order_repository }
    }
}

impl<OR> CommandHandler<GetAllIncompleteOrders, Vec<Order>> for GetAllIncompleteOrdersHandler<OR>
where
    OR: OrderRepositoryPort,
{
    type Error = QueryError;

    async fn execute(
        &mut self,
        _command: GetAllIncompleteOrders,
    ) -> Result<Vec<Order>, Self::Error> {
        self.order_repository
            .raw("SELECT * FROM orders WHERE status != 'completed';".into())
            .map_err(Self::Error::from)
    }
}
