use actix_web::{get, post, web, HttpResponse};
use serde::{Serialize, Deserialize};
use sqlx::mysql::MySqlPoolOptions;
use sqlx::mysql::MySqlQueryResult;
use sqlx::MySqlPool;
use sqlx::Result;

#[derive(Clone)]
pub struct AppState {
    pub pool: MySqlPool,
}

pub async fn root() -> String {
    "Server is up and running".to_string()
}

#[get("/create_tbls")]
pub async fn create_tbls(app_state: web::Data<AppState>) -> HttpResponse {
    let created_tbls = sqlx::query(
        "create table if not exists \
        users(\
            id INT AUTO_INCREMENT,\
            username VARCHAR(15) NOT NULL,\
            email VARCHAR(255) NOT NULL,\
            PRIMARY KEY(id)\
        );"
    ).execute(&app_state.pool).await;

    match created_tbls {
        Ok(_) => HttpResponse::Ok().into(),
        Err(_) => HttpResponse::BadRequest().into(),
    }
}

#[get("/delete_tbls")]
pub async fn delete_tbls(app_state: web::Data<AppState>) -> HttpResponse {
    let created_tbls = sqlx::query(
        "drop table users;"
    ).execute(&app_state.pool).await;

    match created_tbls {
        Ok(_) => HttpResponse::Ok().into(),
        Err(_) => HttpResponse::BadRequest().into(),
    }
}

#[get("/add_users")]
pub async fn add_users(app_state: web::Data<AppState>) -> HttpResponse {
    let added_users = sqlx::query(
        "insert into users(username, email) values \
        ('marc', 'marc@email.de'), ('leon', 'leon@email.de');"
    ).execute(&app_state.pool).await;

    match added_users {
        Ok(_) => HttpResponse::Ok().into(),
        Err(_) => HttpResponse::BadRequest().into(),
    }
}

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
        "select * from users where id = ?"
    ).bind(user_id as u64)
    .fetch_optional(&app_state.pool).await;

    match user {
        Ok(_) => HttpResponse::Ok().json(user.unwrap()),
        Err(_) => HttpResponse::BadRequest().into(),
    }
}

pub async fn delete_user(path: web::Path<usize>, app_state: web::Data<AppState>
) -> HttpResponse {
    let user_id: usize = path.into_inner();

    let deleted: sqlx::Result<MySqlQueryResult> = sqlx::query(
        "delete from users where id = ?"
    ).bind(user_id as u64).execute(&app_state.pool).await;

    match deleted {
        Ok(u) => HttpResponse::Ok().into(),
        Err(_) => HttpResponse::BadRequest().into(),
    }
}