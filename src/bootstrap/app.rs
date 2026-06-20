use crate::{
    config::app::AppConfig,
    infrastructure::{database::pool::create_pool, repositories::user::PostgresUserRepository},
};

pub async fn run() -> Result<(), Box<dyn std::error::Error>> {
    let config = AppConfig::from_env();

    let pool = create_pool(&config.database).await?;

    let user_repo = PostgresUserRepository::new(pool.clone());

    Ok(())
}
