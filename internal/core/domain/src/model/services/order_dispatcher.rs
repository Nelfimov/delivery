use std::cmp::Reverse;

use crate::errors::domain_model_errors::DomainModelError;
use crate::model::courier::courier_aggregate::Courier;
use crate::model::kernel::volume::Volume;
use crate::model::order::order_aggregate::Order;
use crate::model::order::order_aggregate::OrderStatus;

pub trait OrderDispatcher {
    fn dispatch(
        order: &mut Order,
        couriers: &mut Vec<Courier>,
    ) -> Result<Courier, DomainModelError>;
}

pub struct OrderDispatcherService;

impl OrderDispatcher for OrderDispatcherService {
    fn dispatch(
        order: &mut Order,
        couriers: &mut Vec<Courier>,
    ) -> Result<Courier, DomainModelError> {
        if order.status() != &OrderStatus::Created {
            return Err(DomainModelError::UnmetRequirement(
                "status it not 'created'".into(),
            ));
        }

        let order_volume = Volume::new(order.volume())
            .map_err(|_| DomainModelError::UnmetRequirement("invalid order volume".into()))?;

        let mut available = Vec::new();
        let mut i = 0;

        while i < couriers.len() {
            if couriers[i].can_take_order(&order_volume).is_some() {
                let c = couriers.remove(i);
                available.push(c);
            } else {
                i += 1;
            }
        }

        available.sort_by_key(|c| Reverse(c.get_traverse_length(order.location())));

        let mut courier = available.pop().ok_or(DomainModelError::UnmetRequirement(
            "no courier found".into(),
        ))?;

        courier.take_order(order.id(), order_volume);
        match order.assign(courier.id()) {
            Ok(_) => {}
            Err(e) => return Err(e),
        }
        Ok(courier)
    }
}
