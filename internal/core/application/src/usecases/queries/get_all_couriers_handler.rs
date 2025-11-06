use ports::courier_repository_port::CourierRepositoryPort;
use ports::courier_repository_port::GetAllCouriersResponse;

use crate::errors::query_errors::QueryError;
use crate::usecases::CommandHandler;
use crate::usecases::queries::get_all_couriers_query::GetAllCouriers;

pub struct GetAllCouriersHandler<CR>
where
    CR: CourierRepositoryPort,
{
    courier_repository: CR,
}

impl<CR> GetAllCouriersHandler<CR>
where
    CR: CourierRepositoryPort,
{
    pub fn new(courier_repository: CR) -> Self {
        Self { courier_repository }
    }
}

impl<CR> CommandHandler<GetAllCouriers, Vec<GetAllCouriersResponse>> for GetAllCouriersHandler<CR>
where
    CR: CourierRepositoryPort,
{
    type Error = QueryError;

    fn execute(&mut self, _: GetAllCouriers) -> Result<Vec<GetAllCouriersResponse>, Self::Error> {
        Ok(self.courier_repository.get_all_couriers()?)
    }
}
