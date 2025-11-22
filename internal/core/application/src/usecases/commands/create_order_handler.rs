use domain::model::order::order_aggregate::Order;
use ports::geo_service_port::GeoServicePort;
use ports::order_repository_port::OrderRepositoryPort;

use crate::errors::command_errors::CommandError;
use crate::usecases::CommandHandler;
use crate::usecases::commands::create_order_command::CreateOrderCommand;

pub struct CreateOrderHandler<OR, GS>
where
    OR: OrderRepositoryPort,
    GS: GeoServicePort,
{
    order_repository: OR,
    geo_service: GS,
}

impl<OR, GS> CreateOrderHandler<OR, GS>
where
    OR: OrderRepositoryPort,
    GS: GeoServicePort,
{
    pub fn new(order_repository: OR, geo_service: GS) -> Self {
        Self {
            order_repository,
            geo_service,
        }
    }
}

impl<OR, GS> CommandHandler<CreateOrderCommand, ()> for CreateOrderHandler<OR, GS>
where
    OR: OrderRepositoryPort,
    GS: GeoServicePort,
{
    type Error = CommandError;

    async fn execute(&mut self, command: CreateOrderCommand) -> Result<(), Self::Error> {
        let location = self
            .geo_service
            .get_location(command.street())
            .await
            .map_err(|e| CommandError::ExecutionError(e.to_string()))?;
        let order = Order::new(command.order_id(), location, command.volume())
            .map_err(|e| CommandError::ExecutionError(e.to_string()))?;

        self.order_repository
            .add(&order)
            .map_err(|e| CommandError::ExecutionError(e.to_string()))?;

        Ok(())
    }
}
