use std::env;
use std::io::BufReader;
use std::fs;
use std::ffi::OsStr;
use std::sync::{ Arc, Mutex };
use std::thread;
use std::time::Instant;

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
    let a = args.next().unwrap();

    let mut times = vec![];
    let i = 1;
    for _ in 0..i {
        let a = a.clone();
        // let mut files: Arc<Mutex<Vec<String>>> = Arc::new(
        //     Mutex::new(Vec::new())
        // );
        
        // let files_c = files.clone();
        let mut files = vec![];

        let start = Instant::now();
        traverse_dir(a, &mut files);
        // let handle = thread::spawn(
        //     move || {mt_traverse_dir(a, files_c)}
        // );
        // handle.join().unwrap();
        let duration = start.elapsed();
        times.push(duration.as_micros());
    }
    
    println!("Durations: {:#?}", times);
    let avg = times.iter()
    .fold(0, |acc, x| acc + x) / i;
    println!("Durations: {:#?}", avg);

    // handles.push(handle);
    // for handle in handles {
    //     handle.join().unwrap();
    // }

    // println!("Total num files: {}", files.lock().unwrap().len());
    // println!("Files: {:#?}", files.lock().unwrap());

    // let mut files: Vec<String> = vec![];
    // traverse_dir(root_dir, &mut files);
    // println!("Total Files: {:#?}", files);
    let s = "/Users/milton/Desktop/HotelDataParser/data/reviews/SF/review491.json".to_string();
    process_files(s);
}

fn process_files(file_path: String) {
    let file = fs::File::open(file_path).unwrap();
    let reader = BufReader::new(file);
    let val: serde_json::Value = serde_json::from_reader(reader).unwrap();
    let collection = &val["reviewDetails"]["reviewCollection"]["review"];
    let num_reviews = val["reviewDetails"]["numberOfReviewsInThisPage"]
                                .as_u64().unwrap() as usize;
    
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

fn traverse_dir(root_dir: String, files_list:  &mut Vec<String>) {
    let entries = fs::read_dir(root_dir).unwrap();

    for entry in entries {
        let entry_path = entry.as_ref().unwrap()
                                    .path().as_path().to_owned();
        if entry.as_ref().unwrap().path().is_dir() {
            traverse_dir(
                entry_path.into_os_string().into_string().unwrap(),
                files_list
            );
        } else {
            let file_extension = entry_path.extension().unwrap_or(
                OsStr::new("No Extension")
            );
            
            if file_extension == "json" {
                files_list.push(
                    entry_path.file_name().unwrap()
                    .to_os_string().into_string().unwrap()
                );
            }
        }
    }
}

fn mt_traverse_dir(root_dir: String, files_list:  Arc<Mutex<Vec<String>>>) {
    let entries = fs::read_dir(&root_dir).unwrap();
    let mut handles = vec![];

    for entry in entries {
        let entry_path = entry.as_ref().unwrap()
                                    .path().as_path().to_owned();
        if entry.as_ref().unwrap().path().is_dir() {
            let files_list = files_list.clone();
            let root_dir = entry_path.into_os_string()
                                        .into_string().unwrap();

            let handle = thread::spawn(
                move || {mt_traverse_dir(root_dir, files_list)}
            );
            handles.push(handle);
        } else {
            let file_extension = entry_path.extension().unwrap_or(
                OsStr::new("No Extension")
            );
            
            if file_extension == "json" {
                files_list.lock().unwrap().push(
                    entry_path.file_name().unwrap()
                    .to_os_string().into_string().unwrap()
                );
            }
        }
    }

    for handle in handles {
        handle.join().unwrap();
    }
}