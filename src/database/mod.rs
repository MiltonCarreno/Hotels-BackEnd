use std::collections::HashMap;
use sqlx::MySqlPool;

pub mod sql_strs;

pub use sql_strs::*;
pub use crate::hotels_info::*;

#[derive(Clone)]
pub struct AppState {
    pub pool: MySqlPool,
}

pub async fn create_tbls(app_state: &AppState) {
    let users_tbl = sqlx::query(
        CREATE_USERS_TABLE
    ).execute(&app_state.pool).await;

    if let Err(e) = users_tbl {
        eprintln!("Error creating 'users' table: {}", e);
    }

    let hotels_tbl = sqlx::query(
        CREATE_HOTELS_TABLE
    ).execute(&app_state.pool).await;

    if let Err(e) = hotels_tbl {
        eprintln!("Error creating 'hotels' table: {}", e);
    }

    let reviews_tbl = sqlx::query(
        CREATE_REVIEWS_TABLE
    ).execute(&app_state.pool).await;

    if let Err(e) = reviews_tbl {
        eprintln!("Error creating 'hotels' table: {}", e);
    }

    let user_reviews_tbl = sqlx::query(
        CREATE_USER_REVIEWS_TABLE
    ).execute(&app_state.pool).await;

    if let Err(e) = user_reviews_tbl {
        eprintln!("Error creating 'users_reviews' table: {}", e);
    }
}

pub async fn drop_tbls(app_state: &AppState) {
    let users_tbl = sqlx::query(
        DROP_USERS_TABLE
    ).execute(&app_state.pool).await;

    if let Err(e) = users_tbl {
        eprintln!("Error creating 'users' table: {}", e);
    }

    let reviews_tbl = sqlx::query(
        DROP_REVIEWS_TABLE
    ).execute(&app_state.pool).await;

    if let Err(e) = reviews_tbl {
        eprintln!("Error creating 'hotels' table: {}", e);
    }

    let hotels_tbl = sqlx::query(
        DROP_HOTELS_TABLE
    ).execute(&app_state.pool).await;

    if let Err(e) = hotels_tbl {
        eprintln!("Error creating 'hotels' table: {}", e);
    }
}

pub async fn add_users(app_state: &AppState) {
    let added_users = sqlx::query(
        INSERT_USERS
    ).execute(&app_state.pool).await;

    if let Err(e) = added_users {
        eprintln!("Error adding users: {e}");
    }
}

pub async fn add_hotels_data(
    app_state: &AppState, hotels: HashMap<i32, Hotel>
) {    
    for hotel in hotels.values() {
        let added_hotel = sqlx::query(
            INSERT_HOTEL
        ).bind(hotel.hotel_id).bind(hotel.name.clone())
        .bind(hotel.address.clone()).bind(hotel.city.clone())
        .bind(hotel.province.clone()).bind(hotel.country.clone())
        .execute(&app_state.pool).await;
    
        if let Err(e) = added_hotel {
            eprintln!("Error adding users: {e}");
        }
    }
}

pub async fn add_reviews_data(
    app_state: &AppState, reviews: HashMap<i32, Vec<Review>>
) {    
    for review_set in reviews.values() {
        for review in review_set {
            let added_review = sqlx::query(
                INSERT_REVIEW
            ).bind(review.review_id.clone()).bind(review.hotel_id.clone())
            .bind(review.rating).bind(review.author.clone())
            .bind(review.title.clone()).bind(review.text.clone())
            .bind(review.time.to_string())
            .execute(&app_state.pool).await;
        
            if let Err(e) = added_review {
                eprintln!("Error adding users: {e}");
            }
        }
    }
}