use std::io::BufReader;
use std::fs;
use std::ffi::OsStr;
use std::sync::{ Arc, Mutex };
use std::thread;
use std::collections::HashMap;
use chrono::prelude::*;

pub mod hotels_info;

use hotels_info::*;

pub enum Data {
    Hotels,
    Reviews,
}

impl Data {
    pub fn copy(&self) -> Data {
        match self {
            Data::Hotels => Data::Hotels,
            Data::Reviews => Data::Reviews,
        }
    }
}

pub fn mt_processing(
    r_dir_path: &String, h_dir_path: &String
) -> (HashMap<i32, Hotel>, HashMap<i32, Vec<Review>>) {
    // HotelsInfo Struct
    let info: Arc<Mutex<HotelsInfo>> = 
        Arc::new(Mutex::new(HotelsInfo::new()));

    // Reviews Files Parsing
    let rev_dir_path = r_dir_path.clone();
    let hotels_info = info.clone();
    let handle = thread::spawn(move || {
        mt_traverse_dir(rev_dir_path, hotels_info, Data::Reviews)
    });
    handle.join().unwrap();

    // Hotels Files Parsing
    let hot_dir_path = h_dir_path.clone();
    let hotels_info = info.clone();
    let handle = thread::spawn(move || {
        mt_traverse_dir(hot_dir_path, hotels_info, Data::Hotels)
    });
    handle.join().unwrap();

    let hotels = info.lock().unwrap().get_hotels();
    let reviews = info.lock().unwrap().get_reviews();

    return (hotels, reviews);
}

pub fn r_processing(r_dir_path: &String, h_dir_path: &String) {
    // HotelsInfo Struct
    let mut info = HotelsInfo::new();

    // Reviews Parsing
    r_traverse_dir(
        r_dir_path.clone(), &mut info, &Data::Reviews
    );

    // Hotels Parsing
    r_traverse_dir(
        h_dir_path.clone(), &mut info, &Data::Hotels
    );

    println!("\nRecursive Approach: {:#?}", 
        info.search_hotels(20191).unwrap());
    println!("\nRecursive Approach: {:#?}", 
        info.search_reviews(20191).unwrap()[10]);
}

// ****************************************************************************
//                              Multithreading Approach
// ****************************************************************************

/**
 * Multithreading approach to traversing directories
 * 
 * # Parameters:
 * - 'dir_path': A string containing the directory path to be traversed.
 * - 'reviews_set': Hashmap to be populated with hotel ids (key) and their
 *      corresponding reviews (value). Hashmap held in Mutex to prevent
 *      race conditions, and Arc used to allow multiple thread-safe references.
 */
pub fn mt_traverse_dir(
    dir_path: String, hotels_info: Arc<Mutex<HotelsInfo>>, data: Data) {
    let entries = fs::read_dir(dir_path).unwrap();
    let mut handles = vec![];

    for entry in entries {
        let entry_extention = entry.as_ref().unwrap()
            .path().extension().unwrap_or(OsStr::new("No Extension"))
            .to_os_string().into_string().unwrap();
        let entry_path = entry.as_ref().unwrap()
            .path().into_os_string().into_string().unwrap();
        let hotels_info = hotels_info.clone();
        let data = data.copy();

        if entry.as_ref().unwrap().path().is_dir() {
            let handle = thread::spawn(move || {
                mt_traverse_dir(entry_path, hotels_info, data);
            });
            handles.push(handle);
        } else if entry_extention == "json" {    
            let handle = match data {
                Data::Hotels => {thread::spawn(move || {
                    mt_process_hotels(entry_path, hotels_info)
                })},
                Data::Reviews => {thread::spawn(move || {
                    mt_process_reviews(entry_path, hotels_info)
                })},
            };
            handles.push(handle);
        }
    }

    for handle in handles {
        handle.join().unwrap();
    }
}

/**
 * Multithreading approach to process review files
 * 
 * # Parameters:
 * - 'file_path': A string containing the file path to be processed.
 * - 'reviews_set': Hashmap to be populated with hotel ids (keys) and their
 *      corresponding reviews (values). Hashmap held in Mutex to prevent
 *      race conditions, and Arc used to allow multiple thread-safe references.
 */
pub fn mt_process_reviews(file_path: String, hotels_info: Arc<Mutex<HotelsInfo>>) {
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
            let author = collection[i]["userNickname"]
                .as_str().unwrap().to_string();
            let title = collection[i]["title"]
                .as_str().unwrap().to_string();
            let text = collection[i]["reviewText"]
                .as_str().unwrap().to_string();
            let time = collection[i]["reviewSubmissionTime"]
                .as_str().unwrap().parse::<DateTime<Utc>>().unwrap();
        
            reviews.push(
                Review { hotel_id, review_id, rating,
                        author, title, text, time }
            );
        }
        hotels_info.lock().unwrap().add_reviews(reviews[0].hotel_id, reviews);
    }
}

/**
 * Multithreading approach to process hotel files
 * 
 * # Parameters:
 * - 'file_path': A string containing the file path to be processed.
 * - 'hotels_set': Hashmap to be populated with hotel ids (key) and their
 *      corresponding hotel (value). Hashmap held in Mutex to prevent
 *      race conditions, and Arc used to allow multiple thread-safe references.
 */
pub fn mt_process_hotels(file_path: String, hotels_info: Arc<Mutex<HotelsInfo>>) {
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
    hotels_info.lock().unwrap().add_hotels(hotels);
}

// ****************************************************************************
//                              Recursive Approach
// ****************************************************************************


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
            let author = collection[i]["userNickname"]
                .as_str().unwrap().to_string();
            let title = collection[i]["title"]
                .as_str().unwrap().to_string();
            let text = collection[i]["reviewText"]
                .as_str().unwrap().to_string();
            let time = collection[i]["reviewSubmissionTime"]
                .as_str().unwrap().parse::<DateTime<Utc>>().unwrap();
        
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