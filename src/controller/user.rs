use crate::{model, schema, AppState};
use actix_web::{
    delete, get, post,
    web::{Data, Json, Path},
    Responder,
};

#[get("/all")] // Rota GET para obter todos os usuários.
pub async fn get_all_users(data: Data<AppState>) -> impl Responder {
    crate::service::user::get_all_users(data).await
}

#[get("/one/{id}")] // Rota GET para obter uma pesquisa de usuário via Uuid.
pub async fn get_one_user(path: Path<String>, data: Data<AppState>) -> impl Responder {
    crate::service::user::find_user_by_id(path, data).await
}

#[get("/some/{name}")] // Rota GET para obter uma pesquisa de assemelhação de usuários via name.
pub async fn get_some_users(path: Path<String>, data: Data<AppState>) -> impl Responder {
    crate::service::user::find_users_by_name(path, data).await
}

#[post("/manage")] // Rota POST para adicionar usuário via JSON UserModel no body.
pub async fn post_one_user(body: Json<model::User>, data: Data<AppState>) -> impl Responder {
    crate::service::user::add_user_by_model(body, data).await
}

#[delete("/manage")] // Rota POST para remover usuário via autenticação em JSON UserSchema em body.
pub async fn delete_one_user(body: Json<schema::User>, data: Data<AppState>) -> impl Responder {
    crate::service::user::delete_user_by_schema(body, data).await
}
