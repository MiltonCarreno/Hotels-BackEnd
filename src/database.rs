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