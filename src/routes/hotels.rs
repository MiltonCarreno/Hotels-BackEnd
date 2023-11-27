use crate::routes::utils::*;

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
            eprintln!("Error getting hotel by id: {e}"); 
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

#[get("/get_like_hotels/{hotel_name}")]
pub async fn get_like_hotels(
    path: web::Path<String>, app_state: web::Data<AppState>
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
            eprintln!("Error getting hotel by name: {e}"); 
            HttpResponse::BadRequest().into()
        }
    }
}