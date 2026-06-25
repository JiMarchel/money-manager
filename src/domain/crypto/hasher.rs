use crate::domain::crypto::error::PasswordHasherResult;

pub trait PasswordHasher: Send + Sync {
    fn hash(&self, password: &str) -> PasswordHasherResult<String>;
    fn verify(&self, password: &str, hash: &str) -> PasswordHasherResult<bool>;
}
