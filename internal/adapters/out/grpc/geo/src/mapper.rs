use domain::errors::domain_model_errors::DomainModelError;
use domain::model::kernel::location::Location as DomainLocation;

use crate::api::Location;

impl TryFrom<Location> for DomainLocation {
    type Error = DomainModelError;

    fn try_from(v: Location) -> Result<Self, Self::Error> {
        Self::new(v.x as u8, v.y as u8)
    }
}
