#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;
#[macro_use]
extern crate log;

pub mod config;
pub mod endpoints;
pub mod jwt;
pub mod middleware;
pub mod models;
pub mod schema;
pub mod toolbox;

use actix_cors::Cors;
use actix_service::Service;
use actix_web::{
    http::header,
    web,
    web::{delete, get, post, put, resource, scope, ServiceConfig},
    App, HttpServer,
};

use dotenv::dotenv;
use env_logger;
use futures::FutureExt;
use std::env;
use std::io::Result;

#[actix_rt::main]
async fn main() -> Result<()> {
    dotenv().ok().expect("Failed to read the .env file.");
    env::set_var("RUST_LOG", "actix_web,actix-service,back,diesel");
    env_logger::init();
    config::db::init();

    let bind_url = get_bind_url_from_env();

    HttpServer::new(|| {
        // config::app::build_the_app();
        App::new()
            .wrap(
                Cors::new()
                    .send_wildcard()
                    .allowed_origin("http://localhost:3000") // the react front-end
                    .allowed_headers(vec![
                        header::AUTHORIZATION,
                        header::ACCEPT,
                    ])
                    .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
                    .allowed_header(header::CONTENT_TYPE)
                    .finish(),
            )
            .wrap(actix_web::middleware::Logger::default())
            .wrap(middleware::authentication::Authentication)
            // is this really usefull?
            // .wrap_fn(|request, service| {
            //     service.call(request).map(|response| response)
            // })
            .service(
                scope("/auth")
                    .service(
                        resource("/signup")
                            .route(post().to(endpoints::users::signup)),
                    )
                    .service(
                        resource("/login")
                            .route(post().to(endpoints::users::login)),
                    )
                    .service(
                        resource("/logout")
                            .route(post().to(endpoints::users::logout)),
                    ),
            )
            .service(
                scope("/persons")
                    .service(
                        resource("")
                            .route(get().to(endpoints::persons::find_all))
                            .route(post().to(endpoints::persons::create)),
                    )
                    .service(
                        resource("/{id}")
                            .route(get().to(endpoints::persons::find))
                            .route(put().to(endpoints::persons::update))
                            .route(delete().to(endpoints::persons::delete)),
                    ),
            )
            .service(
                scope("/ping")
                    .service(resource("").route(get().to(toolbox::ping::ping))),
            )
        // former routes:
        // .configure(config::routes::config_services)
    })
    .bind(bind_url)?
    .run()
    .await
}

fn get_bind_url_from_env() -> String {
    let host =
        env::var("HOST").expect("a HOST is not provided in the environment");
    let port =
        env::var("PORT").expect("a PORT is not provided in the environment");
    let bind_url = format!("{}:{}", host, port).to_string();
    info!("Listening to {}...", bind_url);
    bind_url
}
