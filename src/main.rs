mod controller;
mod create;
mod model;
mod response;
mod schema;
mod service;

use dotenv::dotenv;
use sqlx::{Pool, Postgres};
use std::io::Result as IoResult;

// Estrutura para o banco de dados do Actix.
pub struct AppState {
    db: Pool<Postgres>,
}

#[actix::main]
async fn main() -> IoResult<()> {
    println!("$ Starting the server...");

    dotenv().ok();

    println!("\n# Loading the environment variables!");
    let addrs = &create::venvs("ADDRESS");
    let database_url = &create::venvs("DATABASE_URL");
    let max_connections = create::venv::<u32>("MAX_CONNECTIONS");

    println!("\n# Connecting to the database!");
    let pool = create::connection(database_url, max_connections).await;

    println!("\n# Creating the http server!");
    let server = create::server(addrs, pool);

    println!("$ Server started successfully.");
    println!("$ Access http://{} to verify the server.", addrs);

    return server.await;
}
