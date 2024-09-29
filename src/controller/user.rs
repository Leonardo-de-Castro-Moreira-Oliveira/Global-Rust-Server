use crate::{model::UserModel, schema::UserSchema, AppState};
use actix_web::{
    delete, get, post,
    web::{Data, Json, Path},
    Responder,
};

#[get("/all")]
pub async fn get_all_users(data: Data<AppState>) -> impl Responder {
    crate::service::user::get_all_users(data).await
}

#[get("/one/{id}")]
pub async fn get_one_user(path: Path<String>, data: Data<AppState>) -> impl Responder {
    crate::service::user::find_one_user(path, data).await
}

#[get("/some/{name}")]
pub async fn get_some_users(path: Path<String>, data: Data<AppState>) -> impl Responder {
    crate::service::user::find_some_users(path, data).await
}

#[post("/manage")]
pub async fn post_one_user(body: Json<UserModel>, data: Data<AppState>) -> impl Responder {
    crate::service::user::add_one_user(body, data).await
}

#[delete("/manage")]
pub async fn delete_one_user(body: Json<UserSchema>, data: Data<AppState>) -> impl Responder {
    crate::service::user::delete_one_user(body, data).await
}
