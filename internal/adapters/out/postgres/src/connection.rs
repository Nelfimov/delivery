use diesel::prelude::*;
use diesel::r2d2::ConnectionManager;
use diesel::r2d2::Pool;

pub struct PgConnectionOptions {
    host: String,
    port: u16,
    user: String,
    password: String,
    database: String,
}

impl PgConnectionOptions {
    pub fn new(host: String, port: u16, user: String, password: String, database: String) -> Self {
        Self {
            host,
            port,
            user,
            password,
            database,
        }
    }

    pub fn host(&self) -> String {
        self.host.clone()
    }
    pub fn port(&self) -> u16 {
        self.port
    }
    pub fn user(&self) -> String {
        self.user.clone()
    }
    pub fn password(&self) -> String {
        self.password.clone()
    }
    pub fn database(&self) -> String {
        self.database.clone()
    }
}

pub fn establish_connection(opt: PgConnectionOptions) -> Pool<ConnectionManager<PgConnection>> {
    let url = format!(
        "postgresql://{}:{}@{}:{}/{}",
        opt.user(),
        opt.password(),
        opt.host(),
        opt.port(),
        opt.database(),
    );

    let manager = ConnectionManager::<PgConnection>::new(url);
    Pool::builder()
        .test_on_check_out(true)
        .build(manager)
        .expect("Could not build connection pool")
}
