use serde::Deserialize;
use uuid::Uuid;

#[derive(Deserialize)]
pub struct Message {
    pub user_id: Uuid,
    pub password: String,
    pub content: String,
}
