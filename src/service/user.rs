use actix_web::{
    web::{scope, Data, Json, Path},
    HttpResponse, Scope,
};
use sqlx::{query, query_as};
use uuid::Uuid;

use crate::{model, response, schema};

// Método para obter uma resposta HTTP contendo a lista
// de todos os itens de rust_user.
pub async fn get_all_users(data: Data<crate::AppState>) -> HttpResponse {
    match query_as!(schema::User, "SELECT * FROM rust_user")
        .fetch_all(&data.db)
        .await
    {
        Ok(users) => HttpResponse::Ok().json(response::Success::new("success", users)), // Retornando a lista de sucesso.
        Err(err) => {
            HttpResponse::InternalServerError().json(response::ServerError::from_sqlx_error(err))
        } // Retornando o erro inesperado pelo sqlx,
    }
}

// Método para pesquisar um usuário a partir de seu id
// e retornando uma resposta HTTP contendo o usuário.
// O erro acontece se não for encontrado ou se o uuid for inválido.
pub async fn find_one_user(path: Path<String>, data: Data<crate::AppState>) -> HttpResponse {
    // Transferindo a variavel de url em Uuid para a pesquisa.
    match Uuid::parse_str(&path) {
        Ok(uuid) => match query_as!(schema::User, "SELECT * FROM rust_user WHERE id = $1", uuid)
            .fetch_one(&data.db)
            .await
        {
            Ok(user) => HttpResponse::Ok().json(response::Success::new("found", user)), // Retornando o usuário que corresponde ao UUID.
            Err(_) => HttpResponse::NotFound().json(response::ServerError::new(
                "not found",
                "This UUID doesn't exist in the database!",
            )), // Retornando o erro no qual indica que o usuário correspondente não foi encontrado,
        },
        Err(_) => HttpResponse::BadRequest().json(response::ServerError::new(
            "bad request",
            "Invalid UUID format, unable to search!",
        )), // Retornando o erro que indca UUID inválido.
    }
}

// Método para pesquisa por um ou mais usuários cujo nome se assemelha
// a variavel determinada na url e retornando uma resposta HTTP contendo
// uma lista dos usuários encontrados.
// O erro acontece caso não seja encontrado nenhum usuário.
pub async fn find_some_users(path: Path<String>, data: Data<crate::AppState>) -> HttpResponse {
    let search_pattern = format!("%{}%", path.to_string()); // Formatando o pattern para a pesquisa.

    match query_as!(
        schema::User,
        "SELECT * FROM rust_user WHERE name LIKE $1",
        search_pattern
    )
    .fetch_all(&data.db)
    .await
    {
        Ok(users) => {
            // Verificando se foi encontrado algum usuário.
            if !users.is_empty() {
                HttpResponse::Ok().json(response::Success::new("success", users))
            // Retornando os usuaŕios encontrados.
            } else {
                HttpResponse::NotFound().json(response::ServerError::new(
                    "not found",
                    "Have no users like this name!",
                )) // Retornando o erro no qual indica que nenhum semelhante ao nome foi encontrado.
            }
        }
        Err(_) => HttpResponse::InternalServerError().json(response::ServerError::new(
            "internal server error",
            "Error while fetching users from the database.",
        )), // Retornando o erro inesperado pelo sqlx.
    }
}

// Método para adicionar um usuário em rust_user e retornando uma
// resposta HTTP contendo o usuário adicionado.
// O error ocorre quando o nome está vazio ou a
// senha contem menos que 8 caracteres.
pub async fn add_one_user(body: Json<model::User>, data: Data<crate::AppState>) -> HttpResponse {
    // Validação do tamanho do nome e da senha.
    if body.name.is_empty() || body.password.len() < 8 {
        return HttpResponse::NotAcceptable().json(response::ServerError::new(
            "not acceptable",
            "The name cannot be empty and the password should be bigger than 7 characters!",
        )); // Retornando o erro indicando que o usuário requisitado não é aceitável.
    }

    match query_as!(
        schema::User,
        "INSERT INTO rust_user (name, password) VALUES ($1, $2) RETURNING *",
        body.name,
        body.password
    )
    .fetch_one(&data.db)
    .await
    {
        Ok(user) => HttpResponse::Created().json(response::Success::new("created", user)), // Retornando o usuario que foi criado.
        Err(err) => {
            HttpResponse::InternalServerError().json(response::ServerError::from_sqlx_error(err))
        } // Retornando o erro inesperado pelo sqlx.
    }
}

// Método para remover o usuário a partir de autenticação
// via id, nome e senha retornando uma resposta HTTP contendo
// o resultado da pesquisa.
// O erro é retornado quando nenhuma linha é afetada.
pub async fn delete_one_user(
    body: Json<schema::User>,
    data: Data<crate::AppState>,
) -> HttpResponse {
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
            // Verificando se alguma linha foi afetada.
            if result.rows_affected() > 0 {
                // Retornando a mensagem que indica sobr eo sucesso na deleção.
                HttpResponse::Ok().json(response::Success::new(
                    "success",
                    "User deleted successfully",
                ))
            } else {
                HttpResponse::NotFound().json(response::ServerError::new(
                    "not found",
                    "No user found with the given credentials",
                )) // Retornando erro que indica a não correspondencia na deleção.
            }
        }
        Err(err) => {
            HttpResponse::InternalServerError().json(response::ServerError::from_sqlx_error(err))
        } // Retornando o erro inesperado pelo sqlx.
    }
}

// Método utilizado no escopo principal para obter o escopo "user".
pub fn get_scope() -> Scope {
    scope("/user")
        .service(crate::controller::user::get_all_users) //   Obter todos os usuários.  GET     ("api/user/all")
        .service(crate::controller::user::get_some_users) //  Pesquisa alguns usuários. GET     ("api/user/some/{name}")
        .service(crate::controller::user::get_one_user) //    Pesquisa um usuário.      GET     ("api/user/one/{id}")
        .service(crate::controller::user::post_one_user) //   Adiciona um usuário.      POST    ("api/user/manage")
        .service(crate::controller::user::delete_one_user) // Deleta um usuário.        DELETE  ("api/user/manage")
}

// Logs da rota.
pub fn logs() {
    println!("\n# Loading user route...");
    println!("@ROUTE('api/user/all')                GET     : Return all users. ");
    println!("@ROUTE('api/user/some/{{name}}')        GET     : Return some users by name.");
    println!("@ROUTE('api/user/one/{{id}}')           GET     : Return one user by id.");
    println!("@ROUTE('api/user/manage')             POST    : Add one user and return.");
    println!(
        "@ROUTE('api/user/manage')             DELETE  : Delete one user by id, name and password."
    );
}
