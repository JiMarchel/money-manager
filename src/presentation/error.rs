use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::Serialize;
use uuid::Uuid;
use validator::ValidationErrors;

use crate::{
    application::auth::register::error::RegisterError,
    shared::response::{ApiErrorResponse, ErrorDetail},
};

pub struct ApiError {
    pub request_id: Uuid,
    pub error: AppError,
}

pub enum AppError {
    BadRequest {
        message: String,
        details: Option<serde_json::Value>,
    },

    Conflict {
        message: String,
    },

    Internal,
}

#[derive(Serialize)]
pub struct ValidationFieldError {
    pub field: String,
    pub message: String,
}

impl From<ValidationErrors> for AppError {
    fn from(err: ValidationErrors) -> Self {
        let mut errors = Vec::new();

        for (field, field_errors) in err.field_errors() {
            for error in field_errors {
                let message = error
                    .message
                    .as_ref()
                    .map(|m| m.to_string())
                    .unwrap_or_else(|| error.code.to_string());

                errors.push(ValidationFieldError {
                    field: field.to_string(),
                    message,
                });
            }
        }

        AppError::BadRequest {
            message: "Validation failed".into(),
            details: Some(serde_json::to_value(errors).unwrap()),
        }
    }
}

impl From<RegisterError> for AppError {
    fn from(err: RegisterError) -> Self {
        match err {
            RegisterError::EmailAlreadyExists => AppError::Conflict {
                message: "Email already exists.".into(),
            },

            RegisterError::InvalidEmail(msg) => AppError::BadRequest {
                message: msg,
                details: None,
            },

            RegisterError::InvalidUsername(msg) => AppError::BadRequest {
                message: msg,
                details: None,
            },

            RegisterError::PasswordHash(_) => AppError::Internal,

            RegisterError::Repository(_) => AppError::Internal,

            RegisterError::WeakPassword => AppError::BadRequest {
                message: "Password too weak".into(),
                details: None,
            },
        }
    }
}

impl AppError {
    pub fn with_request_id(self, request_id: Uuid) -> ApiError {
        ApiError {
            request_id,
            error: self,
        }
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let (status, message, details) = match self.error {
            AppError::BadRequest { message, details } => {
                (StatusCode::BAD_REQUEST, message, details)
            }

            AppError::Conflict { message } => (StatusCode::CONFLICT, message, None),

            AppError::Internal => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Internal server error".into(),
                None,
            ),
        };

        let body = ApiErrorResponse {
            error: ErrorDetail { message, details },

            request_id: Some(self.request_id.to_string()),
        };

        (status, Json(body)).into_response()
    }
}
