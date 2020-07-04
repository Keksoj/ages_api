use crate::{config, middleware};
use actix_cors::Cors;
use actix_service::Service;
use actix_web::{http::header, App};

// #[actix_rt::main]
pub fn build_the_app() -> App<T, B> {
    let app = App::new()
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
        // .wrap_fn(|request, service| service.call(request).map(|response| response))
        .configure(config::routes::config_services);
    app
}
