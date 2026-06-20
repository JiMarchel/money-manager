use thiserror::Error;

#[derive(Debug,Error)]
pub enum ConfigError {
    #[error("Missing environtment variable: {0}")]
    MissingEnv(String)
}