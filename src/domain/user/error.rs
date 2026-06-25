use thiserror::Error;

#[derive(Error, Debug)]
pub enum UserDomainError {
    #[error("Invalid email: {0}")]
    InvalidEmail(String),

    #[error("Invalid username: {0}")]
    InvalidUsername(String),
}