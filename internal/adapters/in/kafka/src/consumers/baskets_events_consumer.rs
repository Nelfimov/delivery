use std::time::Duration;

use rdkafka::ClientConfig;
use rdkafka::Message;
use rdkafka::consumer::Consumer;
use rdkafka::consumer::StreamConsumer;
use tracing::Level;
use tracing::event;
use tracing::info;
use tracing::span;
use tracing::warn;

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
                    let payload = match msg.payload_view() {
                        None => "",
                        Some(Ok(s)) => s,
                        Some(Err(e)) => {
                            warn!("error deserializing message: {:?}", e);
                            ""
                        }
                    };
                    info!(
                        "key: '{:?}', payload: '{}', topic: {}, partition: {}, offset: {}, timestamp: {:?}",
                        msg.key(),
                        payload,
                        msg.topic(),
                        msg.partition(),
                        msg.offset(),
                        msg.timestamp()
                    );
                }
            }
        }
    }
}
