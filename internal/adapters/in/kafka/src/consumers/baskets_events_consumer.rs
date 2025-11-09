use std::time::Duration;

use prost::Message as ProstMessage;
use rdkafka::ClientConfig;
use rdkafka::Message;
use rdkafka::consumer::Consumer;
use rdkafka::consumer::StreamConsumer;
use tracing::Level;
use tracing::event;
use tracing::info;
use tracing::span;
use tracing::warn;

use crate::messages::BasketConfirmedIntegrationEvent;

static TOPIC: [&str; 1] = ["baskets.events"];

pub struct BasketEventsConsumer {
    consumer: StreamConsumer,
}

impl BasketEventsConsumer {
    pub fn new(brokers: &str, group_id: &str) -> Self {
        let mut config = ClientConfig::new();

        let consumer: StreamConsumer = config
            .set("group.id", group_id)
            .set("bootstrap.servers", brokers)
            .set("enable.partition.eof", "false")
            .set("session.timeout.ms", "6000")
            .set("enable.auto.commit", "true")
            .create()
            .expect("could not create consumer");

        consumer
            .subscribe(&TOPIC)
            .unwrap_or_else(|e| panic!("could not subscribe to topic {:?}: {}", TOPIC, e));

        consumer
            .client()
            .fetch_metadata(None, Duration::from_secs(5))
            .expect("kafka metadata fetch failed");

        Self { consumer }
    }

    pub async fn consume(&self) {
        let span = span!(Level::TRACE, "consumer");
        let _ = span.enter();

        event!(Level::INFO, "consuming topic");

        loop {
            match self.consumer.recv().await {
                Err(e) => warn!("could not consume message: {}", e),
                Ok(msg) => {
                    let payload = match msg.payload_view::<[u8]>() {
                        None => continue,
                        Some(Err(e)) => {
                            warn!("error reading kafka payload: {:?}", e);
                            continue;
                        }
                        Some(Ok(payload)) => payload,
                    };

                    match BasketConfirmedIntegrationEvent::decode(payload) {
                        Err(err) => {
                            warn!(?err, "failed to decode BasketConfirmedIntegrationEvent");
                            continue;
                        }
                        Ok(event) => {
                            info!(
                                event_id = event.event_id,
                                basket_id = event.basket_id,
                                "received BasketConfirmedIntegrationEvent"
                            );
                        }
                    }

                    match self
                        .consumer
                        .commit_message(&msg, rdkafka::consumer::CommitMode::Async)
                    {
                        Ok(_) => continue,
                        Err(e) => {
                            warn!("could not commit message: {:?}", e);
                            continue;
                        }
                    }
                }
            }
        }
    }
}
