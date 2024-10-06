use serde::Deserialize;
use uuid::Uuid;

#[derive(Deserialize)]
pub struct Message {
    pub user_id: Uuid,    // Id do usuário que está postando a mensagem
    pub password: String, // Senha do usuário (possivelmente para autenticação)
    pub content: String,  // Conteúdo da mensagem a ser postada
}
