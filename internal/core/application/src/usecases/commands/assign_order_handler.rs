use domain::model::services::order_dispatcher::OrderDispatcher;
use domain::model::services::order_dispatcher::OrderDispatcherService;
use ports::courier_repository_port::CourierRepositoryPort;
use ports::errors::RepositoryError;
use ports::order_repository_port::OrderRepositoryPort;
use ports::unit_of_work_port::UnitOfWorkPort;
use tracing::Level;
use tracing::instrument;

use crate::errors::command_errors::CommandError;
use crate::usecases::CommandHandler;
use crate::usecases::commands::assign_order_command::AssignOrderCommand;

#[derive(Debug)]
pub struct AssignOrderHandler<UOW>
where
    UOW: UnitOfWorkPort + std::fmt::Debug,
{
    uow: UOW,
}

impl<UOW> AssignOrderHandler<UOW>
where
    UOW: UnitOfWorkPort + std::fmt::Debug,
{
    pub fn new(uow: UOW) -> Self {
        Self { uow }
    }
}

impl<UOW> CommandHandler<AssignOrderCommand, ()> for AssignOrderHandler<UOW>
where
    UOW: UnitOfWorkPort + std::fmt::Debug,
{
    type Error = CommandError;

    #[instrument(skip(self))]
    async fn execute(&mut self, _: AssignOrderCommand) -> Result<(), Self::Error> {
        self.uow
            .transaction(|tx| {
                let unassigned_order = {
                    let mut repo = tx.order_repo();
                    repo.raw("SELECT * FROM orders WHERE status = 'created' LIMIT 1;".into())?
                        .pop()
                };

                match unassigned_order {
                    None => {
                        tracing::event!(Level::DEBUG, "no unassigned order found");
                        Ok(())
                    }
                    Some(mut order) => {
                        let span_child = tracing::span!(
                            tracing::Level::TRACE,
                            "handler",
                            id = order.id().0.to_string()
                        );
                        let _enter_child = span_child.enter();

                        let mut available_couriers = {
                            let mut repo = tx.courier_repo();
                            repo.get_all_free()?
                        };

                        let courier =
                            OrderDispatcherService::dispatch(&mut order, &mut available_couriers)
                                .map_err(|e| RepositoryError::from(e.to_string()))?;
                        tx.courier_repo().update(courier.to_owned())?;
                        tx.order_repo().update(&order)?;

                        tracing::event!(tracing::Level::INFO, "succesfully assigned order",);
                        Ok(())
                    }
                }
            })
            .map_err(CommandError::from)
    }
}
