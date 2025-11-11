use diesel::PgConnection;
use diesel::r2d2::ConnectionManager;
use diesel_migrations::EmbeddedMigrations;
use diesel_migrations::MigrationHarness;
use diesel_migrations::embed_migrations;
use r2d2::PooledConnection;

pub fn run_migrations(mut connection: PooledConnection<ConnectionManager<PgConnection>>) {
    const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations");

    connection
        .run_pending_migrations(MIGRATIONS)
        .expect("could not apply migrations");
}
