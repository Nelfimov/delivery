use uuid::Uuid;

use crate::model::kernel::location::Location;

use super::order_aggregate::Order;
use super::order_aggregate::OrderId;

#[test]
fn should_create_order() {
    let location = Location::new(1, 1).unwrap();
    const VOLUME: u16 = 10;
    let order = Order::new(OrderId::new(Uuid::new_v4()), location, VOLUME).unwrap();

    assert_eq!(order.volume(), VOLUME)
}

#[test]
#[should_panic = "Zero"]
fn should_panic_on_nullish_volume() {
    let location = Location::new(1, 1).unwrap();
    const VOLUME: u16 = 0;
    let _ = Order::new(OrderId::new(Uuid::new_v4()), location, VOLUME).unwrap();
}

#[cfg(test)]
mod tests {
    use uuid::Uuid;

    use crate::model::courier::courier_aggregate::CourierId;
    use crate::model::kernel::location::Location;
    use crate::model::order::order_aggregate::OrderStatus;

    use super::super::order_aggregate::Order;
    use super::super::order_aggregate::OrderId;

    #[test]
    #[should_panic = "Exists"]
    fn should_assign_courier() {
        let location = Location::new(1, 1).unwrap();
        const VOLUME: u16 = 10;
        let mut order = Order::new(OrderId::new(Uuid::new_v4()), location, VOLUME).unwrap();

        let courier_id = CourierId(Uuid::new_v4());
        let _ = order.assign(&courier_id);

        assert_eq!(order.courier_id().unwrap(), courier_id);

        let courier_id_another = CourierId(Uuid::new_v4());
        order.assign(&courier_id_another).unwrap();
    }

    #[test]
    #[should_panic = "courier_id is not present"]
    fn should_panic_when_completing_unassigned() {
        let location = Location::new(1, 1).unwrap();
        const VOLUME: u16 = 10;
        let mut order = Order::new(OrderId::new(Uuid::new_v4()), location, VOLUME).unwrap();

        let result = order.complete();
        assert_eq!(result.unwrap(), ());
        assert_eq!(order.status(), &OrderStatus::Completed);

        order.complete().unwrap();
    }

    #[test]
    #[should_panic = "status is already set as Completed"]
    fn should_complete_order() {
        let location = Location::new(1, 1).unwrap();
        const VOLUME: u16 = 10;
        let mut order = Order::new(OrderId::new(Uuid::new_v4()), location, VOLUME).unwrap();

        let courier_id = CourierId(Uuid::new_v4());
        let _ = order.assign(&courier_id);

        let result = order.complete();
        assert_eq!(result.unwrap(), ());
        assert_eq!(order.status(), &OrderStatus::Completed);

        order.complete().unwrap();
    }
}
