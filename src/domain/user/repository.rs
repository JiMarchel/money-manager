use async_trait::async_trait;

use crate::domain::{
    persistence::error::RepositoryResult,
    user::{entity::User},
};

#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn find_by_email(&self, email: &str) -> RepositoryResult<Option<User>>;
    async fn save(&self, user: &User) -> RepositoryResult<()>;
}
