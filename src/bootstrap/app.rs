use std::{net::SocketAddr, sync::Arc};

use anyhow::Result;
use axum::Router;
use tokio::net::TcpListener;

use crate::{
    application::auth::register::usecase::RegisterUseCase,
    config::app::AppConfig,
    infrastructure::{
        crypto::hasher::Argon2Hasher, database::pool::create_pool,
        repositories::user::PostgresUserRepository,
    },
    presentation::{routes::auth, state::AppState},
};

pub async fn run() -> Result<()> {
    //config
    let config = AppConfig::from_env();

    //database
    let pool = create_pool(&config.database).await?;

    //repositories
    let user_repo = Arc::new(PostgresUserRepository::new(pool.clone()));

    //services
    let hasher = Arc::new(Argon2Hasher::new());

    //use case
    let register_usecase = Arc::new(RegisterUseCase::new(user_repo, hasher));

    //state
    let state = AppState { register_usecase };

    //router
    let app = Router::new()
        .nest("/api/auth", auth::router())
        .with_state(state)
        .merge(crate::presentation::docs::openapi::router());

    //server
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));

    let listener = TcpListener::bind(addr).await?;

    println!("Listening on {}", addr);

    axum::serve(listener, app).await?;

    Ok(())
}
