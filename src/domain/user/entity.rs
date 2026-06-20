use serde::Serialize;

#[derive(Serialize, sqlx::FromRow)]
pub struct User {
    pub id: uuid::Uuid,
    pub email: String,
    pub username: String,
    pub password_hash: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}
