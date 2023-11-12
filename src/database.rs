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