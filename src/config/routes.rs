use crate::{endpoints, toolbox};
use actix_web::web::{delete, get, post, put, resource, scope, ServiceConfig};

pub fn config_services(cfg: &mut ServiceConfig) {
    info!("Configurating the routes...");
    cfg.service(
        scope("/auth")
            .service(resource("/signup").route(post().to(endpoints::users::signup)))
            .service(resource("/login").route(post().to(endpoints::users::login)))
            .service(resource("/logout").route(post().to(endpoints::users::logout))),
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
        scope("/ping").service(resource("").route(get().to(toolbox::ping::ping))),
    );
}

pub const IGNORE_ROUTES: [&str; 2] = ["/auth/signup", "/auth/login"];
