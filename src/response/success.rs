use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Success<T> {
    pub status: String,
    pub content: T,
    pub timestamp: DateTime<Utc>,
}

impl<T> Success<T> {
    pub fn new(status: &str, content: T) -> Self {
        Self {
            status: status.to_string(),
            content,
            timestamp: Utc::now(),
        }
    }
}
