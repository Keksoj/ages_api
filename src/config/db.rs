// use crate::toolbox::errors::CustomError;
// use actix_web::web;
use diesel::{
    pg::PgConnection,
    r2d2::{self, ConnectionManager},
};

use diesel_migrations::embed_migrations;

embed_migrations!();

pub type DbConnection = PgConnection;
pub type Pool = r2d2::Pool<ConnectionManager<DbConnection>>;

pub fn migrate_and_config_db(db_url: &str) -> Pool {
    info!("Migrating and configurating database...");
    let manager = ConnectionManager::<DbConnection>::new(db_url);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");
    embedded_migrations::run(&pool.get().expect("Failed to migrate."))
        .expect("The embedded migrations failed");

    pool
}
