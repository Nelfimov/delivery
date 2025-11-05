use domain::model::courier::courier_aggregate::Courier;
use domain::model::kernel::location::Location;
use ports::courier_repository_port::CourierRepositoryPort;

use crate::errors::command_errors::CommandError;
use crate::usecases::CommandHandler;
use crate::usecases::commands::create_courier_command::CreateCourierCommand;

pub struct CreateCourierHandler<CR>
where
    CR: CourierRepositoryPort,
{
    courier_repository: CR,
}

impl<CR> CreateCourierHandler<CR>
where
    CR: CourierRepositoryPort,
{
    pub fn new(courier_repository: CR) -> Self {
        Self { courier_repository }
    }
}

impl<CR> CommandHandler<CreateCourierCommand, ()> for CreateCourierHandler<CR>
where
    CR: CourierRepositoryPort,
{
    type Error = CommandError;

    fn execute(&mut self, command: CreateCourierCommand) -> Result<(), Self::Error> {
        let courier = Courier::new(
            command.name().to_owned(),
            command.speed().to_owned(),
            Location::new_random(),
        )?;
        self.courier_repository
            .add(courier)
            .map_err(|e| CommandError::ExecutionError(e.to_string()))?;

        Ok(())
    }
}
