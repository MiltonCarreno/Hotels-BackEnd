use std::env;
use std::sync::{ Arc, Mutex };
use std::thread;
use std::time::Instant;

use data_parser::*;

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
    let h_dir_path = args.next().unwrap();
    let r_dir_path = args.next().unwrap();
    
    // Multithreading approach
    
    // HotelInfo Struct
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

    // Recursive approach
    let mut info2 = HotelsInfo::new();
    let reviews_dir_path = r_dir_path.clone();
    let data = Data::Reviews;

    let start = Instant::now();
    r_traverse_dir(reviews_dir_path, &mut info2, &data);
    let r_duration = start.elapsed();

    let hotels_dir_path = h_dir_path.clone();
    let data = Data::Hotels;

    let start = Instant::now();
    r_traverse_dir(hotels_dir_path, &mut info2, &data);
    let h_duration = start.elapsed();

    println!("\nRecursive (Reviews) Duration: {:#?}", r_duration);
    println!("Recursive (Hotels) Duration: {:#?}\n", h_duration);

    // println!("\nReview: {:#?}", info2.search_reviews(10323).unwrap().len());
    // println!("\nHotel: {:#?}", info2.search_hotels(10323));
}