pub mod user;
pub mod post;
pub mod chat;

pub use user::*;
pub use post::*;
pub use chat::*;

use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

// Respuesta API estándar
#[derive(Debug, Serialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: Option<T>,
    pub message: String,
}

impl<T> ApiResponse<T> {
    pub fn success(data: T, message: &str) -> Self {
        Self {
            success: true,
            data: Some(data),
            message: message.to_string(),
        }
    }

    pub fn error(message: &str) -> Self {
        Self {
            success: false,
            data: None,
            message: message.to_string(),
        }
    }
}

// Token de respuesta para autenticación
#[derive(Debug, Serialize)]
pub struct AuthResponse {
    pub token: String,
    pub user: UserProfile,
}
