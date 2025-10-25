use uuid::Uuid;

use crate::errors::domain_model_errors::DomainModelError;
use crate::model::courier::courier_aggregate::Courier;
use crate::model::courier::courier_aggregate::CourierId;
use crate::model::courier::courier_aggregate::CourierName;
use crate::model::courier::courier_aggregate::CourierSpeed;
use crate::model::kernel::location::Location;
use crate::model::kernel::volume::Volume;
use crate::model::order::order_aggregate::Order;
use crate::model::order::order_aggregate::OrderId;
use crate::model::services::order_dispatcher::OrderDispatcher;
use crate::model::services::order_dispatcher::OrderDispatcherService;

#[test]
fn fails_when_order_not_created() {
    let mut order = Order::new(
        OrderId::new(Uuid::new_v4()),
        Location::new(1, 1).unwrap(),
        10,
    )
    .unwrap();
    let mut couriers = vec![
        Courier::new(
            CourierName("Bob".into()),
            CourierSpeed(5),
            Location::new(9, 9).unwrap(),
        )
        .unwrap(),
    ];

    let _ = order.assign(&CourierId(Uuid::new_v4()));
    let _ = order.complete();

    let result = OrderDispatcherService::dispatch(&order, &mut couriers);
    assert!(matches!(result, Err(DomainModelError::UnmetRequirement(_))));
}

#[test]
fn fails_when_no_available_couriers() {
    let order = Order::new(
        OrderId::new(Uuid::new_v4()),
        Location::new(1, 1).unwrap(),
        10,
    )
    .unwrap();

    let mut courier_bob = Courier::new(
        CourierName("Bob".into()),
        CourierSpeed(5),
        Location::new(9, 9).unwrap(),
    )
    .unwrap();
    courier_bob.take_order(order.id(), Volume::new(2).unwrap());

    let mut courier_rick = Courier::new(
        CourierName("Rick".into()),
        CourierSpeed(9),
        Location::new(5, 5).unwrap(),
    )
    .unwrap();
    courier_rick.take_order(order.id(), Volume::new(2).unwrap());

    let mut couriers = vec![];

    let result = OrderDispatcherService::dispatch(&order, &mut couriers);
    assert!(matches!(result, Err(DomainModelError::UnmetRequirement(_))));
}

#[test]
fn picks_fastest_available_courier() {
    let order = Order::new(
        OrderId::new(Uuid::new_v4()),
        Location::new(1, 1).unwrap(),
        10,
    )
    .unwrap();
    let courier_bob = Courier::new(
        CourierName("Bob".into()),
        CourierSpeed(1),
        Location::new(9, 9).unwrap(),
    )
    .unwrap();
    let courier_rick = Courier::new(
        CourierName("Rick".into()),
        CourierSpeed(2),
        Location::new(9, 9).unwrap(),
    )
    .unwrap();
    let courier_zack = Courier::new(
        CourierName("Zack".into()),
        CourierSpeed(3),
        Location::new(9, 9).unwrap(),
    )
    .unwrap();
    let mut couriers = vec![courier_bob, courier_rick, courier_zack];

    let result = OrderDispatcherService::dispatch(&order, &mut couriers).unwrap();
    assert_eq!(result.name(), "Zack");
}
