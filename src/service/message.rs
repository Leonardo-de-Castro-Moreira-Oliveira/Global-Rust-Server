use actix_web::{web::scope, Scope};

// Método utilizado no escopo principal para obter o escopo "tech".
pub fn get_scope() -> Scope {
    scope("/tech")
}
