use application::usecases::CommandHandler;
use application::usecases::commands::create_order_command::CreateOrderCommand;
use application::usecases::commands::create_order_handler::CreateOrderHandler;
use ports::geo_service_port::GeoServicePort;
use ports::order_repository_port::OrderRepositoryPort;
use prost::Message as ProstMessage;
use rdkafka::ClientConfig;
use rdkafka::Message;
use rdkafka::consumer::Consumer;
use rdkafka::consumer::StreamConsumer;
use std::str::FromStr;
use std::time::Duration;
use tracing::Level;
use tracing::event;
use tracing::info;
use tracing::span;
use tracing::warn;

use crate::messages::BasketConfirmedIntegrationEvent;
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
    pub fn new(
        brokers: &str,
        group_id: &str,
        order_repo: Shared<OR>,
        geo_service: GS,
    ) -> Self {
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
                            match event.address {
                                None => {
                                    warn!("event has no address");
                                    continue;
                                }
                                Some(address) => match uuid::Uuid::from_str(&event.basket_id) {
                                    Err(e) => {
                                        warn!("event non uuid format: {}", e);
                                        continue;
                                    }
                                    Ok(id) => {
                                        let command = match CreateOrderCommand::new(
                                            id,
                                            address.street,
                                            event.volume as u16,
                                        ) {
                                            Ok(c) => c,
                                            Err(e) => {
                                                warn!("could not create command: {}", e);
                                                continue;
                                            }
                                        };
                                        let mut handler = CreateOrderHandler::new(
                                            self.order_repo.clone(),
                                            self.geo_service.clone(),
                                        );

                                        if let Err(err) = handler.execute(command).await {
                                            warn!(
                                                ?err,
                                                "failed to handle BasketConfirmedIntegrationEvent"
                                            );
                                            continue;
                                        }
                                    }
                                },
                            }
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
