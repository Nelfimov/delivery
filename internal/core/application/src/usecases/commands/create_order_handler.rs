use domain::model::kernel::location::Location;
use domain::model::order::order_aggregate::Order;
use ports::order_repository_port::OrderRepositoryPort;

use crate::errors::command_errors::CommandError;
use crate::usecases::CommandHandler;
use crate::usecases::commands::create_order_command::CreateOrderCommand;

pub struct CreateOrderHandler<OR>
where
    OR: OrderRepositoryPort,
{
    order_repository: OR,
}

impl<OR> CreateOrderHandler<OR>
where
    OR: OrderRepositoryPort,
{
    pub fn new(order_repository: OR) -> Self {
        Self { order_repository }
    }
}

impl<OR> CommandHandler<CreateOrderCommand, ()> for CreateOrderHandler<OR>
where
    OR: OrderRepositoryPort,
{
    type Error = CommandError;

    fn execute(&mut self, command: CreateOrderCommand) -> Result<(), Self::Error> {
        // TODO: switch to geolocation when ready
        let location = Location::new_random();
        let order = Order::new(command.order_id(), location, command.volume())
            .map_err(|e| CommandError::ExecutionError(e.to_string()))?;

        self.order_repository
            .add(&order)
            .map_err(|e| CommandError::ExecutionError(e.to_string()))?;

        Ok(())
    }
}
