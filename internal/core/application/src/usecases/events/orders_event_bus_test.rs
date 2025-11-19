use domain::model::kernel::event::DomainEvent;
use domain::model::order::order_completed_event::OrderCompletedEvent;
use domain::model::order::order_created_event::OrderCreatedEvent;
use ports::events_producer_port::Events;
use ports::events_producer_port::EventsProducerPort;
use std::sync::Arc;
use tokio::sync::Mutex;
use uuid::Uuid;

use crate::usecases::EventBus;
use crate::usecases::events::order_completed_event_handler::OrderCompletedEventHandler;
use crate::usecases::events::order_created_event_hander::OrderCreatedEventHandler;
use crate::usecases::events::orders_event_bus::OrdersEventBus;

#[derive(Clone, Default)]
struct RecordingProducer {
    payloads: Arc<Mutex<Vec<String>>>,
}

impl EventsProducerPort for RecordingProducer {
    fn publish(&self, e: Events) {
        let payloads = self.payloads.clone();
        tokio::spawn(async move {
            let mut guard = payloads.lock().await;
            guard.push(match e {
                Events::OrderCreated(event) => event.id(),
                Events::OrderCompleted(event) => event.id(),
            });
        });
    }
}

#[tokio::test]
async fn fans_out_created_and_completed_events() {
    let producer_one = RecordingProducer::default();
    let producer_two = RecordingProducer::default();

    let handler_one = OrderCreatedEventHandler::new(producer_one.clone());
    let handler_two = OrderCompletedEventHandler::new(producer_two.clone());

    let mut bus = OrdersEventBus::new();
    bus.register_order_created(handler_one);
    bus.register_order_completed(handler_two);

    let order_id = Uuid::new_v4();
    let courier_id = Uuid::new_v4();

    bus.commit(Events::OrderCreated(OrderCreatedEvent::new(order_id)))
        .await
        .unwrap();

    bus.commit(Events::OrderCompleted(OrderCompletedEvent::new(
        order_id, courier_id,
    )))
    .await
    .unwrap();

    tokio::time::sleep(std::time::Duration::from_millis(50)).await;

    assert_eq!(producer_one.payloads.lock().await.len(), 1);
    assert_eq!(producer_two.payloads.lock().await.len(), 1);
}
