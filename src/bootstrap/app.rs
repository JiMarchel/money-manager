use std::net::SocketAddr;

use anyhow::Result;
use axum::{Router, middleware};
use tokio::net::TcpListener;

use crate::{
    application::auth::register::usecase::RegisterUseCase,
    config::app::AppConfig,
    infrastructure::{
        crypto::hasher::Argon2Hasher, database::pool::create_pool,
        repositories::user::PostgresUserRepository,
    },
    presentation::{middleware::request_id::request_id, routes::auth, state::AppState},
    shared::logger::init_logging,
};

pub async fn run() -> Result<()> {
    //log
    init_logging();

    //config
    let config = AppConfig::from_env();

    //database
    let pool = create_pool(&config.database).await?;

    //use case
    let register_usecase = RegisterUseCase::new(
        PostgresUserRepository::new(pool.clone()),
        Argon2Hasher::new(),
    );

    //state
    let state = AppState { register_usecase };

    //router
    let app = Router::new()
        .nest("/api/auth", auth::router())
        .layer(middleware::from_fn(request_id))
        .with_state(state)
        .merge(crate::presentation::docs::openapi::router());

    //server
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));

    let listener = TcpListener::bind(addr).await?;

    println!("Listening on {}", addr);

    axum::serve(listener, app).await?;

    Ok(())
}
