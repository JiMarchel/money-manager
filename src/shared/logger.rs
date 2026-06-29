use std::fmt::Debug;

use tracing::Level;

pub fn init_logging() {
    let env_filter = tracing_subscriber::EnvFilter::from_default_env();
    let app_env = std::env::var("APP_ENV").unwrap_or_else(|_| "development".to_string());

    if app_env.to_lowercase() == "production" || app_env.to_lowercase() == "prod" {
        tracing_subscriber::fmt()
            .json()
            .with_target(true)
            .with_file(true)
            .with_line_number(true)
            .with_env_filter(env_filter)
            .init();
    } else {
        tracing_subscriber::fmt()
            .with_target(true)
            .with_file(true)
            .with_line_number(true)
            .with_env_filter(env_filter)
            .init();
    }
}

pub trait LogPolicy {
    fn should_log(&self) -> bool;
    fn level(&self) -> Level;
}

pub fn log_error<E>(err: &E)
where
    E: Debug + LogPolicy,
{
    if !err.should_log() {
        return;
    }

    match err.level() {
        Level::ERROR => tracing::error!(error = ?err),
        Level::WARN => tracing::warn!(error = ?err),
        Level::INFO => tracing::info!(error = ?err),
        Level::DEBUG => tracing::debug!(error = ?err),
        Level::TRACE => tracing::trace!(error = ?err),
    }
}
