use actix_web::{
    web::{scope, Data, Json, Path},
    HttpResponse, Scope,
};
use serde_json::json;
use sqlx::{query_as, Result as SqlxResult};
use uuid::Uuid;

use crate::{model::UserModel, schema::UserSchema, AppState};

pub async fn get_all_users(data: Data<AppState>) -> SqlxResult<Vec<UserSchema>> {
    match query_as!(UserSchema, "SELECT * FROM rust_user")
        .fetch_all(&data.db)
        .await
    {
        Ok(users) => Ok(users), // Retorna a lista de usuários com sucesso
        Err(e) => Err(e),       // Propaga o erro se houver falha
    }
}

pub async fn find_one_user(path: Path<String>, data: Data<AppState>) -> HttpResponse {
    match Uuid::parse_str(&path) {
        Ok(uuid) => match query_as!(UserSchema, "SELECT * FROM rust_user WHERE id = $1", uuid)
            .fetch_one(&data.db)
            .await
        {
            Ok(user) => HttpResponse::Ok().json(json!({
                "status": "found",
                "user": user,
            })),
            Err(_) => HttpResponse::NotFound().json(json!({
                "status": "not found",
                "message": "This UUID doesn't exist in the database!",
            })),
        },
        Err(_) => HttpResponse::BadRequest().json(json!({
            "status": "bad request",
            "message": "Invalid UUID format, unable to search!",
        })),
    }
}

pub async fn find_some_users(path: Path<String>, data: Data<AppState>) -> HttpResponse {
    let search_pattern = format!("%{}%", path.to_string());

    match query_as!(
        UserSchema,
        "SELECT * FROM rust_user WHERE name LIKE $1",
        search_pattern
    )
    .fetch_all(&data.db)
    .await
    {
        Ok(users) => {
            if !users.is_empty() {
                HttpResponse::Ok().json(json!({
                    "status": "success",
                    "users": users,
                }))
            } else {
                HttpResponse::NotFound().json(json!({
                    "status": "not found",
                    "message": "Have no users like this name!"
                }))
            }
        }
        Err(_) => HttpResponse::InternalServerError().json(json!({
            "status": "error",
            "message": "Error while fetching users from the database."
        })),
    }
}

pub async fn add_one_user(body: Json<UserModel>, data: Data<AppState>) -> HttpResponse {
    // Validação do tamanho do nome e da senha
    if body.name.len() < 1 || body.password.len() < 8 {
        return HttpResponse::NotAcceptable().json(json!({
            "status": "not acceptable",
            "message": "The name cannot be empty and the password should be bigger than 7 characters!"
        }));
    }

    match query_as!(
        UserSchema,
        "INSERT INTO rust_user (name, password) VALUES ($1, $2) RETURNING *",
        body.name,
        body.password
    )
    .fetch_one(&data.db)
    .await
    {
        Ok(user) => HttpResponse::Created().json(json!({
            "status": "success",
            "user": user
        })),
        Err(err) => HttpResponse::InternalServerError().json(json!({
            "status": "error",
            "message": format!("Error: {:?}", err)
        })),
    }
}

pub fn get_scope() -> Scope {
    scope("/user")
        .service(crate::controller::user::get_all_users)
        .service(crate::controller::user::get_some_users)
        .service(crate::controller::user::get_one_user)
        .service(crate::controller::user::post_one_user)
}
