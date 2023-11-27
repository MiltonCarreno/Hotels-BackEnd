use crate::routes::utils::*;

#[post("/add_user_review")]
pub async fn add_user_review(
    review: web::Json<NewUserReview>, app_state: web::Data<AppState>
) -> HttpResponse {
    let added_user_review = sqlx::query(
        INSERT_USER_REVIEW
    ).bind(review.user_id.clone()).bind(review.hotel_id.clone())
    .bind(review.title.clone()).bind(review.text.clone())
    .execute(&app_state.pool).await;

    match added_user_review {
        Ok(_) => HttpResponse::Ok().into(),
        Err(e) => {
            eprintln!("Error adding new user review: {e}");
            HttpResponse::BadRequest().into()
        },
    }
}

#[get("/get_all_user_reviews")]
pub async fn get_all_user_reviews(app_state: web::Data<AppState>
) -> HttpResponse {
    let user_reviews: Result<Vec<UserReview>> = sqlx::query_as(
        SELECT_ALL_USER_REVIEWS
    ).fetch_all(&app_state.pool).await;

    match user_reviews {
        Ok(us) => HttpResponse::Ok().json(us),
        Err(e) => {
            eprintln!("Error getting all user reviews: {e}"); 
            HttpResponse::BadRequest().into()
        }
    }
}

#[get("/delete_user_review/{user_review_id}")]
pub async fn delete_user_review(
    path: web::Path<usize>, app_state: web::Data<AppState>
) -> HttpResponse {
    let user_review_id: usize = path.into_inner();

    let deleted: sqlx::Result<MySqlQueryResult> = sqlx::query(
        DELETE_USER_REVIEW
    ).bind(user_review_id as u64).execute(&app_state.pool).await;

    match deleted {
        Ok(_) => HttpResponse::Ok().into(),
        Err(e) => {
            eprintln!("Error deleting user review: {e}");
            HttpResponse::BadRequest().into()
        },
    }
}