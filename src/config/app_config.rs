use actix_web::http::Method;
use envconfig::Envconfig;
use log::Level;
use std::env;

#[derive(Clone, Envconfig, Debug)]
pub struct AppEnv {
    #[envconfig(from = "POSTGRESQL_ADDON_URI")]
    pub postgresql_uri: String,
    #[envconfig(from = "POSTGRESQL_PASSWORD")]
    pub postgresql_password: String,
    #[envconfig(from = "RUST_LOG")]
    pub log_level: Level,
}

#[derive(Clone, Debug)]
pub struct AppConfig {
    pub app_env: AppEnv,
    pub allowed_methods: Vec<Method>,
}

impl AppConfig {
    pub fn establish() -> Result<Self, anyhow::Error> {
        let app_env = AppEnv::init_from_env()?;
        let allowed_methods =
            vec![Method::GET, Method::POST, Method::PUT, Method::DELETE];

        Ok(Self {
            app_env,
            allowed_methods,
        })
    }

    pub fn get_pg_uri(&self) -> String {
        self.app_env.postgresql_uri.clone()
    }

    pub fn get_pg_password(&self) -> String {
        self.app_env.postgresql_password.clone()
    }

    pub fn get_allowed_methods(&self) -> Vec<Method> {
        self.allowed_methods.clone()
    }
}
