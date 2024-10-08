use actix_web::{
    get, post,
    web::{Data, Json, Path},
    Responder,
};

use crate::model;

#[get("/all")] // Rota GET para obter todas as mensagens.
pub async fn get_all_messages(data: Data<crate::AppState>) -> impl Responder {
    crate::service::message::get_all_messages(data).await
}

#[get("/one/{id}")] // Rota GET para obter uma mensagem pelo id.
pub async fn get_one_message(path: Path<String>, data: Data<crate::AppState>) -> impl Responder {
    crate::service::message::get_message_by_id(path, data).await
}

#[get("/some/{content}")] // Rota GET para obter algumas mensagens pelo conteudo.
pub async fn get_some_messages(path: Path<String>, data: Data<crate::AppState>) -> impl Responder {
    crate::service::message::get_messages_by_content(path, data).await
}

#[get("/from/{user_id}")] // Rota GET para obter todas as mensagens de um usuário.
pub async fn get_messages_from_user(
    path: Path<String>,
    data: Data<crate::AppState>,
) -> impl Responder {
    crate::service::message::get_messages_from_user_id(path, data).await
}

#[post("/manage")] // Rota POST para asdicionar uma mensagem com autênticação.
pub async fn post_message(
    body: Json<model::Message>,
    data: Data<crate::AppState>,
) -> impl Responder {
    crate::service::message::add_message_by_model(body, data).await
}
