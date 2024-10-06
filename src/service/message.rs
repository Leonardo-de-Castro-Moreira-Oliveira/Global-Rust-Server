use actix_web::{
    web::{scope, Data, Path},
    HttpResponse, Scope,
};
use sqlx::query_as;
use uuid::Uuid;

use crate::{model, response, schema};

// Método para obter todas as mensagens da Tech-Connect
// retornando uma resposta HTTP contendo uma lista de schema::Message.
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

// Método para obter uma mensagem a partir do seu id (UUID)
// retornando um schema::Message.
// O erro ocorre quando o UUID não é valido ou quando nenhuma
// mensagem for encontrada.
pub async fn get_message_by_id(path: Path<String>, data: Data<crate::AppState>) -> HttpResponse {
    // Passando a variável de path para UUID.
    match Uuid::parse_str(&path.into_inner()) {
        Ok(uuid) => {
            match query_as!(
                schema::Message,
                "SELECT * FROM messages WHERE id = $1",
                uuid
            )
            .fetch_one(&data.db)
            .await
            {
                Ok(message) => HttpResponse::Ok().json(response::Success::new("found", message)), // Retornando o usuário que corresponde ao UUID.
                Err(_) => HttpResponse::NotFound().json(response::ServerError::new(
                    "not found",
                    "This UUID doesn't exist in the database!",
                )), // Retornando o erro no qual indica que a mensagem correspondente não foi encontrada.
            }
        }
        Err(_) => HttpResponse::BadRequest().json(response::ServerError::new(
            "bad request",
            "Invalid UUID format, unable to search!",
        )), // Retornando o erro que indca UUID inválido.,
    }
}

// Método para obter algumas mensagens a partir de conteúdo
// presente dentro da mensagem, retorna uma resposta HTTP
// contendo uma lista de mensagens.
// O erro ocorre quando não há nenhuma mensagem correspondente.
pub async fn get_messages_by_content(
    path: Path<String>,
    data: Data<crate::AppState>,
) -> HttpResponse {
    // Formatando o pattern para a pesquisa.
    let pattern = format!("%{}%", path.into_inner());

    match query_as!(
        schema::Message,
        "SELECT * FROM messages WHERE content LIKE $1",
        pattern
    )
    .fetch_all(&data.db)
    .await
    {
        Ok(messages) => {
            if messages.is_empty() {
                // Retornando not found quando nenhuma mensagem é
                // correspondida a determinado conteúdo
                HttpResponse::NotFound().json(response::ServerError::new(
                    "not found",
                    "Found no messages with this content!",
                ))
            } else {
                // Retornando a lista das mensagens correspondentes.
                HttpResponse::Ok().json(response::Success::new("found", messages))
            }
        }
        Err(err) => {
            // Retornando o erro inesperado pelo sqlx.
            HttpResponse::InternalServerError().json(response::ServerError::from_sqlx_error(err))
        }
    }
}

// Método para obter todas as mensagens de determinado
// usuário pelo id retornando uma resposta HTTP contendo
// uma lista de mensagens.
// O erro ocorre quando o UUID é inválido ou quando o
// usuario não existe.
pub async fn get_messages_from_user_id(
    path: Path<String>,
    data: Data<crate::AppState>,
) -> HttpResponse {
    // Convertendo o ID do caminho para UUID
    match Uuid::parse_str(&path.into_inner()) {
        Ok(uuid) => {
            // Verifica se o usuário com o UUID fornecido existe
            match query_as!(schema::User, "SELECT * FROM rust_user WHERE id = $1", uuid)
                .fetch_optional(&data.db)
                .await
            {
                Ok(Some(_user)) => {
                    // O usuário existe, agora buscar as mensagens
                    match query_as!(
                        schema::Message,
                        "SELECT * FROM messages WHERE user_id = $1",
                        uuid
                    )
                    .fetch_all(&data.db)
                    .await
                    {
                        Ok(messages) => {
                            // Retorna as mensagens do usuário
                            HttpResponse::Ok().json(response::Success::new("success", messages))
                        }
                        Err(err) => {
                            // Erro ao consultar as mensagens
                            HttpResponse::InternalServerError()
                                .json(response::ServerError::from_sqlx_error(err))
                        }
                    }
                }
                Ok(None) => {
                    // Erro quando o usuário não existe.
                    HttpResponse::NotFound().json(response::ServerError::new(
                        "not found",
                        "This user doesn't exist!",
                    ))
                }
                Err(err) => {
                    // Erro ao consultar o usuário no banco de dados
                    HttpResponse::InternalServerError()
                        .json(response::ServerError::from_sqlx_error(err))
                }
            }
        }
        Err(_) => {
            // Erro indicando que o UUID é inválido.
            HttpResponse::BadRequest().json(response::ServerError::new(
                "bad request",
                "Invalid UUID format, unable to search!",
            ))
        }
    }
}

// Método utilizado no escopo principal para obter o escopo "tech".
pub fn get_scope() -> Scope {
    scope("/tech")
        .service(crate::controller::message::get_all_messages) // Obter todos as mensagens.     GET ("api/tech/all")
        .service(crate::controller::message::get_one_message) // Obter uma única mensagem.      GET ("api/tech/one/{id}")
        .service(crate::controller::message::get_some_messages) // Obter algumas mensagens.     GET ("api/tech/some/{content}")
        .service(crate::controller::message::get_messages_from_user) // Obter do usuário.       GET ("api/tech/from/{user_id}")
}

// Logs da rota.
pub fn logs() {
    println!("\n# Loading tech-connect route...");
    println!("@ROUTE('api/tech/all')                GET     : Return all messages.");
    println!("@ROUTE('api/tech/one/{{id}}')           GET     : Return one message by id.");
    println!("@ROUTE('api/tech/some/{{content}}')     GET     : Return some messages by content.");
    println!("@ROUTE('api/tech/from/{{user_id}}')     GET     : Return some messages by user_id.");
}
