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
    pub fn build(mut args: impl Iterator<Item = String>
    ) -> Result<Config, &'static str> {
        args.next();
        
        if args.size_hint().0 != 2 {
            return Err("Wrong number of arguments, only need 2 arguments \
            (i.e. 'hotels' and 'reviews' directory paths)");
        }
        
        let hotels_path = match args.next() {
            Some(path) => path,
            None => return Err("Didn't get 'hotels' directory path"),
        };

        let reviews_path = match args.next() {
            Some(path) => path,
            None => return Err("Didn't get 'reviews' directory path"),
        };

        return Ok(Config {hotels_path, reviews_path});
    }
}

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

#[derive(Debug)]
pub struct Hotel {
    pub hotel_id: u32,
    pub name: String,
    pub address: String,
    pub city: String,
    pub province: String,
    pub country: String,
}

impl Hotel {
    fn copy(&self) -> Hotel {
        let hotel_id = self.hotel_id.clone();
        let name = self.name.clone();
        let address = self.address.clone();
        let city = self.city.clone();
        let province = self.province.clone();
        let country = self.country.clone();
        return Hotel { hotel_id, name, address, city, province, country };
    }
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

impl Review {
    fn copy(&self) -> Review {
        let hotel_id = self.hotel_id.clone();
        let review_id = self.review_id.clone();
        let rating = self.rating.clone();
        let author = self.author.clone();
        let title = self.title.clone();
        let text = self.text.clone();
        let time = self.time.clone();
        return Review { hotel_id, review_id, rating,
                        author, title, text, time };
    }
}

pub struct HotelsInfo {
    hotels_map: HashMap<u32, Hotel>,
    reviews_map: HashMap<u32, Vec<Review>>,
}

impl HotelsInfo {
    pub fn new() -> HotelsInfo {
        let hotels: HashMap<u32, Hotel> = HashMap::new();
        let reviews: HashMap<u32, Vec<Review>> = HashMap::new();
        return HotelsInfo { hotels_map: hotels, reviews_map: reviews };
    }

    pub fn add_hotels(&mut self, hotels: HashMap<u32, Hotel>) {
        self.hotels_map.extend(hotels);
    }

    pub fn add_reviews(&mut self, hotel_id: u32, reviews: Vec<Review>) {
        match self.reviews_map.contains_key(&hotel_id) {
            true => {
                self.reviews_map.get_mut(&hotel_id)
                    .unwrap().extend(reviews);
            }
            false => {
                self.reviews_map.insert(hotel_id, reviews);
            }
        };
    }

    pub fn search_hotels(&self, hotel_id: u32) -> Option<Hotel> {
        return match self.hotels_map.contains_key(&hotel_id) {
            true => Some(self.hotels_map.get(&hotel_id).unwrap().copy()),
            false => None,
        }
    }

    pub fn search_reviews(&self, hotel_id: u32) -> Option<Vec<Review>> {
        return match self.reviews_map.contains_key(&hotel_id) {
            true => {
                let mut reviews: Vec<Review> = vec![];
                for r in self.reviews_map.get(&hotel_id).unwrap() {
                    reviews.push(r.copy());
                }
                Some(reviews)
            },
            false => None,
        }
    }
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
pub fn mt_process_reviews(
    file_path: String, hotels_info: Arc<Mutex<HotelsInfo>>) {
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
pub fn mt_process_hotels(
    file_path: String, hotels_info: Arc<Mutex<HotelsInfo>>) {
    let file = fs::File::open(file_path).unwrap();
    let reader = BufReader::new(file);
    let val: serde_json::Value = serde_json::from_reader(reader).unwrap();
    let collection = &val["sr"];
    let num_hotels = collection.as_array().unwrap().len();
    let mut hotels: HashMap<u32, Hotel> = HashMap::new();

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
        hotels_info.add_reviews(reviews[0].hotel_id, reviews);
    }
}

pub fn r_process_hotels(file_path: String, hotels_info: &mut HotelsInfo) {
    let file = fs::File::open(file_path).unwrap();
    let reader = BufReader::new(file);
    let val: serde_json::Value = serde_json::from_reader(reader).unwrap();
    let collection = &val["sr"];
    let num_hotels = collection.as_array().unwrap().len();
    let mut hotels: HashMap<u32, Hotel> = HashMap::new();

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
    
        hotels.insert(
            hotel_id,
            Hotel {hotel_id, name, address,
                city, province, country}
        );
    }
    hotels_info.add_hotels(hotels);
}