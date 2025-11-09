mod config;
mod cron;

use in_http::server::start_server;
use in_http::state::AppState;
use in_kafka::consumers::baskets_events_consumer::BasketEventsConsumer;
use out_grpc_geo::geo_service::GeoService;
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

    let app_state = AppState::new(courier_repo, order_repo, uow, geo_service);

    let mut scheduler = start_crons(pool.clone()).await;

    let consumer = BasketEventsConsumer::new(&config.kafka_host, &config.kafka_consumer_group);
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
