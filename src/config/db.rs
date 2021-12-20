use anyhow::Context;
use diesel::{
    pg::PgConnection,
    r2d2::{self, ConnectionManager},
};
use diesel_migrations::embed_migrations;

embed_migrations!();

pub type DbConnection = PgConnection;
pub type Pool = r2d2::Pool<ConnectionManager<DbConnection>>;

pub fn migrate_and_config_db(db_url: &str) -> anyhow::Result<Pool> {
    info!("Migrating and configurating database...");
    let manager = ConnectionManager::<DbConnection>::new(db_url);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .context("Failed to create pool.")?;

    let connection = pool.get().context("Failed to migrate.")?;

    embedded_migrations::run(&connection).context("The embedded migrations failed")?;

    Ok(pool)
}
