use anyhow::Result;
use sqlx::PgPool;
use uuid::Uuid;
use crate::models::{User, CreateUser};
use crate::auth::hash_password;

pub struct UserRepository {
    pool: PgPool,
}

impl UserRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn create_user(&self, data: &CreateUser) -> Result<User> {
        let password_hash = hash_password(&data.password)?;
        
        let user = sqlx::query_as!(
            User,
            r#"
            INSERT INTO users (username, email, password_hash, display_name)
            VALUES ($1, $2, $3, $4)
            RETURNING *
            "#,
            data.username,
            data.email,
            password_hash,
            data.display_name
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(user)
    }

    pub async fn find_by_username(&self, username: &str) -> Result<Option<User>> {
        let user = sqlx::query_as!(
            User,
            "SELECT * FROM users WHERE username = $1 AND is_active = true",
            username
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(user)
    }

    pub async fn find_by_email(&self, email: &str) -> Result<Option<User>> {
        let user = sqlx::query_as!(
            User,
            "SELECT * FROM users WHERE email = $1 AND is_active = true",
            email
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(user)
    }

    pub async fn find_by_id(&self, id: Uuid) -> Result<Option<User>> {
        let user = sqlx::query_as!(
            User,
            "SELECT * FROM users WHERE id = $1 AND is_active = true",
            id
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(user)
    }
}
