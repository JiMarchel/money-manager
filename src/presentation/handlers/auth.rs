use axum::{Json, extract::State};
use validator::Validate;

use crate::{
    presentation::{dto::auth::RegisterRequest, error::AppError, state::AppState},
    shared::response::ApiResponse,
};

#[utoipa::path(
    post,
    path = "/api/auth/register",
    tag = "Auth",
    request_body = RegisterRequest,
    responses(
        (status = 200, description = "User registered successfully"),
        (status = 400, description = "Validation or Bad Request", body = crate::shared::response::ApiErrorResponse),
        (status = 409, description = "Conflict - Email already exists", body = crate::shared::response::ApiErrorResponse)
    )
)]
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
