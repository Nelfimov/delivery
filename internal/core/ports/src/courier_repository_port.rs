use domain::model::courier::courier_aggregate::Courier;
use domain::model::courier::courier_aggregate::CourierId;
use domain::model::courier::courier_aggregate::CourierName;
use domain::model::kernel::location::Location;

use crate::errors::RepositoryError;

pub struct GetAllCouriersResponse {
    pub id: CourierId,
    pub name: CourierName,
    pub location: Location,
}

pub trait CourierRepositoryPort {
    fn add(&mut self, courier: Courier) -> Result<(), RepositoryError>;
    fn update(&mut self, courier: Courier) -> Result<(), RepositoryError>;
    fn get_by_id(&mut self, id: CourierId) -> Result<Courier, RepositoryError>;
    fn get_all_free(&mut self) -> Result<Vec<Courier>, RepositoryError>;
    fn get_all_couriers(&mut self) -> Result<Vec<GetAllCouriersResponse>, RepositoryError>;
}
