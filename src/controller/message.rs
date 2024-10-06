use actix_web::{get, web::Data, Responder};

#[get("/all")] // Rota GET para obter todas as mensagens.
pub async fn get_all_messages(data: Data<crate::AppState>) -> impl Responder {
    crate::service::message::get_all_messages(data).await
}
