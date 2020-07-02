use crate::toolbox::errors::CustomError;
use diesel::pg::PgConnection;
use diesel::r2d2::ConnectionManager;
use diesel_migrations::embed_migrations;

use lazy_static::lazy_static;
use r2d2;
use std::env;

pub type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;
pub type DbConnection = r2d2::PooledConnection<ConnectionManager<PgConnection>>;

lazy_static! {
    static ref POOL: Pool = {
        let db_url = env::var("DATABASE_URL").expect("Database url not set");
        let manager = ConnectionManager::<PgConnection>::new(db_url);
        Pool::new(manager).expect("Failed to create db pool")
    };
}

embed_migrations!();

pub fn init() {
    lazy_static::initialize(&POOL);
    let conn = connection().expect("Failed to get db connection");
    embedded_migrations::run(&conn).unwrap();
}

pub fn connection() -> Result<DbConnection, CustomError> {
    POOL.get().map_err(|error| {
        CustomError::new(500, format!("Failed getting db connection: {}", error))
    })
}
