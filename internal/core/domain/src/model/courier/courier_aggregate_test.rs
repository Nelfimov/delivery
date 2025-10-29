use uuid::Uuid;

use crate::model::courier::courier_aggregate::Courier;
use crate::model::courier::courier_aggregate::CourierName;
use crate::model::courier::courier_aggregate::CourierSpeed;
use crate::model::kernel::location::Location;
use crate::model::kernel::volume::Volume;
use crate::model::order::order_aggregate::OrderId;

fn make_courier_at(x: u8, y: u8) -> Courier {
    Courier::new(
        CourierName("Bob".to_string()),
        CourierSpeed(5),
        Location::new(x, y).unwrap(),
    )
    .unwrap()
}

#[test]
fn creates_courier_with_default_storage() {
    let courier = make_courier_at(1, 1);
    assert_eq!(courier.name(), "Bob");
    assert_eq!(*courier.speed(), 5);
    assert_eq!(courier.storage_places().len(), 1);
    assert_eq!(courier.location().x(), 1);
    assert_eq!(courier.location().y(), 1);
}

#[test]
fn adds_storage_place() {
    let mut courier = make_courier_at(1, 1);
    courier
        .add_storage_place("extra_bag".to_string(), Volume::new(25).unwrap())
        .unwrap();
    assert_eq!(courier.storage_places().len(), 2);
    assert!(
        courier
            .storage_places()
            .iter()
            .any(|sp| sp.name() == "extra_bag")
    );
}

#[test]
fn takes_and_completes_order() {
    let mut courier = make_courier_at(1, 1);
    let order_id = OrderId::new(Uuid::new_v4());
    let volume = Volume::new(5).unwrap();

    assert!(courier.can_take_order(&volume).is_some());

    let _ = courier.take_order(order_id, volume);
    assert!(
        courier
            .storage_places()
            .iter()
            .any(|sp| sp.order_id() == &Some(order_id))
    );

    courier.complete_order(order_id);
    assert!(
        courier
            .storage_places()
            .iter()
            .all(|sp| sp.order_id().is_none())
    );
}

#[test]
fn get_traverse_length_ceil_correctly() {
    let courier = make_courier_at(1, 1);
    let destination = Location::new(9, 1).unwrap();
    let t = courier.get_traverse_length(&destination);
    assert_eq!(t, 2);
}

#[test]
fn move_to_location_moves_by_speed() {
    let mut courier = make_courier_at(1, 1);
    let destination = Location::new(8, 1).unwrap();

    let loc = courier.move_to_location(&destination).unwrap();
    assert_eq!(loc.x(), 6);
    assert_eq!(loc.y(), 1);

    let loc = courier.move_to_location(&destination).unwrap();
    assert_eq!(loc.x(), 8);
    assert_eq!(loc.y(), 1);

    let loc = courier.move_to_location(&destination).unwrap();
    assert_eq!(loc.x(), 8);
    assert_eq!(loc.y(), 1);
}

#[test]
fn move_to_location_works_on_y_axis() {
    let mut courier = make_courier_at(1, 1);
    let destination = Location::new(1, 8).unwrap();
    let loc = courier.move_to_location(&destination).unwrap();
    assert_eq!(loc.x(), 1);
    assert_eq!(loc.y(), 6);
}
