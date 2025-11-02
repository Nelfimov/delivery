use diesel::insert_into;
use diesel::prelude::*;

use domain::model::courier::courier_aggregate::Courier;
use domain::model::courier::courier_aggregate::CourierName;
use domain::model::courier::courier_aggregate::CourierSpeed;
use domain::model::kernel::location::Location;
use domain::model::kernel::volume::Volume;
use domain::model::order::order_aggregate::Order;
use domain::model::order::order_aggregate::OrderId;
use out_postgres::courier::courier_dto::CourierDto;
use out_postgres::courier::courier_schema::couriers;
use out_postgres::order::order_schema::orders;
use out_postgres::unit_of_work::UnitOfWork;
use ports::courier_repository_port::CourierRepositoryPort;
use ports::errors::RepositoryError;
use ports::order_repository_port::OrderRepositoryPort;
use ports::unit_of_work_port::UnitOfWorkPort;

mod common;
use common::TestPg;
use uuid::Uuid;

#[tokio::test]
async fn test_transaction_commit_and_rollback() {
    let test_pg = TestPg::new().await;

    let TestPg {
        connections,
        container: _,
    } = test_pg;

    let main_connection = &mut connections.get().unwrap();
    let mut uow = UnitOfWork::new(main_connection);

    let result: Result<(), RepositoryError> = uow.transaction(|tx| {
        let dto = CourierDto {
            id: Uuid::new_v4(),
            name: "rollback_courier".into(),
            speed: 10,
            location_x: 1,
            location_y: 2,
        };

        insert_into(out_postgres::courier::courier_schema::couriers::table)
            .values(&dto)
            .execute(tx.connection)
            .unwrap();

        Err(RepositoryError::MapError("force rollback".into()))
    });

    assert!(result.is_err(), "expected rollback due to error");

    let count: i64 = couriers::dsl::couriers
        .count()
        .first(&mut connections.get().unwrap())
        .unwrap();

    assert_eq!(count, 0, "rollback must remove inserted record");

    let result = uow.transaction(|tx| {
        let dto = CourierDto {
            id: Uuid::new_v4(),
            name: "committed_courier".into(),
            speed: 12,
            location_x: 3,
            location_y: 4,
        };

        insert_into(out_postgres::courier::courier_schema::couriers::table)
            .values(&dto)
            .execute(tx.connection)
            .unwrap();

        Ok(())
    });

    assert!(result.is_ok(), "commit must succeed");

    let count: i64 = couriers::dsl::couriers
        .count()
        .first(&mut connections.get().unwrap())
        .unwrap();

    assert_eq!(count, 1, "record must persist after commit");

    let _ = uow.transaction(|tx| {
        let courier = Courier::new(
            CourierName("second_courier".into()),
            CourierSpeed(2),
            Location::new(1, 1).unwrap(),
        )
        .unwrap();

        let order = Order::new(
            OrderId::new(Uuid::new_v4()),
            Location::new(2, 2).unwrap(),
            Volume::new(3).unwrap(),
        )
        .unwrap();

        let mut courier_repo = tx.courier_repo();
        courier_repo.add(courier).unwrap();

        let mut order_repo = tx.order_repo();
        order_repo.add(&order).unwrap();

        Ok(())
    });

    let count_couriers: i64 = couriers::dsl::couriers
        .count()
        .first(&mut connections.get().unwrap())
        .unwrap();
    let count_orders: i64 = orders::dsl::orders
        .count()
        .first(&mut connections.get().unwrap())
        .unwrap();

    assert_eq!(count_couriers, 2, "there should be 2 couriers");
    assert_eq!(count_orders, 1, "there should be 1 order");
}
