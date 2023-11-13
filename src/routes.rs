use actix_web::{get, post, web, HttpResponse};
use sqlx::mysql::MySqlQueryResult;
use serde::Serialize;
use sqlx::Result;
use crate::database::*;
use crate::sql_strs::*;

pub async fn root() -> String {
    "Server is up and running".to_string()
}

#[get("/add_this_user/{user_name}/{email}")]
pub async fn add_this_users(
    path: web::Path<(String, String)>, app_state: web::Data<AppState>
) -> HttpResponse {
    let (user_name, email) = path.into_inner();
    let added_users = sqlx::query(
        INSERT_USER
    ).bind(user_name).bind(email).execute(&app_state.pool).await;

    match added_users {
        Ok(_) => HttpResponse::Ok().into(),
        Err(_) => HttpResponse::BadRequest().into(),
    }
}

#[get("/get/{user_id}")]
pub async fn get_user(path: web::Path<usize>, app_state: web::Data<AppState>
) -> HttpResponse {
    let user_id: usize = path.into_inner();

    #[derive(sqlx::FromRow, Serialize)]
    struct User {
        id: i32,
        username: String,
        email: String,
    }

    let user: Result<Option<User>> = sqlx::query_as(
        SELECT_USER
    ).bind(user_id as u64)
    .fetch_optional(&app_state.pool).await;

    match user {
        Ok(_) => HttpResponse::Ok().json(user.unwrap()),
        Err(_) => HttpResponse::BadRequest().into(),
    }
}

#[get("/delete/{user_id}")]
pub async fn delete_user(path: web::Path<usize>, app_state: web::Data<AppState>
) -> HttpResponse {
    let user_id: usize = path.into_inner();

    let deleted: sqlx::Result<MySqlQueryResult> = sqlx::query(
        DELETE_USER
    ).bind(user_id as u64).execute(&app_state.pool).await;

    match deleted {
        Ok(u) => HttpResponse::Ok().into(),
        Err(_) => HttpResponse::BadRequest().into(),
    }
}