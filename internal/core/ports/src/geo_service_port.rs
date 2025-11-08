use async_trait::async_trait;
use domain::model::kernel::location::Location;

use crate::errors::GeoClientError;

#[async_trait]
pub trait GeoServicePort: Send + Sync {
    async fn get_location(&mut self, address: String) -> Result<Location, GeoClientError>;
}
