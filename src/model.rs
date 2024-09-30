use serde::Deserialize;

#[derive(Deserialize)]
pub struct UserModel {
    pub name: String,
    pub password: String,
}
