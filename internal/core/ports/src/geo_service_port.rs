use domain::model::kernel::location::Location;

pub trait GeoServicePort {
    fn get_location(&self, address: String) -> Result<Location, String>;
}
