use crate::errors::domain_model_errors::DomainModelError;
use crate::model::courier::courier_aggregate::Courier;
use crate::model::kernel::volume::Volume;
use crate::model::order::order_aggregate::Order;
use crate::model::order::order_aggregate::OrderStatus;

pub trait OrderDispatcher {
    fn dispatch<'a>(
        order: &mut Order,
        couriers: &'a mut Vec<Courier>,
    ) -> Result<&'a mut Courier, DomainModelError>;
}

pub struct OrderDispatcherService;

impl OrderDispatcher for OrderDispatcherService {
    fn dispatch<'a>(
        order: &mut Order,
        couriers: &'a mut Vec<Courier>,
    ) -> Result<&'a mut Courier, DomainModelError> {
        if order.status() != &OrderStatus::Created {
            return Err(DomainModelError::UnmetRequirement(
                "status it not 'created'".into(),
            ));
        }

        let order_volume = Volume::new(order.volume())
            .map_err(|_| DomainModelError::UnmetRequirement("invalid order volume".into()))?;

        let (idx, _) = couriers
            .iter_mut()
            .enumerate()
            .filter(|(_, c)| c.can_take_order(&order_volume).is_some())
            .min_by_key(|(_, c)| c.get_traverse_length(order.location()))
            .ok_or(DomainModelError::UnmetRequirement(
                "no courier found".into(),
            ))?;

        let courier = &mut couriers[idx];

        courier.take_order(order.id(), order_volume)?;
        order.assign(courier.id())?;

        Ok(courier)
    }
}
