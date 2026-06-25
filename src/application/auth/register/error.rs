use thiserror::Error;

use crate::domain::{
    crypto::error::PasswordHasherError, persistence::error::RepositoryError,
    user::error::UserDomainError,
};

#[derive(Debug, Error)]
pub enum RegisterError {
    #[error("Email already exists.")]
    EmailAlreadyExists,

    #[error("Invalid email: {0}")]
    InvalidEmail(String),

    #[error("Invalid Username: {0}")]
    InvalidUsername(String),

    #[error("Password too weak.")]
    WeakPassword,

    #[error("Repository failed.")]
    Repository(RepositoryError),

    #[error("Hash failed.")]
    PasswordHash(PasswordHasherError),
}

impl From<RepositoryError> for RegisterError {
    fn from(err: RepositoryError) -> Self {
        Self::Repository(err)
    }
}

impl From<PasswordHasherError> for RegisterError {
    fn from(err: PasswordHasherError) -> Self {
        Self::PasswordHash(err)
    }
}

impl From<UserDomainError> for RegisterError {
    fn from(err: UserDomainError) -> Self {
        match err {
            UserDomainError::InvalidEmail(msg) => Self::InvalidEmail(msg),
            UserDomainError::InvalidUsername(msg) => Self::InvalidUsername(msg),
        }
    }
}
