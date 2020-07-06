use std::env;
use actix_web::http::Method;

#[derive(Clone)]
pub struct AppEnv {
    pub database_url: String,
    pub bind_url: String,
    pub allowed_origin: String,
    pub allowed_methods: Vec<Method>,
}

impl AppEnv {
    pub fn establish() -> Self {
        let host = env::var("HOST").expect("a HOST is not provided in the environment");
        let port = env::var("PORT").expect("a PORT is not provided in the environment");
        let bind_url = format!("{}:{}", host, port).to_string();
        
        let database_url = env::var("DATABASE_URL").expect("Database url not set in env");
        
        let allowed_origin =
            env::var("ALLOWED_ORIGIN").expect("allowed origin not set in env");

        let allowed_methods = vec![
            Method::GET,
            Method::POST,
            Method::PUT,
            Method::DELETE,
        ];

        Self {
            database_url,
            bind_url,
            allowed_origin,
            allowed_methods,
        }
    }
}
