use actix_web::{get, post, web, HttpResponse};
use sqlx::mysql::MySqlQueryResult;
use serde::{Serialize, Deserialize};
use sqlx::Result;
use crate::database::*;
use crate::sql_strs::*;
use data_parser::hotels_info::Hotel;

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

#[get("/get_user/{user_id}")]
pub async fn get_user(path: web::Path<usize>, app_state: web::Data<AppState>
) -> HttpResponse {
    let user_id: usize = path.into_inner();

    let user: Result<Option<User>> = sqlx::query_as(
        SELECT_USER
    ).bind(user_id as u64)
    .fetch_optional(&app_state.pool).await;

    match user {
        Ok(u) => HttpResponse::Ok().json(u.unwrap()),
        Err(e) => {
            eprintln!("Error getting user: {e}"); 
            HttpResponse::BadRequest().into()
        }
    }
}

#[get("/get_hotel/{hotel_id}")]
pub async fn get_hotel(path: web::Path<usize>, app_state: web::Data<AppState>
) -> HttpResponse {
    let hotel_id: usize = path.into_inner();

    let hotel: Result<Option<Hotel>> = sqlx::query_as(
        SELECT_HOTEL
    ).bind(hotel_id as u64)
    .fetch_optional(&app_state.pool).await;

    match hotel {
        Ok(h) => HttpResponse::Ok().json(h.unwrap()),
        Err(e) => {
            eprintln!("Error getting hotel: {e}"); 
            HttpResponse::BadRequest().into()
        }
    }
}

#[get("/get_like_hotels/{hotel_name}")]
pub async fn get_like_hotels(path: web::Path<String>, app_state: web::Data<AppState>
) -> HttpResponse {
    let mut hotel_name =  path.into_inner();
    hotel_name = "%".to_string() + &hotel_name + "%";

    let hotels: Result<Vec<Hotel>> = sqlx::query_as(
        SELECT_LIKE_HOTELS
    ).bind(hotel_name)
    .fetch_all(&app_state.pool).await;

    match hotels {
        Ok(hs) => HttpResponse::Ok().json(hs),
        Err(e) => {
            eprintln!("Error getting hotel: {e}"); 
            HttpResponse::BadRequest().into()
        }
    }
}

#[get("/get_all_users")]
pub async fn get_all_users(app_state: web::Data<AppState>) -> HttpResponse {
    let users: Result<Vec<User>> = sqlx::query_as(
        SELECT_ALL_USERS
    ).fetch_all(&app_state.pool).await;

    match users {
        Ok(us) => HttpResponse::Ok().json(us),
        Err(e) => {
            eprintln!("Error getting all users: {e}"); 
            HttpResponse::BadRequest().into()
        }
    }
}

#[get("/get_all_hotels")]
pub async fn get_all_hotels(app_state: web::Data<AppState>) -> HttpResponse {
    let hotels: Result<Vec<Hotel>> = sqlx::query_as(
        SELECT_ALL_HOTELS
    ).fetch_all(&app_state.pool).await;

    match hotels {
        Ok(hs) => HttpResponse::Ok().json(hs),
        Err(e) => {
            eprintln!("Error getting all hotels: {e}"); 
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