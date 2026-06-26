use crate::domain::crypto::{
    error::{PasswordHasherError, PasswordHasherResult},
    hasher::PasswordHasher,
};
use argon2::{
    Argon2, PasswordHash, PasswordHasher as ArgonHasher, PasswordVerifier,
    password_hash::{SaltString, rand_core},
};

#[derive(Clone)]
pub struct Argon2Hasher {
    argon2: Argon2<'static>,
}

impl Argon2Hasher {
    pub fn new() -> Self {
        Self {
            argon2: Argon2::default(),
        }
    }
}

impl PasswordHasher for Argon2Hasher {
    fn hash(&self, password: &str) -> PasswordHasherResult<String> {
        let salt = SaltString::generate(&mut rand_core::OsRng);

        let hash_password = self
            .argon2
            .hash_password(password.as_bytes(), &salt)
            .map_err(PasswordHasherError::from)?
            .to_string();

        Ok(hash_password)
    }

    fn verify(&self, password: &str, hash: &str) -> PasswordHasherResult<bool> {
        let parsed_hash = PasswordHash::new(hash).map_err(PasswordHasherError::from)?;

        match self
            .argon2
            .verify_password(password.as_bytes(), &parsed_hash)
        {
            Ok(_) => Ok(true),
            Err(argon2::password_hash::Error::Password) => Ok(false),
            Err(e) => Err(PasswordHasherError::from(e)),
        }
    }
}

impl From<argon2::password_hash::Error> for PasswordHasherError {
    fn from(_: argon2::password_hash::Error) -> Self {
        PasswordHasherError::Failed
    }
}
