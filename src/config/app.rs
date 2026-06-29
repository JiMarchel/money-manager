use std::env;

use crate::config::{database::DatabaseConfig, error::ConfigError};

pub enum AppEnvironment {
    Development,
    Production,
}

impl AppEnvironment {
    fn from_str(env: &str) -> Self {
        match env.to_lowercase().as_str() {
            "production" | "prod" => AppEnvironment::Production,
            _ => AppEnvironment::Development,
        }
    }
}

pub struct AppConfig {
    pub database: DatabaseConfig,
    pub app_env: AppEnvironment,
}

fn get_env(key: &str) -> Result<String, ConfigError> {
    env::var(key).map_err(|e| ConfigError::MissingEnv(e.to_string()))
}

impl AppConfig {
    pub fn from_env() -> Self {
        let app_env_str = env::var("APP_ENV").unwrap_or_else(|_| "development".to_string());
        Self {
            database: DatabaseConfig {
                username: get_env("POSTGRES_USER").expect("POSTGRES_USER not found"),
                database_name: get_env("POSTGRES_DB_NAME").expect("POSTGRES_DB_NAME not found"),
                password: get_env("POSTGRES_PASSWORD").expect("POSTGRES_PASSWORD not found"),
                host: get_env("DB_HOST").expect("DB_HOST not found"),
                port: get_env("DB_PORT")
                    .expect("DB_PORT not found")
                    .parse()
                    .unwrap(),
            },
            app_env: AppEnvironment::from_str(&app_env_str),
        }
    }
}
