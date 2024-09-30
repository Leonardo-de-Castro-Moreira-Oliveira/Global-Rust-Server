pub mod user; // Exportando a lógica das rotas.

use actix_web::web::{scope, ServiceConfig};

// Função principal para definir os escopos das rotas.
pub fn config(conf: &mut ServiceConfig) {
    // Escope principal para a api.
    let scope = scope("/api").service(user::get_scope());
    conf.service(scope); // Definindo o escopo na configuração do serviço.
}
