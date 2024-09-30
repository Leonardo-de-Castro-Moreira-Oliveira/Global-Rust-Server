use chrono::{DateTime, Utc};
use serde::Serialize;

// Estrutura para retornar alguma resposta
// quando ouver erro,
#[derive(Serialize)]
pub struct ServerError {
    pub status: String,
    pub message: String,
    pub timestamp: DateTime<Utc>,
}

impl ServerError {
    pub fn new(status: &str, message: &str) -> Self {
        Self {
            status: status.to_string(),   // Definindo o status sugerido pelo client.
            message: message.to_string(), // Definindo a mensagem sugerida pelo client.
            timestamp: Utc::now(),        // Definindo a hora em que ocorreu o error.
        }
    }

    // Retorna um erro a partir de um error do sqlx
    // evitando aumentos no codigo de origem.
    pub fn from_sqlx_error(err: sqlx::Error) -> Self {
        Self {
            status: "internal server error".to_string(), // Definindo o titulo devido a incerteza sobre o erro.
            message: err.to_string(), // Definindo a mensagem do erro desconhecido.
            timestamp: Utc::now(),    // Definindo a hora em que ocorreu o erro.
        }
    }
}
