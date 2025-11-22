use ports::courier_repository_port::CourierRepositoryPort;
use ports::errors::RepositoryError;
use ports::order_repository_port::OrderRepositoryPort;
use ports::unit_of_work_port::UnitOfWorkPort;
use std::fmt::Debug;
use tracing::debug;
use tracing::instrument;
use tracing::warn;

use crate::errors::command_errors::CommandError;
use crate::usecases::CommandHandler;
use crate::usecases::commands::move_couriers_command::MoveCouriersCommand;

pub struct MoveCouriersHandler<UOW>
where
    UOW: UnitOfWorkPort + Debug,
{
    uow: UOW,
}

impl<UOW> MoveCouriersHandler<UOW>
where
    UOW: UnitOfWorkPort + Debug,
{
    pub fn new(uow: UOW) -> Self {
        Self { uow }
    }
}

impl<UOW> CommandHandler<MoveCouriersCommand, ()> for MoveCouriersHandler<UOW>
where
    UOW: UnitOfWorkPort + Debug,
{
    type Error = CommandError;

    #[instrument(skip_all)]
    async fn execute(&mut self, _command: MoveCouriersCommand) -> Result<(), Self::Error> {
        self.uow
            .transaction(|tx| {
                let mut assigned_orders = {
                    let mut order_repo = tx.order_repo();
                    order_repo.get_all_assigned()?
                };

                if assigned_orders.is_empty() {
                    debug!("no assigned orders found");
                    return Ok(());
                }

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

                    let mut courier = {
                        let mut courier_repo = tx.courier_repo();
                        match courier_repo.get_by_id(courier_id) {
                            Ok(courier) => {
                                debug!("found courier {}", &courier_id.0);
                                courier
                            }
                            Err(RepositoryError::NotFound(_)) => {
                                warn!("courier by id {} not found", &courier_id.0);
                                continue;
                            }
                            Err(err) => return Err(err),
                        }
                    };

                    courier
                        .move_to_location(order.location())
                        .map_err(|err| RepositoryError::from(err.to_string()))?;

                    if courier.location() == order.location() {
                        debug!(
                            "courier {} is at order {} loaction, completing the order",
                            &courier_id.0,
                            &order.id().0
                        );
                        order
                            .complete()
                            .map_err(|err| RepositoryError::from(err.to_string()))?;
                    }

                    {
                        let mut courier_repo = tx.courier_repo();
                        courier_repo.update(courier)?;
                    }

                    {
                        let mut order_repo = tx.order_repo();
                        order_repo.update(order)?;
                    }
                }
                debug!("finished moving courier and adjusting order");
                Ok(())
            })
            .map_err(|e| CommandError::ExecutionError(e.to_string()))?;

        Ok(())
    }
}
