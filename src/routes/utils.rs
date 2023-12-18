pub use actix_web::{get, post, web, HttpResponse};
pub use sqlx::mysql::MySqlQueryResult;
pub use serde::{Serialize, Deserialize};
pub use sqlx::Result;
use jsonwebtoken::{
    encode, decode, EncodingKey, Header, DecodingKey, Validation
};
use chrono::{Utc, Duration};

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

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    user_id: usize,
    name: String,
    email: String,
    exp: usize,
}

async fn create_jwt(user_id: usize, secret: String) -> String {
    let claims: Claims = Claims { 
        user_id, name: "name".to_string(), email: "email".to_string(), 
        exp: (Utc::now() + Duration::hours(1)).timestamp() as usize
    };

    return encode(
        &Header::default(), 
        &claims, 
        &EncodingKey::from_secret(secret.as_ref())
    ).unwrap();
}

async fn check_jwt(token: String, secret: String) -> bool {
    let token = decode::<Claims>(
        &token, &DecodingKey::from_secret(secret.as_ref()),
        &Validation::default()
    );
    match token {
        Ok(_) => true,
        Err(e) => {
            println!("Error validating token: {e}");
            false
        }
    }
}