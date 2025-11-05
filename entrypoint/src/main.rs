mod config;

use in_http::server::start_server;
use in_http::state::AppState;
use out_postgres::connection::PgConnectionOptions;
use out_postgres::connection::establish_connection;

use config::Config;
use out_postgres::courier::courier_repository::CourierRepository;
use out_postgres::order::order_repository::OrderRepository;
use out_postgres::unit_of_work::UnitOfWork;

#[tokio::main]
async fn main() {
    let config = Config::from_env().expect("missing env variables");
    let pool = establish_connection(PgConnectionOptions::new(
        config.db_host,
        config.db_port,
        config.db_user,
        config.db_password,
    ));

    let courier_repo = CourierRepository::new(pool.clone());
    let order_repo = OrderRepository::new(pool.clone());
    let uow = UnitOfWork::new(pool.clone());

    let app_state = AppState::new(courier_repo, order_repo, uow);

    start_server(
        &format!("{}:{}", config.server_address, config.server_port),
        app_state,
    )
    .await;
}
