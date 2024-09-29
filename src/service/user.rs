use actix_web::{
    web::{scope, Data, Json, Path},
    HttpResponse, Scope,
};
use sqlx::{query, query_as};
use uuid::Uuid;

use crate::{
    model::UserModel,
    response::{ServerError, Success},
    schema::UserSchema,
    AppState,
};

pub async fn get_all_users(data: Data<AppState>) -> HttpResponse {
    match query_as!(UserSchema, "SELECT * FROM rust_user")
        .fetch_all(&data.db)
        .await
    {
        Ok(users) => HttpResponse::Ok().json(Success::new("success", users)),
        Err(err) => HttpResponse::InternalServerError().json(ServerError::from_sqlx_error(err)),
    }
}

pub async fn find_one_user(path: Path<String>, data: Data<AppState>) -> HttpResponse {
    match Uuid::parse_str(&path) {
        Ok(uuid) => match query_as!(UserSchema, "SELECT * FROM rust_user WHERE id = $1", uuid)
            .fetch_one(&data.db)
            .await
        {
            Ok(user) => HttpResponse::Ok().json(Success::new("found", user)),
            Err(_) => HttpResponse::NotFound().json(ServerError::new(
                "not found",
                "This UUID doesn't exist in the database!",
            )),
        },
        Err(_) => HttpResponse::BadRequest().json(ServerError::new(
            "bad request",
            "Invalid UUID format, unable to search!",
        )),
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
                HttpResponse::Ok().json(Success::new("success", users))
            } else {
                HttpResponse::NotFound().json(ServerError::new(
                    "not found",
                    "Have no users like this name!",
                ))
            }
        }
        Err(_) => HttpResponse::InternalServerError().json(ServerError::new(
            "internal server error",
            "Error while fetching users from the database.",
        )),
    }
}

pub async fn add_one_user(body: Json<UserModel>, data: Data<AppState>) -> HttpResponse {
    // Validação do tamanho do nome e da senha
    if body.name.len() < 1 || body.password.len() < 8 {
        return HttpResponse::NotAcceptable().json(ServerError::new(
            "not acceptable",
            "The name cannot be empty and the password should be bigger than 7 characters!",
        ));
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
        Ok(user) => HttpResponse::Created().json(Success::new("created", user)),
        Err(err) => HttpResponse::InternalServerError().json(ServerError::from_sqlx_error(err)),
    }
}

pub async fn delete_one_user(body: Json<UserSchema>, data: Data<AppState>) -> HttpResponse {
    match query!(
        "DELETE FROM rust_user WHERE id = $1 AND name = $2 AND password = $3",
        body.id,
        body.name,
        body.password
    )
    .execute(&data.db)
    .await
    {
        Ok(result) => {
            if result.rows_affected() > 0 {
                HttpResponse::Ok().json(Success::new("success", "User deleted successfully"))
            } else {
                HttpResponse::NotFound().json(ServerError::new(
                    "not found",
                    "No user found with the given credentials",
                ))
            }
        }
        Err(err) => HttpResponse::InternalServerError().json(ServerError::from_sqlx_error(err)),
    }
}

pub fn get_scope() -> Scope {
    scope("/user")
        .service(crate::controller::user::get_all_users)
        .service(crate::controller::user::get_some_users)
        .service(crate::controller::user::get_one_user)
        .service(crate::controller::user::post_one_user)
        .service(crate::controller::user::delete_one_user)
}
