use axum::{Router, routing::post};

use crate::presentation::{handlers::auth::register, state::AppState};

pub fn router() -> Router<AppState> {
    Router::new().route("/register", post(register))
}
