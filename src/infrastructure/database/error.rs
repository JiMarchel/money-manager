use thiserror::Error;

use crate::domain::persistence::error::RepositoryError;

#[derive(Debug, Error)]
pub enum DatabaseError {
    #[error("Database connection failed")]
    ConnectionFailed,

    #[error("Database query failed")]
    QueryFailed,

    #[error("Database constraint violation")]
    ConstraintViolation,

    #[error("Database operation timed out")]
    Timeout,
}

impl From<sqlx::Error> for DatabaseError {
    fn from(err: sqlx::Error) -> Self {
        match err {
            sqlx::Error::PoolTimedOut | sqlx::Error::PoolClosed | sqlx::Error::Io(_) => {
                DatabaseError::ConnectionFailed
            }

            sqlx::Error::Database(db_err) => match db_err.code().as_deref() {
                Some("23505") => DatabaseError::ConstraintViolation,
                _ => DatabaseError::QueryFailed,
            },

            _ => DatabaseError::QueryFailed,
        }
    }
}

impl From<DatabaseError> for RepositoryError {
    fn from(err: DatabaseError) -> Self {
        match err {
            DatabaseError::ConnectionFailed => RepositoryError::Unavailable,
            DatabaseError::ConstraintViolation => RepositoryError::Conflict,
            DatabaseError::Timeout => RepositoryError::Timeout,
            DatabaseError::QueryFailed => RepositoryError::Unknown,
        }
    }
}
