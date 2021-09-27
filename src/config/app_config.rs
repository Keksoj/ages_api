use actix_web::http::Method;
use envconfig::Envconfig;
use log::Level;
use std::net::SocketAddr;

#[derive(Clone, Envconfig, Debug)]
pub struct AppEnv {
    #[envconfig(from = "POSTGRESQL_ADDON_URI")]
    pub postgresql_uri: String,
    #[envconfig(from = "RUST_LOG", default = "debug")]
    pub log_level: Level,
    #[envconfig(from = "SOCKET_ADDRESS", default = "127.0.0.1:8080")]
    pub socket_address: String,
}

#[derive(Clone, Debug)]
pub struct AppConfig {
    pub postgresql_uri: String,
    pub log_level: Level,
    pub socket_address: SocketAddr,
    pub allowed_methods: Vec<Method>,
}

impl AppConfig {
    pub fn establish() -> Result<Self, anyhow::Error> {
        let app_env = AppEnv::init_from_env()?;

        Ok(Self {
            postgresql_uri: app_env.postgresql_uri,
            log_level: app_env.log_level,
            socket_address: app_env.socket_address.parse::<SocketAddr>()?,
            allowed_methods: vec![Method::GET, Method::POST, Method::PUT, Method::DELETE],
        })
    }
}
