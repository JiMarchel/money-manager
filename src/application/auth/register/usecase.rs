use std::sync::Arc;

use crate::{
    application::auth::register::{command::RegisterCommand, error::RegisterError},
    domain::{
        crypto::hasher::PasswordHasher,
        user::{
            entity::NewUser,
            repository::UserRepository,
            value_object::{Email, Username},
        },
    },
};

pub struct RegisterUseCase {
    user_repo: Arc<dyn UserRepository>,
    hasher: Arc<dyn PasswordHasher>,
}

impl RegisterUseCase {
    pub fn new(user_repo: Arc<dyn UserRepository>, hasher: Arc<dyn PasswordHasher>) -> Self {
        Self { user_repo, hasher }
    }

    pub async fn execute(&self, command: RegisterCommand) -> Result<(), RegisterError> {
        let email = Email::new(command.email)?;
        let username = Username::new(command.username)?;

        let exists = self
            .user_repo
            .find_by_email(email.as_ref())
            .await?
            .is_some();

        if exists {
            return Err(RegisterError::EmailAlreadyExists);
        }

        let password_hash = self.hasher.hash(&command.password)?;

        let user = NewUser {
            email,
            username,
            password_hash,
        };

        self.user_repo.save(&user).await?;

        Ok(())
    }
}
