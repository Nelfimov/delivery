use rdkafka::ClientConfig;
use rdkafka::producer::FutureProducer;

static TOPIC: [&str; 1] = ["orders.events"];

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

    pub async fn consume(&self) {}
}
