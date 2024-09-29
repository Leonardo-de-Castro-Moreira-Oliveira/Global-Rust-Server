use actix_web::{web::Data, App, HttpServer};
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use std::io::Result as IoResult;
use std::{env::var, process::exit, str::FromStr};

use crate::{service, AppState};

pub fn venvs(key: &str) -> String {
    match var(key) {
        Ok(value) => {
            println!("@ENV {} loaded successfully.", key);
            value
        }
        Err(_) => {
            println!("@ENV {} must be set!", key);
            exit(1)
        }
    }
}

pub fn venv<T: FromStr>(key: &str) -> T {
    match var(key) {
        Ok(var_value) => {
            println!("@ENV {} loaded successfully.", key);
            match var_value.trim().parse::<T>() {
                Ok(value) => value,
                Err(_) => {
                    println!("@ENV {} cannot be parsed to the expected type!", key);
                    exit(1);
                }
            }
        }
        Err(_) => {
            println!("@ENV {} must be set!", key);
            exit(1);
        }
    }
}

pub async fn connection(database_url: &str, max_connections: u32) -> Pool<Postgres> {
    match PgPoolOptions::new()
        .max_connections(max_connections)
        .connect(database_url)
        .await
    {
        Ok(pool) => {
            println!("$ Connection DB resolved");
            pool
        }
        Err(err) => {
            println!("$ Failed to connect to the database: {:?}", err);
            exit(1)
        }
    }
}

pub async fn server(addrs: &str, pool: Pool<Postgres>) -> IoResult<()> {
    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(AppState { db: pool.clone() }))
            .configure(service::config)
    })
    .bind(addrs)?
    .run()
    .await
}
