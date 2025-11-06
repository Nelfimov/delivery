use domain::model::services::order_dispatcher::OrderDispatcher;
use domain::model::services::order_dispatcher::OrderDispatcherService;
use ports::courier_repository_port::CourierRepositoryPort;
use ports::errors::RepositoryError;
use ports::order_repository_port::OrderRepositoryPort;
use ports::unit_of_work_port::UnitOfWorkPort;

use crate::errors::command_errors::CommandError;
use crate::usecases::CommandHandler;
use crate::usecases::commands::assign_order_command::AssignOrderCommand;

pub struct AssignOrderHandler<UOW>
where
    UOW: UnitOfWorkPort,
{
    uow: UOW,
}

impl<UOW> AssignOrderHandler<UOW>
where
    UOW: UnitOfWorkPort,
{
    pub fn new(uow: UOW) -> Self {
        Self { uow }
    }
}

impl<UOW> CommandHandler<AssignOrderCommand, ()> for AssignOrderHandler<UOW>
where
    UOW: UnitOfWorkPort,
{
    type Error = CommandError;

    fn execute(&mut self, _: AssignOrderCommand) -> Result<(), Self::Error> {
        self.uow
            .transaction(|tx| {
                let unassigned_order = {
                    let mut repo = tx.order_repo();
                    repo.raw("SELECT * FROM orders WHERE status = 'created' LIMIT 1;".into())?
                        .pop()
                };

                match unassigned_order {
                    None => {
                        println!("No unassigned order found");
                        Ok(())
                    }
                    Some(mut order) => {
                        let mut available_couriers = {
                            let mut repo = tx.courier_repo();
                            repo.get_all_free()?
                        };

                        let courier =
                            OrderDispatcherService::dispatch(&mut order, &mut available_couriers)
                                .map_err(|e| RepositoryError::from(e.to_string()))?;
                        tx.courier_repo().update(courier.to_owned())?;
                        tx.order_repo().update(&order)?;
                        Ok(())
                    }
                }
            })
            .map_err(CommandError::from)
    }
}
