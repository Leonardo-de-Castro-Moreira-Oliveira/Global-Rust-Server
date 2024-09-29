use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone)]
pub struct UserModel {
    pub name: String,
    pub password: String,
}
