use std::io::BufReader;
use std::fs;
use std::ffi::OsStr;
use std::collections::HashMap;
use chrono::prelude::*;

use crate::parser::utils::*;

pub fn r_traverse_dir(dir_path: String, hotels_info: &mut HotelsInfo, data: &Data) {
    let entries = fs::read_dir(dir_path).unwrap();

    for entry in entries {
        let entry_path = entry.as_ref().unwrap()
            .path().into_os_string().into_string().unwrap();
        let entry_extention = entry.as_ref().unwrap()
            .path().extension().unwrap_or(
                OsStr::new("No Extension")
            ).to_os_string().into_string().unwrap();
                                    
        if entry.as_ref().unwrap().path().is_dir() {
            r_traverse_dir(entry_path, hotels_info, data);
        } else if entry_extention == "json" {
            match data {
                Data::Hotels => r_process_hotels(entry_path, hotels_info),
                Data::Reviews => r_process_reviews(entry_path, hotels_info),
            };
        }
    }
}

pub fn r_process_reviews(file_path: String, hotels_info: &mut HotelsInfo) {
    let file = fs::File::open(file_path).unwrap();
    let reader = BufReader::new(file);
    let val: serde_json::Value = serde_json::from_reader(reader).unwrap();
    let collection = &val["reviewDetails"]["reviewCollection"]["review"];
    let num_reviews = val["reviewDetails"]["numberOfReviewsInThisPage"]
                                .as_u64().unwrap() as usize;
    if num_reviews > 0 {
        let mut reviews: Vec<Review> = vec![];
        for i in 0..num_reviews {
            let hotel_id: i32 = collection[i]["hotelId"]
                .as_str().unwrap().parse().unwrap();
            let review_id = collection[i]["reviewId"]
                .as_str().unwrap().to_string();
            let rating: i32 = collection[i]["ratingOverall"]
                .to_string().parse().unwrap();
            let text = collection[i]["reviewText"]
                .as_str().unwrap().to_string();
            let time = collection[i]["reviewSubmissionTime"]
                .as_str().unwrap().parse::<DateTime<Utc>>().unwrap();
            let author = match collection[i]["userNickname"]
                .as_str().unwrap().to_string().is_empty() {
                true => { "ANONYMOUS".to_string() }
                false => { collection[i]["userNickname"].as_str().unwrap()
                    .to_string() }
            };
            let title = match collection[i]["title"]
                .as_str().unwrap().to_string().is_empty() {
                true => { "NO TITLE".to_string() }
                false => { collection[i]["title"].as_str().unwrap()
                    .to_string() }
            };

            reviews.push(
                Review { hotel_id, review_id, rating,
                        author, title, text, time }
            );
        }
        hotels_info.add_reviews(reviews[0].hotel_id, reviews);
    }
}

pub fn r_process_hotels(file_path: String, hotels_info: &mut HotelsInfo) {
    let file = fs::File::open(file_path).unwrap();
    let reader = BufReader::new(file);
    let val: serde_json::Value = serde_json::from_reader(reader).unwrap();
    let collection = &val["sr"];
    let num_hotels = collection.as_array().unwrap().len();
    let mut hotels: HashMap<i32, Hotel> = HashMap::new();

    for i in 0..num_hotels {
        let hotel_id: i32 = collection[i]["id"]
            .as_str().unwrap().parse().unwrap();
        let name = collection[i]["f"]
            .as_str().unwrap().to_string();
        let address = collection[i]["ad"]
            .as_str().unwrap().to_string();
        let city = collection[i]["ci"]
            .as_str().unwrap().to_string();
        let province = collection[i]["pr"]
            .as_str().unwrap().to_string();
        let country = collection[i]["c"]
            .as_str().unwrap().to_string();
    
        hotels.insert(
            hotel_id,
            Hotel {hotel_id, name, address,
                city, province, country}
        );
    }
    hotels_info.add_hotels(hotels);
}