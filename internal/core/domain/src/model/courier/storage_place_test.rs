use super::storage_place::StoragePlace;
use uuid::Uuid;

#[test]
#[should_panic]
fn should_create() {
    StoragePlace::new(String::from(""), 50, Some(Uuid::new_v4())).unwrap();
}

#[test]
fn should_not_equal() {
    let a = StoragePlace::new(String::from("backpack"), 50, Some(Uuid::new_v4())).unwrap();
    let b = StoragePlace::new(String::from("bag"), 32, None).unwrap();

    assert!(a != b);
    assert!(a == a);
}

#[test]
fn should_check_if_can_place_order() {
    const STORAGE_VOLUME: u16 = 50;
    const LOW_VOLUME: u16 = 30;
    const HIGH_VOLUME: u16 = 60;
    let order_id = Uuid::new_v4();

    let without_order = StoragePlace::new(String::from("backpack"), STORAGE_VOLUME, None).unwrap();

    assert!(without_order.can_place_order(Uuid::new_v4(), LOW_VOLUME));
    assert!(!without_order.can_place_order(Uuid::new_v4(), HIGH_VOLUME));

    let with_order =
        StoragePlace::new(String::from("backpack"), STORAGE_VOLUME, Some(order_id)).unwrap();

    assert!(!with_order.can_place_order(Uuid::new_v4(), LOW_VOLUME));
    assert!(!with_order.can_place_order(Uuid::new_v4(), HIGH_VOLUME));
}

#[test]
fn should_place_order() {
    const STORAGE_VOLUME: u16 = 50;
    const LOW_VOLUME: u16 = 30;
    const HIGH_VOLUME: u16 = 60;
    let order_id = Some(Uuid::new_v4());

    let mut storage = StoragePlace::new(String::from("backpack"), STORAGE_VOLUME, None).unwrap();

    let _ = storage.place_order(order_id.unwrap(), HIGH_VOLUME);
    assert_ne!(storage.order_id(), &order_id);

    let _ = storage.place_order(order_id.unwrap(), LOW_VOLUME);
    assert_eq!(storage.order_id(), &order_id);
}

#[test]
fn should_remove_order() {
    const STORAGE_VOLUME: u16 = 50;
    let order_id = Some(Uuid::new_v4());

    let mut storage =
        StoragePlace::new(String::from("backpack"), STORAGE_VOLUME, order_id).unwrap();

    storage.remove_order();
    assert_eq!(storage.order_id(), &None);
}
