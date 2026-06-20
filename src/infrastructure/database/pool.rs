use sqlx::{PgPool, postgres::PgPoolOptions};

use crate::{config::database::DatabaseConfig, infrastructure::database::error::DatabaseError};

pub async fn create_pool(config: &DatabaseConfig) -> Result<PgPool, DatabaseError> {
    PgPoolOptions::new()
        .max_connections(10)
        .connect(&config.connection_string())
        .await
        .map_err(|_| DatabaseError::ConnectionFailed)
}
