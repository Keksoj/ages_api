use crate::config::app_config::AppConfig;
use anyhow::Context;
use diesel::{
    pg::PgConnection,
    r2d2::{self, ConnectionManager},
    // Connection,
};

use diesel_migrations::embed_migrations;

embed_migrations!();

pub type DbConnection = PgConnection;
pub type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;

pub fn migrate_and_config_db(config: &AppConfig) -> anyhow::Result<Pool> {
    let pg_uri = &config.postgresql_uri;

    info!("Create a connection manager to {}", pg_uri);
    let manager = ConnectionManager::<PgConnection>::new(pg_uri);
    info!("Create a connection pool");
    let pool = r2d2::Pool::builder()
        .max_size(config.max_connections)
        .build(manager)
        .with_context(|| "Failed to create pool.")?;

    info!("Migrating...");
    embedded_migrations::run(
        &pool
            .get()
            .with_context(|| "Failed to retrieve a connection to the pool.")?,
    )
    .with_context(|| "The embedded migrations failed")?;

    debug!("Successfully ran the migrations");
    Ok(pool)
}
