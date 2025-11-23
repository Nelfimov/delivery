use async_trait::async_trait;
use domain::model::kernel::location::Location;
use domain::model::order::order_aggregate::Order;
use domain::model::order::order_aggregate::OrderId;
use ports::errors::GeoClientError;
use ports::errors::RepositoryError;
use ports::geo_service_port::GeoServicePort;
use ports::order_repository_port::OrderRepositoryPort;
use std::sync::Arc;
use std::sync::Mutex;
use uuid::Uuid;

use crate::errors::command_errors::CommandError;
use crate::usecases::CommandHandler;
use crate::usecases::Handler;
use crate::usecases::events::event_bus::EventBus;

use super::create_order_command::CreateOrderCommand;
use super::create_order_handler::CreateOrderHandler;

struct MockOrderRepository {
    added_order_id: Arc<Mutex<Option<OrderId>>>,
    fail_on_add: bool,
}

impl MockOrderRepository {
    fn new(added_order_id: Arc<Mutex<Option<OrderId>>>) -> Self {
        Self {
            added_order_id,
            fail_on_add: false,
        }
    }

    fn new_failing(added_order_id: Arc<Mutex<Option<OrderId>>>) -> Self {
        Self {
            added_order_id,
            fail_on_add: true,
        }
    }
}

impl OrderRepositoryPort for MockOrderRepository {
    fn add(&mut self, order: &Order) -> Result<(), RepositoryError> {
        if self.fail_on_add {
            return Err(RepositoryError::DatabaseError("db unavailable".to_string()));
        }

        let mut slot = self.added_order_id.lock().expect("mutex poisoned");
        *slot = Some(order.id());
        Ok(())
    }

    fn update(&mut self, _: &Order) -> Result<(), RepositoryError> {
        unimplemented!("not required for this test");
    }

    fn get_by_id(&mut self, _: OrderId) -> Result<Order, RepositoryError> {
        unimplemented!("not required for this test");
    }

    fn get_any_new(&mut self) -> Result<Order, RepositoryError> {
        unimplemented!("not required for this test");
    }

    fn get_all_assigned(&mut self) -> Result<Vec<Order>, RepositoryError> {
        unimplemented!("not required for this test");
    }

    fn raw(&mut self, _: String) -> Result<Vec<Order>, RepositoryError> {
        unimplemented!("not required for this test");
    }
}

#[derive(Clone, Copy)]
struct GeoServiceMock;

#[async_trait]
impl GeoServicePort for GeoServiceMock {
    async fn get_location(&mut self, _address: String) -> Result<Location, GeoClientError> {
        Ok(Location::new(1, 1)
            .map_err(|e| GeoClientError::ExecutionError(e.to_string()))
            .unwrap())
    }
}

struct RecordingEventBus {
    events: Arc<Mutex<Vec<Events>>>,
}

impl RecordingEventBus {
    fn new(events: Arc<Mutex<Vec<Events>>>) -> Self {
        Self { events }
    }
}

impl EventBus for RecordingEventBus {
    fn register_order_created(&mut self, _subscriber: impl Handler + 'static) {}

    fn register_order_completed(&mut self, _subscriber: impl Handler + 'static) {}

    async fn commit(&mut self, event: Events) -> Result<(), CommandError> {
        let mut events = self.events.lock().expect("event log poisoned");
        events.push(event);
        Ok(())
    }
}

#[tokio::test]
async fn handle_persists_order_via_repository() {
    let stored_id = Arc::new(Mutex::new(None));
    let repo = MockOrderRepository::new(stored_id.clone());
    let geo_service = GeoServiceMock;
    let observed_events = Arc::new(Mutex::new(Vec::new()));
    let _event_bus = RecordingEventBus::new(observed_events.clone());

    let mut handler = CreateOrderHandler::new(repo, geo_service);
    let command = CreateOrderCommand::new(Uuid::new_v4(), "Tverskaya street 1".to_string(), 10)
        .expect("command should be valid");

    handler
        .execute(command)
        .await
        .expect("handler should persist order");

    let observed = *stored_id.lock().expect("mutex poisoned");
    assert!(
        observed.is_some(),
        "order id should be recorded in repository"
    );

    let events = observed_events.lock().expect("event log poisoned");
    assert_eq!(events.len(), 1, "order creation should emit event");
}

#[tokio::test]
async fn handle_propagates_repository_error() {
    let stored_id = Arc::new(Mutex::new(None));
    let repo = MockOrderRepository::new_failing(stored_id);
    let geo_service = GeoServiceMock;
    let _event_bus = RecordingEventBus::new(Arc::new(Mutex::new(Vec::new())));

    let mut handler = CreateOrderHandler::new(repo, geo_service);
    let command = CreateOrderCommand::new(Uuid::new_v4(), "Nevsky prospect 10".to_string(), 5)
        .expect("command should be valid");

    let result = handler.execute(command).await;
    assert!(result.is_err(), "handler must surface repository failures");
}
use ports::events_producer_port::Events;
