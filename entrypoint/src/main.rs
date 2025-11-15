mod config;
mod cron;

use application::usecases::EventBus;
use application::usecases::events::order_completed_event_handler::OrderCompletedEventHandler;
use application::usecases::events::order_created_event_hander::OrderCreatedEventHandler;
use application::usecases::events::orders_event_bus::OrdersEventBus;
use in_http::server::start_server;
use in_http::state::AppState;
use in_http::state::AsyncShared;
use in_kafka::baskets_events_consumer::BasketEventsConsumer;
use in_kafka::shared::Shared;
use out_grpc_geo::geo_service::GeoService;
use out_kafka::orders_events_producer::OrdersEventsProducer;
use out_postgres::connection::PgConnectionOptions;
use out_postgres::connection::establish_connection;
use out_postgres::courier::courier_repository::CourierRepository;
use out_postgres::order::order_repository::OrderRepository;
use out_postgres::unit_of_work::UnitOfWork;

use crate::config::Config;
use crate::cron::start_crons;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    let config = Config::from_env().expect("missing env variables");
    tracing::event!(
        tracing::Level::INFO,
        "Start server: {}:{}",
        config.server_address,
        config.server_port
    );

    let geo_service = GeoService::new(format!("{}:{}", config.geo_address, config.geo_port))
        .await
        .expect("could not connect to geo service");
    tracing::event!(tracing::Level::INFO, "Succesfull connect to geo");

    let pool = establish_connection(PgConnectionOptions::new(
        config.db_host.clone(),
        config.db_port,
        config.db_user.clone(),
        config.db_password.clone(),
        config.db_name.clone(),
    ));

    let courier_repo = CourierRepository::new(pool.clone());
    let order_repo = OrderRepository::new(pool.clone());
    let uow = UnitOfWork::new(pool.clone());

    let mut event_bus = OrdersEventBus::new();
    let orders_created_producer =
        OrdersEventsProducer::new(&config.kafka_host, &config.kafka_consumer_group);
    let orders_completed_producer =
        OrdersEventsProducer::new(&config.kafka_host, &config.kafka_consumer_group);
    event_bus.register_order_created(OrderCreatedEventHandler::new(orders_created_producer));
    event_bus.register_order_completed(OrderCompletedEventHandler::new(orders_completed_producer));
    let orders_event_bus = AsyncShared::new(event_bus);
    let consumer_event_bus = orders_event_bus.clone();

    let app_state = AppState::new(
        courier_repo,
        order_repo,
        uow,
        geo_service.clone(),
        consumer_event_bus,
    );

    let cron_event_bus = orders_event_bus.clone();
    let mut scheduler = start_crons(pool.clone(), cron_event_bus).await;

    let consumer_order_repo = Shared::new(OrderRepository::new(pool.clone()));
    let consumer_geo_service = geo_service;
    let consumer = BasketEventsConsumer::new(
        &config.kafka_host,
        &config.kafka_consumer_group,
        consumer_order_repo,
        consumer_geo_service,
        orders_event_bus,
    );
    let _consumer_handle = tokio::spawn(async move {
        consumer.consume().await;
    });

    start_server(
        &format!("{}:{}", config.server_address, config.server_port),
        app_state,
    )
    .await;

    if let Err(error) = scheduler.shutdown().await {
        tracing::error!(?error, "failed to shutdown cron scheduler");
    }
}
