use serde::Deserialize;
use validator::Validate;

use crate::application::auth::register::command::RegisterCommand;

#[derive(Debug, Deserialize, Validate)]
pub struct RegisterRequest {
    #[validate(
        email(message = "Invalid email format"),
        length(min = 3, max = 255, message = "Email must be between 3 and 255 characters")
    )]
    pub email: String,

    #[validate(length(min = 3, max = 255, message = "Username must be between 3 and 255 characters"))]
    pub username: String,

    #[validate(length(min = 3, max = 255, message = "Password must be between 3 and 255 characters"))]
    pub password: String,
}

impl From<RegisterRequest> for RegisterCommand {
    fn from(req: RegisterRequest) -> Self {
        Self {
            email: req.email,
            username: req.username,
            password: req.password,
        }
    }
}