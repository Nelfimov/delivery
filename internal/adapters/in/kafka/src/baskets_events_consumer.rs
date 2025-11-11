use application::usecases::CommandHandler;
use application::usecases::commands::create_order_command::CreateOrderCommand;
use application::usecases::commands::create_order_handler::CreateOrderHandler;
use ports::geo_service_port::GeoServicePort;
use ports::order_repository_port::OrderRepositoryPort;
use rdkafka::ClientConfig;
use rdkafka::Message;
use rdkafka::consumer::Consumer;
use rdkafka::consumer::StreamConsumer;
use std::str::FromStr;
use std::time::Duration;
use tracing::Level;
use tracing::debug;
use tracing::event;
use tracing::info;
use tracing::span;
use tracing::warn;

use crate::mapper::BasketEventPayload;
use crate::shared::Shared;

static TOPIC: [&str; 1] = ["baskets.events"];

pub struct BasketEventsConsumer<OR, GS>
where
    OR: OrderRepositoryPort,
    GS: GeoServicePort + Clone,
{
    consumer: StreamConsumer,
    order_repo: Shared<OR>,
    geo_service: GS,
}

impl<OR, GS> BasketEventsConsumer<OR, GS>
where
    OR: OrderRepositoryPort,
    GS: GeoServicePort + Clone,
{
    pub fn new(brokers: &str, group_id: &str, order_repo: Shared<OR>, geo_service: GS) -> Self {
        let mut config = ClientConfig::new();

        let consumer: StreamConsumer = config
            .set("group.id", group_id)
            .set("bootstrap.servers", brokers)
            .set("enable.partition.eof", "false")
            .set("session.timeout.ms", "6000")
            .create()
            .expect("could not create consumer");

        consumer
            .subscribe(&TOPIC)
            .unwrap_or_else(|e| panic!("could not subscribe to topic {:?}: {}", TOPIC, e));

        consumer
            .client()
            .fetch_metadata(None, Duration::from_secs(5))
            .expect("kafka metadata fetch failed");

        Self {
            consumer,
            order_repo,
            geo_service,
        }
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
                        Some(Ok(payload)) => {
                            debug!("{:?}", payload);
                            payload
                        }
                    };

                    let event: BasketEventPayload = match serde_json::from_slice(payload) {
                        Ok(event) => event,
                        Err(err) => {
                            warn!(?err, "failed to parse BasketConfirmedIntegrationEvent JSON");
                            continue;
                        }
                    };

                    info!(
                        event_id = event.event_id,
                        basket_id = event.basket_id,
                        "received BasketConfirmedIntegrationEvent"
                    );

                    let address = match event.address {
                        Some(address) => address,
                        None => {
                            warn!("event has no address");
                            continue;
                        }
                    };

                    let id = match uuid::Uuid::from_str(&event.basket_id) {
                        Ok(id) => id,
                        Err(err) => {
                            warn!(?err, "event basket_id is not a valid UUID");
                            continue;
                        }
                    };

                    let volume = match u16::try_from(event.volume) {
                        Ok(volume) => volume,
                        Err(err) => {
                            warn!(?err, "event volume is out of range for u16");
                            continue;
                        }
                    };

                    let command = match CreateOrderCommand::new(id, address.street, volume) {
                        Ok(command) => command,
                        Err(err) => {
                            warn!(?err, "could not create command");
                            continue;
                        }
                    };

                    let mut handler =
                        CreateOrderHandler::new(self.order_repo.clone(), self.geo_service.clone());

                    if let Err(err) = handler.execute(command).await {
                        warn!(?err, "failed to handle BasketConfirmedIntegrationEvent");
                        continue;
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
