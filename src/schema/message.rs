use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use uuid::Uuid;

#[derive(Serialize, Deserialize, FromRow)]
pub struct MessageSchema {
    pub id: Uuid,
    pub likes: u32,
    pub owner: Uuid,
    pub content: String,
    pub sended_at: DateTime<Utc>,
}
