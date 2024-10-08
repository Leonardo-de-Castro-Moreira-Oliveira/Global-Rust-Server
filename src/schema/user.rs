use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use uuid::Uuid;

#[derive(Deserialize, Serialize, FromRow)]
pub struct User {
    pub id: Uuid,
    pub name: String,
    pub password: String,
}
