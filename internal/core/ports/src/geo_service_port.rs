use domain::model::kernel::location::Location;

use crate::errors::GeoClientError;

#[trait_variant::make(HttpService: Send)]
pub trait GeoServicePort {
    async fn get_location(&mut self, address: String) -> Result<Location, GeoClientError>;
}
