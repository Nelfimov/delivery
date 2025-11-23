use domain::model::order::order_aggregate::Order;
use ports::events_producer_port::Events;
use ports::geo_service_port::GeoServicePort;
use ports::order_repository_port::OrderRepositoryPort;

use crate::errors::command_errors::CommandError;
use crate::usecases::CommandHandler;
use crate::usecases::commands::create_order_command::CreateOrderCommand;
use crate::usecases::events::event_bus::EventBus;

pub struct CreateOrderHandler<OR, GS, EB>
where
    OR: OrderRepositoryPort,
    GS: GeoServicePort,
    EB: EventBus,
{
    order_repository: OR,
    geo_service: GS,
    event_bus: EB,
}

impl<OR, GS, EB> CreateOrderHandler<OR, GS, EB>
where
    OR: OrderRepositoryPort,
    GS: GeoServicePort,
    EB: EventBus,
{
    pub fn new(order_repository: OR, geo_service: GS, event_bus: EB) -> Self {
        Self {
            order_repository,
            geo_service,
            event_bus,
        }
    }
}

impl<OR, GS, EB> CommandHandler<CreateOrderCommand, ()> for CreateOrderHandler<OR, GS, EB>
where
    OR: OrderRepositoryPort,
    GS: GeoServicePort,
    EB: EventBus,
{
    type Error = CommandError;

    async fn execute(&mut self, command: CreateOrderCommand) -> Result<(), Self::Error> {
        let location = self
            .geo_service
            .get_location(command.street())
            .await
            .map_err(|e| CommandError::ExecutionError(e.to_string()))?;
        let mut order = Order::new(command.order_id(), location, command.volume())
            .map_err(|e| CommandError::ExecutionError(e.to_string()))?;

        self.order_repository
            .add(&order)
            .map_err(|e| CommandError::ExecutionError(e.to_string()))?;

        let events: Vec<Events> = order.pop_domain_events().into_iter().map(Events::from).collect();
        for event in events {
            self.event_bus.commit(event).await?;
        }

        Ok(())
    }
}
