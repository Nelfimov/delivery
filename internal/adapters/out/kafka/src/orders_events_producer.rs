use ports::events_producer_port::Events;
use ports::events_producer_port::EventsProducerPort;
use prost::Message;
use rdkafka::ClientConfig;
use rdkafka::producer::FutureProducer;
use rdkafka::producer::FutureRecord;

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
            .set("group.id", group_id)
            .set("bootstrap.servers", brokers)
            .set("enable.partition.eof", "false")
            .set("session.timeout.ms", "6000")
            .create()
            .expect("could not create consumer");

        Self { producer }
    }
}

impl<'a> EventsProducerPort for OrdersEventsProducer {
    fn publish(&self, e: Events) {
        let payload = match e {
            Events::OrderCreated(body) => OrderCreatedIntegrationEvent::from(body).encode_to_vec(),
            Events::OrderCompleted(body) => {
                OrderCompletedIntegrationEvent::from(body).encode_to_vec()
            }
        };

        if let Err((error, _)) = self
            .producer
            .send_result(FutureRecord::<'a, Vec<u8>, Vec<u8>>::to(TOPIC).payload(&payload))
        {
            tracing::error!(?error, "failed to enqueue orders event to kafka");
        }
    }
}
