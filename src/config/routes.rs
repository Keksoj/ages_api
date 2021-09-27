use crate::{controllers, toolbox};
use actix_web::web::{delete, get, post, put, resource, scope, ServiceConfig};

pub fn config_routes(cfg: &mut ServiceConfig) {
    info!("Configurating the routes...");
    cfg.service(
        scope("/auth")
            .service(resource("/signup").route(post().to(controllers::users::signup)))
            .service(resource("/login").route(post().to(controllers::users::login)))
            .service(resource("/update").route(put().to(controllers::users::update)))
            .service(resource("/delete").route(delete().to(controllers::users::delete))),
    )
    .service(
        scope("/persons")
            .service(
                resource("")
                    .route(get().to(controllers::persons::find_all))
                    .route(post().to(controllers::persons::create))
                    .route(put().to(controllers::persons::update)),
            )
            .service(
                resource("/{id}")
                    .route(get().to(controllers::persons::find))
                    .route(delete().to(controllers::persons::delete)),
            ),
    )
    .service(scope("/ping").service(resource("").route(get().to(toolbox::ping::ping))));
    // the "/documentation" route is served in main
    info!("Routes are configured!")
}

pub const IGNORE_ROUTES: [&str; 4] = ["/auth/signup", "/auth/login", "/ping", "/documentation"];
