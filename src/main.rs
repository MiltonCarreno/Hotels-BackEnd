use std::env;
use std::process;
use std::sync::{ Arc, Mutex };
use std::thread;
use std::time::Instant;

use data_parser::*;

fn main() {
    // Parse arguments
    let config = Config::build(env::args())
        .unwrap_or_else(|err| {
            println!("\nError parsing arguments: {err}\n");
            process::exit(1);
        });
    
    // Multithreading approach
    mt_processing(
        &config.reviews_path,
        &config.hotels_path,
    );

    // Recursive approach
    r_processing(
        &config.reviews_path,
        &config.hotels_path,
    );
}

fn mt_processing(r_dir_path: &String, h_dir_path: &String) {
    // HotelsInfo Struct
    let mut info: Arc<Mutex<HotelsInfo>> = 
        Arc::new(Mutex::new(HotelsInfo::new()));
        
    // Reviews Files Parsing
    let reviews_dir_path = r_dir_path.clone();
    let info_copy = info.clone();
    let data = Data::Reviews;
        
    let start = Instant::now();
    let handle = thread::spawn(move || {
        mt_traverse_dir(reviews_dir_path, info_copy, data)
    });
    handle.join().unwrap();
    let r_duration = start.elapsed();

    // Hotels Files Parsing
    let hotels_dir_path = h_dir_path.clone();
    let info_copy = info.clone();
    let data = Data::Hotels;
        
    let start = Instant::now();
    let handle = thread::spawn(move || {
        mt_traverse_dir(hotels_dir_path, info_copy, data)
    });
    handle.join().unwrap();
    let h_duration = start.elapsed();

    println!("\nMultiThreading (Reviews) Duration: {:#?}", r_duration);
    println!("MultiThreading (Hotels) Duration: {:#?}", h_duration);

    // println!("\nReview: {:#?}", info.lock().unwrap().search_reviews(10323).unwrap().len());
    // println!("\nHotel: {:#?}", info.lock().unwrap().search_hotels(10323));
}

fn r_processing(r_dir_path: &String, h_dir_path: &String) {
    // HotelsInfo Struct
    let mut info2 = HotelsInfo::new();
    let reviews_dir_path = r_dir_path.clone();
    let data = Data::Reviews;

    // Reviews Parsing
    let start = Instant::now();
    r_traverse_dir(reviews_dir_path, &mut info2, &data);
    let r_duration = start.elapsed();

    let hotels_dir_path = h_dir_path.clone();
    let data = Data::Hotels;

    // Hotels Parsing
    let start = Instant::now();
    r_traverse_dir(hotels_dir_path, &mut info2, &data);
    let h_duration = start.elapsed();

    println!("\nRecursive (Reviews) Duration: {:#?}", r_duration);
    println!("Recursive (Hotels) Duration: {:#?}\n", h_duration);

    // println!("\nReview: {:#?}", info2.search_reviews(10323).unwrap().len());
    // println!("\nHotel: {:#?}", info2.search_hotels(10323));
}