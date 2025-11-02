use domain::model::courier::courier_aggregate::CourierId;
use domain::model::kernel::location::Location;
use domain::model::kernel::volume::Volume;
use domain::model::order::order_aggregate::Order;
use domain::model::order::order_aggregate::OrderId;
use domain::model::order::order_aggregate::OrderStatus;

use crate::order::order_dto::OrderDto;

impl From<&Order> for OrderDto {
    fn from(order: &Order) -> Self {
        Self {
            id: order.id().value(),
            courier_id: order.courier_id().map(|c| c.0),
            location_x: order.location().x() as i16,
            location_y: order.location().y() as i16,
            volume: order.volume() as i16,
            status: order.status().into(),
        }
    }
}

impl TryFrom<OrderDto> for Order {
    type Error = String;

    fn try_from(row: OrderDto) -> Result<Self, Self::Error> {
        let status = match row.status.as_str() {
            "created" => OrderStatus::Created,
            "assigned" => OrderStatus::Assigned,
            "completed" => OrderStatus::Completed,
            _ => return Err("invalid status".into()),
        };

        let id = OrderId::new(row.id);
        let volume = Volume::new(row.volume as u16)?;
        let location = Location::new(row.location_x as u8, row.location_y as u8)?;
        let courier_id = row.courier_id.map(CourierId);

        Ok(Order::restore(id, courier_id, location, volume, status))
    }
}
