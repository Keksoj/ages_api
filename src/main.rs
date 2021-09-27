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
use config::{app_config::AppConfig, db::migrate_and_config_db, routes::config_routes};
use env_logger;
use middleware::authentication::Authentication;

#[actix_rt::main]
async fn main() -> anyhow::Result<()> {
    
    let app_config = AppConfig::establish()?;
    debug!("Starting the app with this config: {:#?}", app_config);
    let cloned_config = app_config.clone();
    
    std::env::set_var("RUST_LOG", app_config.log_level.to_string());
    env_logger::init();
    

    let pool = migrate_and_config_db(&app_config)?;

    HttpServer::new(move || {
        App::new()
            .wrap(
                Cors::new()
                    .send_wildcard()
                    .allowed_origin("*")
                    .allowed_headers(vec![header::AUTHORIZATION, header::ACCEPT])
                    .allowed_methods(&cloned_config.allowed_methods)
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
    .bind(&app_config.socket_address)?
    .run()
    .await?;
    Ok(())
}
