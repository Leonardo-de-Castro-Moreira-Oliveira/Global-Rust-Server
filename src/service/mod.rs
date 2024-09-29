pub mod user;

use actix_web::web::{scope, ServiceConfig};

pub fn config(conf: &mut ServiceConfig) {
    let scope = scope("/api").service(user::get_scope());
    conf.service(scope);
}
