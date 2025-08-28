use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;
use chrono::{DateTime, Utc};
use validator::Validate;

#[derive(Debug, Serialize, Deserialize, Clone, FromRow)]
pub struct Post {
    pub id: Uuid,
    pub user_id: Uuid,
    pub content: String,
    pub image_url: Option<String>,
    pub likes_count: i32,
    pub comments_count: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PostWithUser {
    pub id: Uuid,
    pub user_id: Uuid,
    pub username: String,
    pub display_name: Option<String>,
    pub avatar_url: Option<String>,
    pub content: String,
    pub image_url: Option<String>,
    pub likes_count: i32,
    pub comments_count: i32,
    pub created_at: DateTime<Utc>,
    pub is_liked: Option<bool>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct CreatePost {
    #[validate(length(min = 1, max = 500))]
    pub content: String,
    pub image_url: Option<String>,
}
