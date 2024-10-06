use actix_web::{
    web::{scope, Data},
    HttpResponse, Scope,
};
use sqlx::query_as;

use crate::{model, response, schema};

// Método para obter todas as mensagens da Tech-Connect.
pub async fn get_all_messages(data: Data<crate::AppState>) -> HttpResponse {
    match query_as!(schema::Message, "SELECT * FROM messages")
        .fetch_all(&data.db)
        .await
    {
        Ok(messages) => HttpResponse::Ok().json(response::Success::new("success", messages)), // Retornando a lista de mensagens.
        Err(err) => {
            HttpResponse::InternalServerError().json(response::ServerError::from_sqlx_error(err))
        } // Retornando erro inesperado pelo sqlx.
    }
}

// Método utilizado no escopo principal para obter o escopo "tech".
pub fn get_scope() -> Scope {
    // Logs de sucesso.
    println!("\n# Loading tech-connect route...");
    println!("@ROUTE('api/tech/all')                GET : Return all messages.");

    // Retornando o escopo.
    scope("/tech").service(crate::controller::message::get_all_messages) // Obter todos as mensagens    GET ("api/tech/all")
}
