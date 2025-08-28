use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Message {
    pub id: Uuid,
    pub sender_id: Uuid,
    pub receiver_id: Uuid,
    pub content: String,
    pub is_read: bool,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct SendMessage {
    pub receiver_id: Uuid,
    pub content: String,
}
