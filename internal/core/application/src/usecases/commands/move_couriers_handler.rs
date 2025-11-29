use ports::courier_repository_port::CourierRepositoryPort;
use ports::errors::RepositoryError;
use ports::events_producer_port::Events;
use ports::order_repository_port::OrderRepositoryPort;
use ports::unit_of_work_port::UnitOfWorkPort;
use std::fmt::Debug;
use tracing::debug;
use tracing::instrument;
use tracing::warn;

use crate::errors::command_errors::CommandError;
use crate::usecases::CommandHandler;
use crate::usecases::commands::move_couriers_command::MoveCouriersCommand;
use crate::usecases::events::event_bus::EventBus;

pub struct MoveCouriersHandler<UOW, EB>
where
    UOW: UnitOfWorkPort + Debug,
    EB: EventBus,
{
    uow: UOW,
    event_bus: EB,
}

impl<UOW, EB> MoveCouriersHandler<UOW, EB>
where
    UOW: UnitOfWorkPort + Debug,
    EB: EventBus,
{
    pub fn new(uow: UOW, event_bus: EB) -> Self {
        Self { uow, event_bus }
    }
}

impl<UOW, EB> CommandHandler<MoveCouriersCommand, ()> for MoveCouriersHandler<UOW, EB>
where
    UOW: UnitOfWorkPort + Debug,
    EB: EventBus,
{
    type Error = CommandError;

    #[instrument(skip_all)]
    async fn execute(&mut self, _c: MoveCouriersCommand) -> Result<(), Self::Error> {
        let events = self
            .uow
            .transaction(|tx| {
                let mut order_repo = tx.order_repo();
                let mut assigned_orders = order_repo.get_all_assigned()?;

                if assigned_orders.is_empty() {
                    debug!("no assigned orders found");
                    return Ok(Vec::<Events>::new());
                }

                let mut courier_repo = tx.courier_repo();
                let mut events = Vec::new();

                for order in &mut assigned_orders {
                    let courier_id = match order.courier_id() {
                        Some(courier_id) => *courier_id,
                        None => continue,
                    };

                    tracing::debug!(
                        "moving courier {} from order {}",
                        &order.id().0,
                        &courier_id.0
                    );

                    let order_events = (|| {
                        let mut courier = match courier_repo.get_by_id(courier_id) {
                            Ok(courier) => {
                                debug!("found courier {}", &courier_id.0);
                                courier
                            }
                            Err(RepositoryError::NotFound(_)) => {
                                warn!("courier by id {} not found", &courier_id.0);
                                return Ok(Vec::<Events>::new());
                            }
                            Err(err) => return Err(err),
                        };

                        courier
                            .move_to_location(order.location())
                            .map_err(|err| RepositoryError::from(err.to_string()))?;

                        if courier.location() == order.location() {
                            debug!(
                                "courier {} is at order {} location, completing the order",
                                &courier_id.0,
                                &order.id().0
                            );
                            order
                                .complete()
                                .map_err(|err| RepositoryError::from(err.to_string()))?;
                            courier.complete_order(order.id());
                        }

                        courier_repo.update(courier)?;
                        order_repo.update(order)?;

                        Ok(order
                            .pop_domain_events()
                            .into_iter()
                            .map(Events::from)
                            .collect::<Vec<Events>>())
                    })();

                    match order_events {
                        Ok(order_events) => events.extend(order_events),
                        Err(err) => {
                            warn!(
                                error = ?err,
                                "failed to process order {}, continuing",
                                &order.id().0
                            );
                        }
                    }
                }
                debug!("finished moving courier and adjusting order");
                Ok(events)
            })
            .map_err(CommandError::from)?;

        for event in events {
            self.event_bus.commit(event).await?;
        }

        Ok(())
    }
}
