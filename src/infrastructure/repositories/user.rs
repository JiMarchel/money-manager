use async_trait::async_trait;
use sqlx::PgPool;

use crate::{
    domain::{
        persistence::error::RepositoryResult,
        user::{
            entity::{NewUser, User},
            repository::UserRepository,
        },
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

    async fn save(&self, user: &NewUser) -> RepositoryResult<()> {
        sqlx::query!(
            r#"
            INSERT INTO users(email, username, password_hash)
            VALUES($1, $2, $3)
            "#,
            user.email.as_ref(),
            user.username.as_ref(),
            user.password_hash
        )
        .execute(&self.pool)
        .await
        .map_err(DatabaseError::from)?;

        Ok(())
    }
}
