use crate::routes::utils::*;

#[get("/get_hotel_reviews/{hotel_id}")]
pub async fn get_hotel_reviews(
    path: web::Path<usize>, app_state: web::Data<AppState>
) -> HttpResponse {
    let hotel_id: usize = path.into_inner();

    #[derive(Debug, Clone, sqlx::FromRow, Serialize, Deserialize)]
    pub struct Review {
        pub hotel_id: i32,
        pub review_id: String,
        pub rating: i32,
        pub author: String,
        pub title: String,
        pub text: String,
        pub time: String,
    }

    let hotel: Result<Vec<Review>> = sqlx::query_as(
        SELECT_HOTEL_REVIEWS
    ).bind(hotel_id as u64)
    .fetch_all(&app_state.pool).await;

    match hotel {
        Ok(h) => HttpResponse::Ok().json(h),
        Err(e) => {
            eprintln!("Error getting hotel reviews: {e}"); 
            HttpResponse::BadRequest().into()
        }
    }
}