use ports::courier_repository_port::CourierRepositoryPort;
use ports::errors::RepositoryError;
use ports::order_repository_port::OrderRepositoryPort;
use ports::unit_of_work_port::UnitOfWorkPort;

use crate::errors::command_errors::CommandError;
use crate::usecases::CommandHandler;
use crate::usecases::commands::move_couriers_command::MoveCouriersCommand;

pub struct MoveCouriersHandler<UOW>
where
    UOW: UnitOfWorkPort,
{
    uow: UOW,
}

impl<UOW> MoveCouriersHandler<UOW>
where
    UOW: UnitOfWorkPort,
{
    pub fn new(uow: UOW) -> Self {
        Self { uow }
    }
}

impl<UOW> CommandHandler<MoveCouriersCommand, ()> for MoveCouriersHandler<UOW>
where
    UOW: UnitOfWorkPort,
{
    type Error = CommandError;

    fn execute(&mut self, _command: MoveCouriersCommand) -> Result<(), Self::Error> {
        self.uow
            .transaction(|tx| {
                let mut assigned_orders = {
                    let mut order_repo = tx.order_repo();
                    order_repo.get_all_assigned()?
                };

                for order in &mut assigned_orders {
                    let courier_id = match order.courier_id() {
                        Some(courier_id) => *courier_id,
                        None => continue,
                    };

                    let mut courier = {
                        let mut courier_repo = tx.courier_repo();
                        match courier_repo.get_by_id(courier_id) {
                            Ok(courier) => courier,
                            Err(RepositoryError::NotFound(_)) => continue,
                            Err(err) => return Err(err),
                        }
                    };

                    courier
                        .move_to_location(order.location())
                        .map_err(|err| RepositoryError::from(err.to_string()))?;

                    if courier.location() == order.location() {
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
                Ok(())
            })
            .map_err(|e| CommandError::ExecutionError(e.to_string()))?;

        Ok(())
    }
}
