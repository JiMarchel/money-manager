use thiserror::Error;

#[derive(Error, Debug)]
pub enum UserDomainError {
    #[error("User not found with: {0}")]
    UserNotFound(String),

    #[error("Invalid email format: {0}")]
    InvalidEmail(String),
}