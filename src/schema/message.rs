use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use uuid::Uuid;

#[derive(Serialize, Deserialize, FromRow)]
pub struct Message {
    pub id: Uuid,
    pub likes: i32,
    pub user_id: Uuid,
    pub content: String,
    pub sended_at: DateTime<Utc>,
}
