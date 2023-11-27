use std::sync::{ Arc, Mutex };
use std::thread;
use std::collections::HashMap;

pub mod multithreaded;
pub mod recursive;
mod utils;

use crate::hotels_info::*;
use multithreaded::*;
use recursive::*;
use utils::*;

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
    let mut info = HotelsInfo::new();

    r_traverse_dir(
        r_dir_path.clone(), &mut info, &Data::Reviews
    );

    r_traverse_dir(
        h_dir_path.clone(), &mut info, &Data::Hotels
    );

    println!("\nRecursive Approach: {:#?}", 
        info.search_hotels(20191).unwrap());
    println!("\nRecursive Approach: {:#?}", 
        info.search_reviews(20191).unwrap()[10]);
}
