use crate::{model::UserModel, AppState};
use actix_web::{
    get, post,
    web::{Data, Json, Path},
    HttpResponse, Responder,
};
use serde_json::json;

#[get("/all")]
pub async fn get_all_users(data: Data<AppState>) -> impl Responder {
    match crate::service::user::get_all_users(data).await {
        Ok(users) => HttpResponse::Ok().json(json!({
            "status": "success",
            "users": users
        })),
        Err(err) => HttpResponse::InternalServerError().json(json!({
            "status": "error",
            "message": format!("{:?}", err)
        })),
    }
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
