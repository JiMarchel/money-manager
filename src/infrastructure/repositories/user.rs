use async_trait::async_trait;
use sqlx::PgPool;

use crate::{
    domain::{
        persistence::error::RepositoryResult,
        user::{entity::User, error::UserDomainError, repository::UserRepository},
    },
    infrastructure::database::error::DatabaseError,
};

pub struct PostgresUserRepository {
    pool: PgPool,
}

impl PostgresUserRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl UserRepository for PostgresUserRepository {
    async fn find_by_email(&self, email: &str) -> RepositoryResult<Option<User>> {
        let row = sqlx::query_as!(User, "SELECT * FROM users WHERE email = $1", email)
            .fetch_optional(&self.pool)
            .await
            .map_err(DatabaseError::from)?;

        Ok(row)
    }

    async fn save(&self, user: &User) -> RepositoryResult<()> {
        sqlx::query!(
            r#"
            INSERT INTO users(id, email)
            VALUES($1, $2)
            "#,
            user.id,
            user.email,
        )
        .execute(&self.pool)
        .await
        .map_err(DatabaseError::from)?;

        Ok(())
    }
}
