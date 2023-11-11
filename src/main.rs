use std::env;
use std::process;
use std::sync::{ Arc, Mutex };
use std::thread;

mod config;
mod hotels_info;

use config::Config;
use data_parser::hotels_info::HotelsInfo;
use data_parser::r_traverse_dir;
use data_parser::mt_traverse_dir;
use data_parser::Data;

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

    println!("\nMultithreading Approach: {:#?}", 
        info.lock().unwrap().search_hotels(20191).unwrap());
    println!("\nMultithreading Approach: {:#?}", 
        info.lock().unwrap().search_reviews(20191).unwrap()[10]);
}

fn r_processing(r_dir_path: &String, h_dir_path: &String) {
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