use uuid::Uuid;

use crate::model::kernel::volume::Volume;
use crate::model::order::order_aggregate::OrderId;

use super::storage_place::StoragePlace;

#[test]
#[should_panic]
fn should_create() {
    StoragePlace::new(
        String::from(""),
        Volume::new(50).unwrap(),
        Some(OrderId::new(Uuid::new_v4())),
    )
    .unwrap();
}

#[test]
fn should_not_equal() {
    let a = StoragePlace::new(
        String::from("backpack"),
        Volume::new(50).unwrap(),
        Some(OrderId::new(Uuid::new_v4())),
    )
    .unwrap();
    let b = StoragePlace::new(String::from("bag"), Volume::new(32).unwrap(), None).unwrap();

    assert!(a != b);
    assert!(a == a);
}

#[test]
fn should_check_if_can_place_order() {
    let storage_volume = Volume::new(50).unwrap();
    let low_volume = Volume::new(30).unwrap();
    let high_volume = Volume::new(60).unwrap();
    let order_id = Uuid::new_v4();

    let without_order = StoragePlace::new(String::from("backpack"), storage_volume, None).unwrap();

    assert!(without_order.can_place_order(&low_volume));
    assert!(!without_order.can_place_order(&high_volume));

    let with_order = StoragePlace::new(
        String::from("backpack"),
        storage_volume,
        Some(OrderId::new(order_id)),
    )
    .unwrap();

    assert!(!with_order.can_place_order(&low_volume));
    assert!(!with_order.can_place_order(&high_volume));
}

#[test]
fn should_place_order() {
    let storage_volume = Volume::new(50).unwrap();
    let low_volume = Volume::new(30).unwrap();
    let high_volume = Volume::new(60).unwrap();
    let order_id = Some(OrderId::new(Uuid::new_v4()));

    let mut storage = StoragePlace::new(String::from("backpack"), storage_volume, None).unwrap();

    let _ = storage.place_order(order_id.unwrap(), high_volume);
    assert_ne!(storage.order_id(), &order_id);

    let _ = storage.place_order(order_id.unwrap(), low_volume);
    assert_eq!(storage.order_id(), &order_id);
}

#[test]
fn should_remove_order() {
    let storage_volume = Volume::new(50).unwrap();
    let order_id = Some(OrderId::new(Uuid::new_v4()));

    let mut storage =
        StoragePlace::new(String::from("backpack"), storage_volume, order_id).unwrap();

    storage.remove_order();
    assert_eq!(storage.order_id(), &None);
}
