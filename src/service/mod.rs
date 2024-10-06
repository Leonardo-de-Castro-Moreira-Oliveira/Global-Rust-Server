pub mod message; // Exportando a lógica das rotas sociais.
pub mod user; // Exportando a lógica das rotas de usuário.

use actix_web::web::{scope, ServiceConfig};

// Função principal para definir os escopos das rotas.
pub fn config(conf: &mut ServiceConfig) {
    // Escope principal para a api.
    conf.service(
        scope("/api")
            .service(user::get_scope())
            .service(message::get_scope()),
    );
}
