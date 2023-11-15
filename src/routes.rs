use actix_web::{get, post, web, HttpResponse};
use sqlx::mysql::MySqlQueryResult;
use serde::{Serialize, Deserialize};
use sqlx::Result;
use crate::database::*;
use crate::sql_strs::*;

#[derive(Deserialize, Clone)]
pub struct NewUser {
    username: String,
    email: String,
}

#[derive(Debug, sqlx::FromRow, Serialize, Deserialize, Clone)]
pub struct User {
    id: i32,
    username: String,
    email: String,
}

pub async fn root() -> String {
    "Server is up and running".to_string()
}

#[post("/add_user")]
pub async fn add_user(
    user: web::Json<NewUser>, app_state: web::Data<AppState>
) -> HttpResponse {
    let added_user = sqlx::query(
        INSERT_USER
    ).bind(user.username.clone()).bind(user.email.clone())
    .execute(&app_state.pool).await;

    match added_user {
        Ok(_) => HttpResponse::Ok().into(),
        Err(e) => {
            eprintln!("Error adding new user: {e}");
            HttpResponse::BadRequest().into()
        },
    }
}

#[get("/get/{user_id}")]
pub async fn get_user(path: web::Path<usize>, app_state: web::Data<AppState>
) -> HttpResponse {
    let user_id: usize = path.into_inner();

    let user: Result<Option<User>> = sqlx::query_as(
        SELECT_USER
    ).bind(user_id as u64)
    .fetch_optional(&app_state.pool).await;

    match user {
        Ok(_) => HttpResponse::Ok().json(user.unwrap()),
        Err(e) => {
            eprintln!("Error getting user: {e}"); 
            HttpResponse::BadRequest().into()
        }
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
        Ok(_) => HttpResponse::Ok().into(),
        Err(e) => {
            eprintln!("Error deleting user: {e}");
            HttpResponse::BadRequest().into()
        },
    }
}

#[post("/update")]
pub async fn update_user(
    user: web::Form<User>, app_state: web::Data<AppState>
) -> HttpResponse {
    let updated: sqlx::Result<MySqlQueryResult> = sqlx::query(
        UPDATE_USER
    ).bind(user.username.clone()).bind(user.email.clone()).bind(user.id)
    .execute(&app_state.pool).await;

    match updated {
        Ok(_) => HttpResponse::Ok().into(),
        Err(e) => {
            eprintln!("Error updating user: {e}");
            HttpResponse::BadRequest().into()
        },
    }
}