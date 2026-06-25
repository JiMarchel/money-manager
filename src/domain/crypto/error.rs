use thiserror::Error;

#[derive(Debug, Error)]
pub enum PasswordHasherError {
    #[error("Hashing password failed")]
    Failed,
}

pub type PasswordHasherResult<T> = Result<T, PasswordHasherError>;
