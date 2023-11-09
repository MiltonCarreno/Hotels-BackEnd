use std::io::BufReader;
use std::fs;
use std::ffi::OsStr;
use std::sync::{ Arc, Mutex };
use std::thread;
use std::collections::HashMap;
use chrono::prelude::*;

pub struct Config {
    pub hotels_path: String,
    pub reviews_path: String,
}

impl Config {
    pub fn build(args: &Vec<String>) -> Result<Config, &'static str>{
        if args.len() != 3 {
            return Err("Incorrect number of arguments");
        }

        let hotels_path = args[1].clone();
        let reviews_path = args[2].clone();
        return Ok(Config {hotels_path, reviews_path});
    }
}


#[derive(Debug)]
pub struct Hotel {
    pub hotel_id: u32,
    pub name: String,
    pub address: String,
    pub city: String,
    pub province: String,
    pub country: String,
}

#[derive(Debug)]
pub struct Review {
    pub hotel_id: u32,
    pub review_id: String,
    pub rating: u32,
    pub author: String,
    pub title: String,
    pub text: String,
    pub time: DateTime<Utc>,
}

// ****************************************************************************
//                              Multithreading Approach
// ****************************************************************************

/**
 * Multithreading approach to process hotel files
 * 
 * # Parameters:
 * - 'file_path': A string containing the file path to be processed.
 * - 'hotels_set': Hashmap to be populated with hotel ids (key) and their
 *      corresponding hotel (value). Hashmap held in Mutex to prevent
 *      race conditions, and Arc used to allow multiple thread-safe references.
 */
pub fn mt_process_hotels(file_path: String, hotels_set: Arc<Mutex<HashMap<u32, Hotel>>>) {
    let file = fs::File::open(file_path).unwrap();
    let reader = BufReader::new(file);
    let val: serde_json::Value = serde_json::from_reader(reader).unwrap();
    let collection = &val["sr"];
    let num_hotels = collection.as_array().unwrap().len();

    for i in 0..num_hotels {
        let hotel_id: u32 = collection[i]["id"]
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
    
       hotels_set.lock().unwrap().insert(
            hotel_id,
            Hotel {hotel_id, name, address,
                city, province, country}
        );
    }
}

/**
 * Multithreading approach to traversing directories
 * 
 * # Parameters:
 * - 'dir_path': A string containing the directory path to be traversed.
 * - 'reviews_set': Hashmap to be populated with hotel ids (key) and their
 *      corresponding reviews (value). Hashmap held in Mutex to prevent
 *      race conditions, and Arc used to allow multiple thread-safe references.
 */
pub fn mt_traverse_h_dir(dir_path: String, hotels_set: Arc<Mutex<HashMap<u32, Hotel>>>) {
    let entries = fs::read_dir(&dir_path).unwrap();
    let mut handles = vec![];

    for entry in entries {
        let entry_path = entry.as_ref().unwrap()
            .path().into_os_string().into_string().unwrap();
        let entry_extention = entry.as_ref().unwrap()
            .path().extension().unwrap_or(
                OsStr::new("No Extension")
            ).to_os_string().into_string().unwrap();
        let hotels_set = hotels_set.clone();

        if entry.as_ref().unwrap().path().is_dir() {
            let handle = thread::spawn(
                move || {mt_traverse_h_dir(entry_path, hotels_set);}
            );
            handles.push(handle);
        } else if entry_extention == "json" {    
            let handle = thread::spawn(
                move || {mt_process_hotels(entry_path, hotels_set);}
            );
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
pub fn mt_process_reviews(file_path: String, reviews_set: Arc<Mutex<HashMap<u32, Vec<Review>>>>) {
    let file = fs::File::open(file_path).unwrap();
    let reader = BufReader::new(file);
    let val: serde_json::Value = serde_json::from_reader(reader).unwrap();
    let collection = &val["reviewDetails"]["reviewCollection"]["review"];
    let num_reviews = val["reviewDetails"]["numberOfReviewsInThisPage"]
                                .as_u64().unwrap() as usize;
    if num_reviews > 0 {
        let mut reviews: Vec<Review> = vec![];
        for i in 0..num_reviews {
            let hotel_id: u32 = collection[i]["hotelId"]
                .as_str().unwrap().parse().unwrap();
            let review_id = collection[i]["reviewId"]
                .as_str().unwrap().to_string();
            let rating: u32 = collection[i]["ratingOverall"]
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
        reviews_set.lock().unwrap().insert(reviews[0].hotel_id, reviews);
    }
}

/**
 * Multithreading approach to traversing directories
 * 
 * # Parameters:
 * - 'dir_path': A string containing the directory path to be traversed.
 * - 'reviews_set': Hashmap to be populated with hotel ids (keys) and their
 *      corresponding reviews (values). Hashmap held in Mutex to prevent
 *      race conditions, and Arc used to allow multiple thread-safe references.
 */
pub fn mt_traverse_dir(dir_path: String, reviews_set: Arc<Mutex<HashMap<u32, Vec<Review>>>>) {
    let entries = fs::read_dir(&dir_path).unwrap();
    let mut handles = vec![];

    for entry in entries {
        let entry_path = entry.as_ref().unwrap()
            .path().into_os_string().into_string().unwrap();
        let entry_extention = entry.as_ref().unwrap()
            .path().extension().unwrap_or(
                OsStr::new("No Extension")
            ).to_os_string().into_string().unwrap();
        let reviews_set = reviews_set.clone();

        if entry.as_ref().unwrap().path().is_dir() {
            let handle = thread::spawn(
                move || {mt_traverse_dir(entry_path, reviews_set);}
            );
            handles.push(handle);
        } else if entry_extention == "json" {    
            let handle = thread::spawn(
                move || {mt_process_reviews(entry_path, reviews_set);}
            );
            handles.push(handle);
        }
    }

    for handle in handles {
        handle.join().unwrap();
    }
}

// ****************************************************************************
//                              Recursive Approach
// ****************************************************************************


pub fn traverse_dir(dir_path: String, reviews_set: &mut HashMap<u32, Vec<Review>>) {
    let entries = fs::read_dir(dir_path).unwrap();

    for entry in entries {
        let entry_path = entry.as_ref().unwrap()
            .path().into_os_string().into_string().unwrap();
        let entry_extention = entry.as_ref().unwrap()
            .path().extension().unwrap_or(
                OsStr::new("No Extension")
            ).to_os_string().into_string().unwrap();    
                                    
        if entry.as_ref().unwrap().path().is_dir() {
            traverse_dir(entry_path, reviews_set);
        } else if entry_extention == "json" {
            process_reviews(entry_path, reviews_set);
        }
    }
}

pub fn process_reviews(file_path: String, reviews_set: &mut HashMap<u32, Vec<Review>>) {
    let file = fs::File::open(file_path).unwrap();
    let reader = BufReader::new(file);
    let val: serde_json::Value = serde_json::from_reader(reader).unwrap();
    let collection = &val["reviewDetails"]["reviewCollection"]["review"];
    let num_reviews = val["reviewDetails"]["numberOfReviewsInThisPage"]
                                .as_u64().unwrap() as usize;
    if num_reviews > 0 {
        let mut reviews: Vec<Review> = vec![];
        for i in 0..num_reviews {
            let hotel_id: u32 = collection[i]["hotelId"]
                .as_str().unwrap().parse().unwrap();
            let review_id = collection[i]["reviewId"]
                .as_str().unwrap().to_string();
            let rating: u32 = collection[i]["ratingOverall"]
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
        reviews_set.insert(reviews[0].hotel_id, reviews);
    }
}