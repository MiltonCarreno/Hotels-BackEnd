use std::env;
use std::process;
use actix_web::{get, post, web, App, Result,
    HttpResponse, HttpServer, Responder};
use serde::{Serialize, Deserialize};
use sqlx::mysql::MySqlPoolOptions;
use sqlx::MySqlPool;

mod config;
mod database;

use config::Config;
use database::*;
use data_parser::{mt_processing, r_processing};
use sqlx::mysql::MySqlQueryResult;


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // cargo r -- /Users/milton/Desktop/HotelDataParser/data/hotels /Users/milton/Desktop/HotelDataParser/data/reviews
    // Parse arguments
    // let config = Config::build(env::args())
    //     .unwrap_or_else(|err| {
    //         println!("\nError parsing arguments: {err}\n");
    //         process::exit(1);
    //     });
    
    // // Multithreading approach
    // mt_processing(
    //     &config.reviews_path,
    //     &config.hotels_path,
    // );

    // // Recursive approach
    // r_processing(
    //     &config.reviews_path,
    //     &config.hotels_path,
    // );

    const DB_URL: &str = "mysql://root:123123123@127.0.0.1:3306/hotels_data";

    let pool = MySqlPoolOptions::new()
        .max_connections(10)
        .connect(DB_URL)
        .await
        .unwrap();

    let app_state = AppState { pool };

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(app_state.clone()))
            .route("/", web::get().to(root))
            .service(create_tbls)
            .service(delete_tbls)
            .service(add_users)
            .route("/get/{user_id}", web::get().to(get_user))
            .route("/delete/{user_id}", web::get().to(delete_user))
    }).bind(("127.0.0.1", 8080))?.run().await
}