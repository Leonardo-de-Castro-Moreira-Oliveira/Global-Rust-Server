use chrono::{DateTime, Utc};
use serde::Serialize;

// Estrutura para retornar respostas de sucesso
// em alguma operação junto a um status, um
// valor generico e a hora da requisição.
#[derive(Serialize)]
pub struct Success<T> {
    pub status: String,
    pub response: T,
    pub timestamp: DateTime<Utc>,
}

impl<T> Success<T> {
    pub fn new(status: &str, response: T) -> Self {
        Self {
            status: status.to_string(), // Definindo o status do retorno esperado.
            response,                   // Definindo o valor de sucesso da resposta.
            timestamp: Utc::now(),      // Definindo a hora em que a requisição foi respondida.
        }
    }
}
