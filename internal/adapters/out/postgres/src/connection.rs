use diesel::prelude::*;

pub struct PgConnectionOptions {
    host: String,
    port: u16,
    user: String,
    password: String,
}

impl PgConnectionOptions {
    pub fn new(host: String, port: u16, user: String, password: String) -> Self {
        Self {
            host,
            port,
            user,
            password,
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
}

pub fn establish_connection(opt: PgConnectionOptions) -> PgConnection {
    let database_url = format!(
        "postgresql://{}:{}@{}:{}",
        opt.user(),
        opt.password(),
        opt.host(),
        opt.port()
    );

    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}
