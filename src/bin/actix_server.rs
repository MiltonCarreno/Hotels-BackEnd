use std::env;
use std::process;
use actix_web::{http::header, web, App, HttpServer};
use actix_cors::Cors;
use sqlx::mysql::MySqlPoolOptions;

use data_parser::{config::*, parser::*, database::*, routes::*};


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // cargo r -- /Users/milton/Desktop/HotelDataParser/data/hotels /Users/milton/Desktop/HotelDataParser/data/reviews
    // Parse arguments
    let config = Config::build(env::args())
        .unwrap_or_else(|err| {
            println!("\nError parsing arguments: {err}\n");
            process::exit(1);
        });
    
    // Multithreading approach
    let (_hotels, _reviews) = 
    mt_processing(
        &config.reviews_path,
        &config.hotels_path,
    );

    const DB_URL: &str = "mysql://root:123123123@127.0.0.1:3306/hotels_data";

    let pool = MySqlPoolOptions::new()
        .max_connections(10)
        .connect(DB_URL)
        .await
        .unwrap();

    let app_state = AppState { pool };
    let _app_state_c = app_state.clone();

    // create_tbls(&app_state).await;
    // add_hotels_data(&app_state, hotels).await;
    // add_reviews_data(&app_state, reviews).await;
    // add_users(&app_state).await;

    let server = HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin("http://localhost:5173")
            .allowed_methods(vec!["GET", "POST"])
            .allowed_headers(vec![header::AUTHORIZATION, header::ACCEPT])
            .allowed_header(header::CONTENT_TYPE)
            .max_age(3600);

        App::new().wrap(cors)
            .app_data(web::Data::new(app_state.clone()))
            .route("/", web::get().to(root))
            .service(get_user)
            .service(check_user)
            .service(add_user)
            .service(delete_user)
            .service(update_user)
            .service(get_all_users)
            .service(get_hotel)
            .service(get_all_hotels)
            .service(get_like_hotels)
            .service(get_hotel_reviews)
            .service(add_user_review)
            .service(get_all_user_reviews)
            .service(delete_user_review)
    }).bind(("127.0.0.1", 8080))?.run().await;

    // drop_tbls(&app_state_c).await;

    server
}