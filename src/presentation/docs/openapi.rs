use axum::{routing::get, Json, Router};
use scalar_api_reference::axum::router as scalar_router;
use serde_json::json;
use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    paths(
        crate::presentation::handlers::auth::register
    ),
    components(
        schemas(
            crate::presentation::dto::auth::RegisterRequest,
            crate::shared::response::ApiErrorResponse,
            crate::shared::response::ErrorDetail
        )
    ),
    tags(
        (name = "Auth", description = "Authentication API")
    )
)]
struct ApiDoc;

pub fn router() -> Router {
    let configuration = json!({
        "url": "/openapi.json",
    });

    Router::new()
        .route("/openapi.json", get(|| async { Json(ApiDoc::openapi()) }))
        .merge(scalar_router("/docs", &configuration))
}