#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;
#[macro_use]
extern crate log;

pub mod config;
pub mod endpoints;
pub mod toolbox;
pub mod models;
pub mod schema;
pub mod jwt;
pub mod middleware;

use actix_cors::Cors;
use actix_service::Service;
use actix_web::{http::header, App, HttpServer};
use dotenv::dotenv;
use env_logger;
use futures::FutureExt;
use std::env;
use std::io::Result;

#[actix_rt::main]
async fn main() -> Result<()> {
    dotenv().ok().expect("Failed to read the .env file.");
    env::set_var("RUST_LOG", "actix_web,actix-service,todo_back_end,diesel");
    env_logger::init();
    config::db::init();

    let bind_url = get_bind_url_from_env();

    HttpServer::new(|| {
        App::new()
            .wrap(
                Cors::new()
                    .send_wildcard()
                    .allowed_origin("http://localhost:3000") // the react front-end
                    .allowed_headers(vec![header::AUTHORIZATION, header::ACCEPT])
                    .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
                    .allowed_header(header::CONTENT_TYPE)
                    .finish(),
            )
            .wrap(actix_web::middleware::Logger::default())
            .wrap(middleware::authentication::Authentication)
            .wrap_fn(|request, service| {
                service.call(request).map(|response| response)
            })
            .configure(config::routes::config_services)
    })
    .bind(bind_url)?
    .run()
    .await
}

fn get_bind_url_from_env() -> String {
    let host = env::var("HOST").expect("a HOST is not provided in the environment");
    let port = env::var("PORT").expect("a PORT is not provided in the environment");
    let bind_url = format!("{}:{}", host, port).to_string();
    info!("Listening to {}...", bind_url);
    bind_url
}
