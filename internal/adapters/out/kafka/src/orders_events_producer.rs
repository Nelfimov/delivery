use domain::model::order::order_events::OrderEvent;
use ports::events_producer_port::Events;
use ports::events_producer_port::EventsProducerPort;
use prost::Message;
use prost_types::Timestamp;
use rdkafka::ClientConfig;
use rdkafka::producer::FutureProducer;
use rdkafka::producer::FutureRecord;
use std::time::SystemTime;

use crate::order_event_gen::OrderCompletedIntegrationEvent;
use crate::order_event_gen::OrderCreatedIntegrationEvent;

static TOPIC: &str = "orders.events";

pub struct OrdersEventsProducer {
    producer: FutureProducer,
}

impl OrdersEventsProducer {
    pub fn new(brokers: &str, group_id: &str) -> Self {
        let mut config = ClientConfig::new();

        let producer: FutureProducer = config
            .set("bootstrap.servers", brokers)
            .create()
            .expect("could not create consumer");

        Self { producer }
    }
}

impl<'a> EventsProducerPort for OrdersEventsProducer {
    fn publish(&self, e: Events) {
        let payload = match e {
            Events::Order(event) => match event {
                OrderEvent::Created { 0: e } => OrderCreatedIntegrationEvent {
                    event_id: e.id.0.to_string(),
                    event_type: e.name,
                    occurred_at: Some(Timestamp::from(SystemTime::now())),
                    order_id: e.order_id.0.to_string(),
                }
                .encode_to_vec(),
                OrderEvent::Completed { 0: e } => OrderCompletedIntegrationEvent {
                    event_id: e.id.0.to_string(),
                    event_type: e.name,
                    order_id: e.order_id.0.to_string(),
                    courier_id: e.courier_id.0.to_string(),
                    occurred_at: Some(Timestamp::from(SystemTime::now())),
                }
                .encode_to_vec(),
            },
        };

        if let Err((error, _)) = self
            .producer
            .send_result(FutureRecord::<'a, Vec<u8>, Vec<u8>>::to(TOPIC).payload(&payload))
        {
            tracing::error!(?error, "failed to enqueue orders event to kafka");
        }
    }
}
