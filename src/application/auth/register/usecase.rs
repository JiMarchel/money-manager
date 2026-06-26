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
    infrastructure::{crypto::hasher::Argon2Hasher, repositories::user::PostgresUserRepository},
};

#[derive(Clone)]
pub struct RegisterUseCase<U, H>
where
    U: UserRepository,
    H: PasswordHasher,
{
    user_repo: U,
    hasher: H,
}

pub type Register = RegisterUseCase<PostgresUserRepository, Argon2Hasher>;

impl<U: UserRepository, H: PasswordHasher> RegisterUseCase<U, H> {
    pub fn new(user_repo: U, hasher: H) -> Self {
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
