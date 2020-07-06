use crate::{endpoints, toolbox};
use actix_web::web::{delete, get, post, put, resource, scope, ServiceConfig};

pub fn config_routes(cfg: &mut ServiceConfig) {
    info!("Configurating the routes...");
    cfg.service(
        scope("/auth")
            .service(resource("/signup").route(post().to(endpoints::users::signup)))
            .service(resource("/login").route(post().to(endpoints::users::login)))
            .service(resource("/delete").route(delete().to(endpoints::users::delete))),
    )
    .service(
        scope("/persons")
            .service(
                resource("")
                    .route(get().to(endpoints::persons::find_all))
                    .route(post().to(endpoints::persons::create))
                    .route(put().to(endpoints::persons::update)),
            )
            .service(
                resource("/{id}")
                    .route(get().to(endpoints::persons::find))
                    .route(delete().to(endpoints::persons::delete)),
            ),
    )
    .service(scope("/ping").service(resource("").route(get().to(toolbox::ping::ping))));
}

pub const IGNORE_ROUTES: [&str; 4] = ["/auth/signup", "/auth/login", "/ping", "/documentation"];
