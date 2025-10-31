use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::r2d2::ConnectionManager;
use diesel::r2d2::Pool;
use out_postgres::connection::PgConnectionOptions;
use out_postgres::connection::establish_connection;
use testcontainers::ContainerAsync;
use testcontainers::runners::AsyncRunner;
use testcontainers_modules::postgres::Postgres;

pub struct TestPg {
    pub connections: Pool<ConnectionManager<PgConnection>>,
    pub container: ContainerAsync<Postgres>,
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
        ));

        println!("Connection string {}:{}", host, port);

        diesel::sql_query(
            r#"
            CREATE TABLE couriers (
                id UUID PRIMARY KEY,
                name TEXT NOT NULL,
                speed SMALLINT NOT NULL,
                location_x SMALLINT NOT NULL,
                location_y SMALLINT NOT NULL
            );
            "#,
        )
        .execute(&mut connections.get().unwrap())
        .expect("create couriers table failed");
        diesel::sql_query(
            r#"
            CREATE TABLE orders (
                id UUID PRIMARY KEY,
                courier_id UUID,
                location_x SMALLINT NOT NULL,
                location_y SMALLINT NOT NULL,
                volume SMALLINT NOT NULL,
                status TEXT NOT NULL
            );
            "#,
        )
        .execute(&mut connections.get().unwrap())
        .expect("create orders table failed");

        Self {
            connections,
            container,
        }
    }
}
