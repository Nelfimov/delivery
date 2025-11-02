use uuid::Uuid;

use domain::model::kernel::volume::Volume;
use domain::model::order::order_aggregate::OrderId;

use crate::errors::command_errors::CommandError;

pub struct CreateOrderCommand {
    order_id: OrderId,
    street: String,
    volume: Volume,
}

impl CreateOrderCommand {
    pub fn new(order_id: Uuid, street: String, volume: u16) -> Result<Self, CommandError> {
        if street.is_empty() {
            return Err(CommandError::ArgumentError(format!(
                "Found empty street: {}",
                street
            )));
        }
        let volume = Volume::new(volume).map_err(CommandError::from)?;
        let order_id = OrderId::new(order_id);

        Ok(Self {
            order_id,
            volume,
            street,
        })
    }

    pub fn order_id(&self) -> OrderId {
        self.order_id
    }

    pub fn street(&self) -> String {
        self.street.to_owned()
    }

    pub fn volume(&self) -> Volume {
        self.volume
    }
}
