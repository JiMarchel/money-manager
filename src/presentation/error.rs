use axum::{Json, http::StatusCode, response::IntoResponse};
use serde::Serialize;
use validator::ValidationErrors;

use crate::{
    application::auth::register::error::RegisterError,
    shared::response::{ApiErrorResponse, ErrorDetail},
};

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

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        let (status, message, details) = match self {
            AppError::BadRequest { message, details } => {
                (StatusCode::BAD_REQUEST, message, details)
            }
            AppError::Conflict { message } => (StatusCode::CONFLICT, message, None),

            AppError::Internal => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Internal server error".to_string(),
                None,
            ),
        };

        let body = ApiErrorResponse {
            error: ErrorDetail { message, details },
            request_id: None,
        };

        (status, Json(body)).into_response()
    }
}
