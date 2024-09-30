use actix_web::{web::Data, App, HttpServer};
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use std::io::Result as IoResult;
use std::{env::var, process::exit, str::FromStr};

use crate::{service, AppState};

// Método para obter variável de ambiente em forma de String.
pub fn venvs(key: &str) -> String {
    match var(key) {
        Ok(value) => {
            println!("@ENV {} loaded successfully.", key); // Propagando um log de suceso.
            value // Retornando a variável.
        }
        Err(_) => {
            println!("@ENV {} must be set!", key); // Propagando um log de erro.
            exit(1) // Finalizando a execução.
        }
    }
}

// Método para obter variável de ambiente em forma de um tipo genérico.
pub fn venv<T: FromStr>(key: &str) -> T {
    match var(key) {
        Ok(var_value) => {
            println!("@ENV {} loaded successfully.", key); // Propagando um log de sucesso.
            match var_value.trim().parse::<T>() {
                Ok(value) => value, // Retornando a variável.
                Err(_) => {
                    println!("@ENV {} cannot be parsed to the expected type!", key); // Propagando um log de erro.
                    exit(1) // Finalizando a execução.
                }
            }
        }
        Err(_) => {
            println!("@ENV {} must be set!", key); // Propagando um log de erro.
            exit(1) // Finalizando a execução.
        }
    }
}

// Método para aguardar uma conexão com o PostgresSQL
pub async fn connection(database_url: &str, max_connections: u32) -> Pool<Postgres> {
    match PgPoolOptions::new()
        .max_connections(max_connections) // Definindo o maximo de conexões simultâneas.
        .connect(database_url) // Conectando via DATABASE-URL.
        .await // Aguardando a operação.
    {
        Ok(pool) => {
            println!("$ Connection DB resolved"); // Propagando um log de sucesso.
            pool // Retornando a conexão.
        }
        Err(err) => {
            println!("$ Failed to connect to the database: {:?}", err); // Propagando um log de erro.
            exit(1) // Finalizando execução.
        }
    }
}

// Método para aguardar a inicialização de um server.
pub async fn server(addrs: &str, pool: Pool<Postgres>) -> IoResult<()> {
    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(AppState { db: pool.clone() })) // Definindo o banco de dados.
            .configure(service::config) // Definindo a configuração do serviços.
    })
    .bind(addrs)? // Adicionando a host.
    .run() // Inicializando o server.
    .await // Aguardando o client.
}
