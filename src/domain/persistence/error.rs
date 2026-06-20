use thiserror::Error;

/// Errors exposed by repository abstractions.
///
/// The domain knows that persistence failed,
/// but it does NOT know whether the implementation
/// uses PostgreSQL, Redis, MongoDB, etc.
#[derive(Debug, Error)]
pub enum RepositoryError {
    #[error("Persistence service is unavailable")]
    Unavailable,

    #[error("Resource already exists")]
    Conflict,

    #[error("Persistence operation timed out")]
    Timeout,

    #[error("Unexpected persistence error")]
    Unknown,
}

pub type RepositoryResult<T> = Result<T, RepositoryError>;