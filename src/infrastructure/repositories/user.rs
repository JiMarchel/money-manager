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

#[derive(Clone)]
pub struct PostgresUserRepository {
    pool: PgPool,
}

impl PostgresUserRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

impl UserRepository for PostgresUserRepository {
    #[tracing::instrument(name = "user_repo_find_by_email", skip(self), fields(email = email))]
    async fn find_by_email(&self, email: &str) -> RepositoryResult<Option<User>> {
        let row = sqlx::query_as!(User, "SELECT * FROM users WHERE email = $1", email)
            .fetch_optional(&self.pool)
            .await
            .map_err(DatabaseError::from)?;

        Ok(row)
    }

    #[tracing::instrument(name = "user_repo_save", skip(self, user))]
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
