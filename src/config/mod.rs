pub mod db;
pub mod routes;

use anyhow::Context;
use std::env;
use actix_web::http::Method;

#[derive(Clone)]
pub struct Config {
    pub database_url: String,
    pub bind_url: String,
    pub allowed_origin: String,
    pub allowed_methods: Vec<Method>,
}

impl Config {
    pub fn get_from_env() -> anyhow::Result<Self> {
        let host = env::var("HOST").context("a HOST is not provided in the environment")?;
        let port = env::var("PORT").context("a PORT is not provided in the environment")?;
        let bind_url = format!("{}:{}", host, port).to_string();
        
        let database_url = env::var("DATABASE_URL").context("Database url not set in env")?;
        
        let allowed_origin =
            env::var("ALLOWED_ORIGIN").context("allowed origin not set in env")?;

        let allowed_methods = vec![
            Method::GET,
            Method::POST,
            Method::PUT,
            Method::DELETE,
        ];

        Ok(Self {
            database_url,
            bind_url,
            allowed_origin,
            allowed_methods,
        })
    }
}
