use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;
use chrono::{DateTime, Utc};
use validator::Validate;

#[derive(Debug, Serialize, Deserialize, Clone, FromRow)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    pub password_hash: String,
    pub display_name: Option<String>,
    pub bio: Option<String>,
    pub avatar_url: Option<String>,
    pub followers_count: i32,
    pub following_count: i32,
    pub posts_count: i32,
    pub is_verified: bool,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UserProfile {
    pub id: Uuid,
    pub username: String,
    pub display_name: Option<String>,
    pub bio: Option<String>,
    pub avatar_url: Option<String>,
    pub followers_count: i32,
    pub following_count: i32,
    pub posts_count: i32,
    pub is_verified: bool,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct CreateUser {
    #[validate(length(min = 3, max = 30))]
    pub username: String,
    #[validate(email)]
    pub email: String,
    #[validate(length(min = 8))]
    pub password: String,
    #[validate(length(max = 100))]
    pub display_name: Option<String>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct LoginUser {
    pub username: String,
    pub password: String,
}

impl From<User> for UserProfile {
    fn from(user: User) -> Self {
        Self {
            id: user.id,
            username: user.username,
            display_name: user.display_name,
            bio: user.bio,
            avatar_url: user.avatar_url,
            followers_count: user.followers_count,
            following_count: user.following_count,
            posts_count: user.posts_count,
            is_verified: user.is_verified,
            created_at: user.created_at,
        }
    }
}
