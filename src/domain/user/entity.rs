use crate::domain::user::value_object::{Email, Username};

pub struct User {
    pub id: uuid::Uuid,
    pub email: String,
    pub username: String,
    pub password_hash: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

pub struct NewUser {
    pub email: Email,
    pub username: Username,
    pub password_hash: String,
}

impl NewUser {
    pub fn new(email: Email, username: Username, password_hash: String) -> Self {
        Self {
            email,
            username,
            password_hash,
        }
    }
}
