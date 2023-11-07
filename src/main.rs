use std::env;
use std::io::BufReader;
use std::fs;
use std::ffi::OsStr;
use std::sync::{ Arc, Mutex };
use std::thread;
use std::time::Instant;
use std::collections::HashMap;

struct Config {
    hotels_path: String,
    reviews_path: String,
}

impl Config {
    fn build(args: &Vec<String>) -> Result<Config, &'static str>{
        if args.len() != 3 {
            return Err("Incorrect number of arguments");
        }

        let hotels_path = args[1].clone();
        let reviews_path = args[2].clone();
        return Ok(Config {hotels_path, reviews_path});
    }
}

fn main() {
    // let args: Vec<String> = env::args().collect();
    // println!("\nArgs Len: {} \nPath 1: {} \nPath 2: {}",
    // args.len(), args[1], args[2]);

    // let config = Config::build(&args);
    // if let Err(e) = config {
    //     println!("Error: {e}");
    //     process::exit(1);
    // }

    let mut args = env::args();
    args.next();
    args.next();
    let dir_path = args.next().unwrap();
    
    // Multithreading approach
    let mut reviews_set: Arc<Mutex<HashMap<u32, Vec<Review>>>> = 
        Arc::new(Mutex::new(HashMap::new()));
    let reviews_dir_path = dir_path.clone();    
    let files_c = reviews_set.clone();

    let start = Instant::now();
    let handle = thread::spawn(
        move || {mt_traverse_dir(reviews_dir_path, files_c)}
    );
    handle.join().unwrap();
    let duration = start.elapsed();

    println!("Duration: {:#?}", duration);
    println!("{:#?}", reviews_set.lock().unwrap().get(&10323).unwrap().len());
    println!("{:#?}", reviews_set.lock().unwrap().get(&828).unwrap().len());
    println!("{:#?}", reviews_set.lock().unwrap().get(&10323).unwrap()[0]);
    println!("{:#?}", reviews_set.lock().unwrap().len());

    // Recursive approach
    let mut reviews_set: HashMap<u32, Vec<Review>> = HashMap::new();
    let reviews_dir_path = dir_path.clone();

    let start = Instant::now();
    traverse_dir(reviews_dir_path, &mut reviews_set);
    let duration = start.elapsed();
    println!("Duration: {:#?}", duration);

    println!("{:#?}", reviews_set.get(&10323).unwrap().len());
    println!("{:#?}", reviews_set.get(&828).unwrap().len());
    println!("{:#?}", reviews_set.get(&10323).unwrap()[0]);
    println!("{:#?}", reviews_set.len());
}

fn mt_process_reviews(file_path: String, review_set: Arc<Mutex<HashMap<u32, Vec<Review>>>>) {
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
                                .as_str().unwrap().to_string();
        
            reviews.push(
                Review { hotel_id, review_id, rating,
                        author, title, text, time }
            );
        }
        review_set.lock().unwrap().insert(reviews[0].hotel_id, reviews);
    }
}

fn process_reviews(file_path: String, review_set: &mut HashMap<u32, Vec<Review>>) {
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
                                .as_str().unwrap().to_string();
        
            reviews.push(
                Review { hotel_id, review_id, rating,
                        author, title, text, time }
            );
        }
        review_set.insert(reviews[0].hotel_id, reviews);
    }
}

#[derive(Debug)]
struct Review {
    hotel_id: u32,
    review_id: String,
    rating: u32,
    author: String,
    title: String,
    text: String,
    time: String,
}

fn traverse_dir(dir_path: String, review_set: &mut HashMap<u32, Vec<Review>>) {
    let entries = fs::read_dir(dir_path).unwrap();

    for entry in entries {
        let entry_path = entry.as_ref().unwrap()
            .path().into_os_string().into_string().unwrap();
        let entry_extention = entry.as_ref().unwrap()
            .path().extension().unwrap_or(
                OsStr::new("No Extension")
            ).to_os_string().into_string().unwrap();    
                                    
        if entry.as_ref().unwrap().path().is_dir() {
            traverse_dir(entry_path, review_set);
        } else if entry_extention == "json" {
            process_reviews(entry_path, review_set);
        }
    }
}

fn mt_traverse_dir(dir_path: String, review_set: Arc<Mutex<HashMap<u32, Vec<Review>>>>) {
    let entries = fs::read_dir(&dir_path).unwrap();
    let mut handles = vec![];

    for entry in entries {
        let entry_path = entry.as_ref().unwrap()
            .path().into_os_string().into_string().unwrap();
        let entry_extention = entry.as_ref().unwrap()
            .path().extension().unwrap_or(
                OsStr::new("No Extension")
            ).to_os_string().into_string().unwrap();
        let review_set = review_set.clone();

        if entry.as_ref().unwrap().path().is_dir() {
            let handle = thread::spawn(
                move || {mt_traverse_dir(entry_path, review_set)}
            );
            handles.push(handle);
        } else if entry_extention == "json" {    
            let handle = thread::spawn(
                move || {mt_process_reviews(entry_path, review_set);}
            );
            handles.push(handle);
        }
    }

    for handle in handles {
        handle.join().unwrap();
    }
}