use super::location::Location;

#[test]
fn gets_distance() {
    let a = Location::new(5, 5).unwrap();
    let b = Location::new(1, 2).unwrap();

    assert_eq!(a.get_distance(&b), 7);
    assert_eq!(b.get_distance(&a), 7);
}

#[test]
fn creates_random() {
    let a = Location::new_random();

    assert!(a.x() >= 1 && a.x() <= 10);
    assert!(a.y() >= 1 && a.y() <= 10);
}

#[test]
fn compares() {
    let a = Location::new(1, 1).unwrap();
    let b = Location::new(2, 2).unwrap();
    let c = Location::new(1, 1).unwrap();

    assert_ne!(a, b);
    assert_eq!(a, c);
    assert_ne!(b, c);
}
