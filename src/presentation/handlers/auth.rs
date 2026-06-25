use axum::{Json, extract::State};
use validator::Validate;

use crate::{
    presentation::{dto::auth::RegisterRequest, error::AppError, state::AppState},
    shared::response::ApiResponse,
};

pub async fn register(
    State(state): State<AppState>,
    Json(req): Json<RegisterRequest>,
) -> Result<Json<ApiResponse<()>>, AppError> {
    req.validate()?;

    state.register_usecase.execute(req.into()).await?;

    Ok(Json(ApiResponse::success_with_message(
        (),
        "Register successfully.",
    )))
}
