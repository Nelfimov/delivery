use std::cell::RefCell;
use std::fmt::Display;
use std::rc::Rc;

use domain::model::courier::courier_aggregate::Courier;
use domain::model::courier::courier_aggregate::CourierId;
use domain::model::courier::courier_aggregate::CourierName;
use domain::model::courier::courier_aggregate::CourierSpeed;
use domain::model::kernel::location::Location;
use domain::model::kernel::volume::Volume;
use domain::model::order::order_aggregate::Order;
use domain::model::order::order_aggregate::OrderId;
use domain::model::order::order_aggregate::OrderStatus;
use ports::courier_repository_port::CourierRepositoryPort;
use ports::courier_repository_port::GetAllCouriersResponse;
use ports::errors::RepositoryError;
use ports::order_repository_port::OrderRepositoryPort;
use ports::unit_of_work_port::UnitOfWorkPort;
use uuid::Uuid;

use crate::usecases::CommandHandler;
use crate::usecases::commands::assign_order_command::AssignOrderCommand;
use crate::usecases::commands::assign_order_handler::AssignOrderHandler;

#[derive(Clone, Debug)]
struct StoredOrder {
    id: OrderId,
    courier_id: Option<CourierId>,
    location: Location,
    volume: Volume,
    status: OrderStatus,
}

impl Display for StoredOrder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl StoredOrder {
    fn from_order(order: &Order) -> Self {
        Self {
            id: order.id(),
            courier_id: *order.courier_id(),
            location: order.location().clone(),
            volume: Volume::new(order.volume()).expect("volume must be positive"),
            status: copy_status(order.status()),
        }
    }

    fn to_order(&self) -> Order {
        Order::restore(
            self.id,
            self.courier_id,
            self.location.clone(),
            self.volume,
            copy_status(&self.status),
        )
    }

    fn update_from(&mut self, order: &Order) {
        self.courier_id = *order.courier_id();
        self.location = order.location().clone();
        self.volume = Volume::new(order.volume()).expect("volume must be positive");
        self.status = copy_status(order.status());
    }
}

fn copy_status(status: &OrderStatus) -> OrderStatus {
    match status {
        OrderStatus::Created => OrderStatus::Created,
        OrderStatus::Assigned => OrderStatus::Assigned,
        OrderStatus::Completed => OrderStatus::Completed,
    }
}

struct TestOrderRepository {
    orders: Rc<RefCell<Vec<StoredOrder>>>,
}

impl OrderRepositoryPort for TestOrderRepository {
    fn add(&mut self, _order: &Order) -> Result<(), RepositoryError> {
        unimplemented!()
    }

    fn update(&mut self, order: &Order) -> Result<(), RepositoryError> {
        let mut orders = self.orders.borrow_mut();
        if let Some(stored) = orders.iter_mut().find(|o| o.id == order.id()) {
            stored.update_from(order);
            return Ok(());
        }

        Err(RepositoryError::NotFound("order not found".into()))
    }

    fn get_by_id(&mut self, _id: OrderId) -> Result<Order, RepositoryError> {
        unimplemented!()
    }

    fn get_any_new(&mut self) -> Result<Order, RepositoryError> {
        unimplemented!()
    }

    fn get_all_assigned(&mut self) -> Result<Vec<Order>, RepositoryError> {
        unimplemented!()
    }

    fn raw(&mut self, _query: String) -> Result<Vec<Order>, RepositoryError> {
        Ok(self
            .orders
            .borrow()
            .iter()
            .map(StoredOrder::to_order)
            .collect())
    }
}

struct TestCourierRepository {
    couriers: Rc<RefCell<Vec<Courier>>>,
}

impl CourierRepositoryPort for TestCourierRepository {
    fn add(&mut self, courier: Courier) -> Result<(), RepositoryError> {
        self.couriers.borrow_mut().push(courier);
        Ok(())
    }

    fn update(&mut self, courier: Courier) -> Result<(), RepositoryError> {
        let mut couriers = self.couriers.borrow_mut();
        if let Some(existing) = couriers
            .iter_mut()
            .find(|stored| stored.id() == courier.id())
        {
            *existing = courier;
            return Ok(());
        }

        Err(RepositoryError::NotFound("courier not found".into()))
    }

    fn get_by_id(&mut self, _id: CourierId) -> Result<Courier, RepositoryError> {
        unimplemented!()
    }

    fn get_all_free(&mut self) -> Result<Vec<Courier>, RepositoryError> {
        Ok(self.couriers.borrow().clone())
    }

    fn get_all_couriers(&mut self) -> Result<Vec<GetAllCouriersResponse>, RepositoryError> {
        unimplemented!()
    }
}

struct TestUnitOfWork {
    orders: Rc<RefCell<Vec<StoredOrder>>>,
    couriers: Rc<RefCell<Vec<Courier>>>,
}

impl std::fmt::Debug for TestUnitOfWork {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TestUnitOfWork")
            .field("orders", &self.orders)
            .field("couriers", &self.couriers)
            .finish()
    }
}

impl TestUnitOfWork {
    fn from_state(
        orders: Rc<RefCell<Vec<StoredOrder>>>,
        couriers: Rc<RefCell<Vec<Courier>>>,
    ) -> Self {
        Self { orders, couriers }
    }
}

struct TestUnitOfWorkTx {
    orders: Rc<RefCell<Vec<StoredOrder>>>,
    couriers: Rc<RefCell<Vec<Courier>>>,
}

impl UnitOfWorkPort for TestUnitOfWork {
    type Uow = TestUnitOfWorkTx;
    type CourierRepo = TestCourierRepository;
    type OrderRepo = TestOrderRepository;

    fn transaction<F, T>(&mut self, f: F) -> Result<T, RepositoryError>
    where
        F: for<'tx> FnOnce(&mut Self::Uow) -> Result<T, RepositoryError>,
    {
        let mut tx = TestUnitOfWorkTx {
            orders: Rc::clone(&self.orders),
            couriers: Rc::clone(&self.couriers),
        };
        f(&mut tx)
    }

    fn courier_repo(&mut self) -> Self::CourierRepo {
        TestCourierRepository {
            couriers: Rc::clone(&self.couriers),
        }
    }

    fn order_repo(&mut self) -> Self::OrderRepo {
        TestOrderRepository {
            orders: Rc::clone(&self.orders),
        }
    }
}

impl UnitOfWorkPort for TestUnitOfWorkTx {
    type Uow = TestUnitOfWorkTx;
    type CourierRepo = TestCourierRepository;
    type OrderRepo = TestOrderRepository;

    fn transaction<F, T>(&mut self, f: F) -> Result<T, RepositoryError>
    where
        F: for<'tx> FnOnce(&mut Self::Uow) -> Result<T, RepositoryError>,
    {
        f(self)
    }

    fn courier_repo(&mut self) -> Self::CourierRepo {
        TestCourierRepository {
            couriers: Rc::clone(&self.couriers),
        }
    }

    fn order_repo(&mut self) -> Self::OrderRepo {
        TestOrderRepository {
            orders: Rc::clone(&self.orders),
        }
    }
}

fn initial_state() -> (Vec<StoredOrder>, Vec<Courier>) {
    let courier_1 = Courier::new(
        CourierName("Bob".into()),
        CourierSpeed(1),
        Location::new(1, 1).unwrap(),
    )
    .unwrap();
    let courier_2 = Courier::new(
        CourierName("Rick".into()),
        CourierSpeed(2),
        Location::new(2, 2).unwrap(),
    )
    .unwrap();
    let courier_3 = Courier::new(
        CourierName("Nick".into()),
        CourierSpeed(3),
        Location::new(3, 3).unwrap(),
    )
    .unwrap();

    let order_1 = Order::new(
        OrderId::new(Uuid::new_v4()),
        Location::new(1, 1).unwrap(),
        Volume::new(1).unwrap(),
    )
    .unwrap();
    (
        vec![StoredOrder::from_order(&order_1)],
        vec![courier_1, courier_2, courier_3],
    )
}

#[tokio::test]
async fn handle_assigns_order() {
    let (orders, couriers) = initial_state();
    let orders_state = Rc::new(RefCell::new(orders));
    let couriers_state = Rc::new(RefCell::new(couriers));

    let mut handler = AssignOrderHandler::new(TestUnitOfWork::from_state(
        Rc::clone(&orders_state),
        Rc::clone(&couriers_state),
    ));
    let command = AssignOrderCommand::new().expect("command should be valid");

    handler
        .execute(command)
        .await
        .expect("handler should finish successfully");

    let orders = orders_state.borrow();
    assert!(orders.iter().all(|order| {
        println!("{:?}", order);
        matches!(order.status, OrderStatus::Assigned)
    }));
    assert!(orders.iter().all(|order| order.courier_id.is_some()));
}
