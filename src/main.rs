#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;
#[macro_use]
extern crate log;

pub mod config;
pub mod controllers;
pub mod jwt;
pub mod middleware;
pub mod models;
pub mod schema;
pub mod toolbox;

use actix_cors::Cors;
use actix_files::Files;
use actix_web::middleware::Logger;
use actix_web::{http::header, App, HttpServer};
use anyhow::Context;
use config::{app_env::AppEnv, db::migrate_and_config_db, routes::config_routes};
use dotenv::dotenv;
use env_logger;
use middleware::authentication::Authentication;
// use std::env;

#[actix_rt::main]
async fn main() -> anyhow::Result<()> {
    let _path_buffer = dotenv().context("Failed to read the .env file.")?;

    let app_env = AppEnv::establish()?;
    let cloned_env = app_env.clone();

    env_logger::init();

    let pool = migrate_and_config_db(&app_env.database_url)
        .context("Failed to migrate and configure database")?;

    HttpServer::new(move || {
        App::new()
            .wrap(
                Cors::new()
                    .send_wildcard()
                    .allowed_origin(&cloned_env.allowed_origin)
                    .allowed_headers(vec![header::AUTHORIZATION, header::ACCEPT])
                    .allowed_methods(&cloned_env.allowed_methods)
                    .allowed_header(header::CONTENT_TYPE)
                    .finish(),
            )
            .data(pool.clone())
            .wrap(Logger::default())
            .wrap(Authentication)
            .configure(config_routes)
            .service(
                Files::new("/documentation", "./openapi").index_file("apicontract.json"),
            )
    })
    .bind(&app_env.bind_url)?
    .run()
    .await?;

    Ok(())
}
