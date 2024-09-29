use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct ServerError {
    pub status: String,
    pub message: String,
    pub timestamp: DateTime<Utc>,
}

impl ServerError {
    pub fn new(status: &str, message: &str) -> Self {
        Self {
            status: status.to_string(),
            message: message.to_string(),
            timestamp: Utc::now(),
        }
    }

    pub fn from_sqlx_error(err: sqlx::Error) -> Self {
        Self {
            status: "internal server error".to_string(),
            message: err.to_string(),
            timestamp: Utc::now(),
        }
    }
}
