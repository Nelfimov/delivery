use diesel::pg::PgConnection;
use diesel::r2d2::ConnectionManager;
use diesel::r2d2::Pool;
use out_postgres::connection::PgConnectionOptions;
use out_postgres::connection::establish_connection;
use testcontainers::ContainerAsync;
use testcontainers::runners::AsyncRunner;
use testcontainers_modules::postgres::Postgres;

pub struct TestPg {
    pub connections: Pool<ConnectionManager<PgConnection>>,
    pub _container: ContainerAsync<Postgres>,
}

impl TestPg {
    pub async fn new() -> Self {
        let container = Postgres::default().start().await.unwrap();

        println!("Container started");

        let host = container.get_host().await.unwrap();
        let port = container.get_host_port_ipv4(5432).await.unwrap();
        let connections = establish_connection(PgConnectionOptions::new(
            host.to_string(),
            port,
            "postgres".into(),
            "postgres".into(),
            "postgres".into(),
        ));

        Self {
            connections,
            _container: container,
        }
    }
}
