use domain::model::courier::courier_aggregate::CourierId;
use domain::model::kernel::event::EventId;
use domain::model::order::order_aggregate::OrderId;
use domain::model::order::order_events::OrderEvent;
use ports::events_producer_port::Events;
use ports::events_producer_port::EventsProducerPort;
use std::sync::Arc;
use tokio::sync::Mutex;
use uuid::Uuid;

use crate::usecases::events::event_bus::EventBus;
use crate::usecases::events::event_bus::EventBusImpl;
use crate::usecases::events::order_completed_event_handler::OrderCompletedEventHandler;
use crate::usecases::events::order_created_event_hander::OrderCreatedEventHandler;

#[derive(Clone, Default)]
struct RecordingProducer {
    payloads: Arc<Mutex<Vec<EventId>>>,
}

impl EventsProducerPort for RecordingProducer {
    fn publish(&self, e: Events) {
        let payloads = self.payloads.clone();
        tokio::spawn(async move {
            let mut guard = payloads.lock().await;
            guard.push(match e {
                Events::Order(event) => match event {
                    OrderEvent::Created { id, .. } => id,
                    OrderEvent::Completed { id, .. } => id,
                },
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

    let mut bus = EventBusImpl::new();
    bus.register_order_created(handler_one);
    bus.register_order_completed(handler_two);

    let order_id = OrderId::new(Uuid::new_v4());
    let courier_id = CourierId(Uuid::new_v4());

    bus.commit(Events::Order(OrderEvent::created(order_id)))
        .await
        .unwrap();

    bus.commit(Events::Order(OrderEvent::completed(order_id, courier_id)))
        .await
        .unwrap();

    tokio::time::sleep(std::time::Duration::from_millis(50)).await;

    assert_eq!(producer_one.payloads.lock().await.len(), 1);
    assert_eq!(producer_two.payloads.lock().await.len(), 1);
}
