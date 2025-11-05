use domain::model::courier::courier_aggregate::CourierName;
use domain::model::courier::courier_aggregate::CourierSpeed;

use crate::errors::command_errors::CommandError;

pub struct CreateCourierCommand {
    name: CourierName,
    speed: CourierSpeed,
}

impl CreateCourierCommand {
    pub fn new(name: CourierName, speed: CourierSpeed) -> Result<Self, CommandError> {
        Ok(Self { name, speed })
    }

    pub fn name(&self) -> &CourierName {
        &self.name
    }

    pub fn speed(&self) -> &CourierSpeed {
        &self.speed
    }
}
