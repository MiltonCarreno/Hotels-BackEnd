pub use actix_web::{get, post, web, HttpResponse};
pub use sqlx::mysql::MySqlQueryResult;
pub use serde::{Serialize, Deserialize};
pub use sqlx::Result;

pub use crate::database::*;

#[derive(Deserialize, Clone)]
pub struct NewUser {
    pub username: String,
    pub email: String,
}

#[derive(Debug, sqlx::FromRow, Serialize, Deserialize, Clone)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub email: String,
}

#[derive(Deserialize, Clone)]
pub struct NewUserReview {
    pub user_id: i32,
    pub hotel_id: i32,
    pub title: String,
    pub text: String,
}

#[derive(Debug, sqlx::FromRow, Serialize, Deserialize, Clone)]
pub struct UserReview {
    pub review_id: i32,
    pub user_id: i32,
    pub hotel_id: i32,
    pub title: String,
    pub text: String,
}