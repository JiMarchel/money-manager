use axum::{extract::FromRequestParts, http::request::Parts};

use crate::presentation::error::{ApiError, AppError};

#[derive(Clone, Debug)]
pub struct RequestContext {
    pub request_id: uuid::Uuid,
}

impl<S> FromRequestParts<S> for RequestContext
where
    S: Send + Sync,
{
    type Rejection = ApiError;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        parts
            .extensions
            .get::<RequestContext>()
            .cloned()
            .ok_or_else(|| AppError::Internal.with_request_id(uuid::Uuid::nil()))
    }
}
