use std::env;
use std::sync::{ Arc, Mutex };
use std::thread;
use std::time::Instant;
use std::collections::HashMap;

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
    let mut reviews_map: Arc<Mutex<HashMap<u32, Vec<Review>>>> = 
        Arc::new(Mutex::new(HashMap::new()));
    let reviews_dir_path = r_dir_path.clone();    
    let files_c = reviews_map.clone();

    let start = Instant::now();
    let handle = thread::spawn(
        move || {mt_traverse_dir(reviews_dir_path, files_c)}
    );
    handle.join().unwrap();
    let duration = start.elapsed();

    println!("\nMultiThreading Reviews Parsing Duration: {:#?}", duration);

    let mut hotels_map: Arc<Mutex<HashMap<u32, Hotel>>> = 
        Arc::new(Mutex::new(HashMap::new()));
    let hotels_dir_path = h_dir_path.clone();    
    let files_c = hotels_map.clone();

    let start = Instant::now();
    let handle = thread::spawn(
        move || {mt_traverse_h_dir(hotels_dir_path, files_c)}
    );
    handle.join().unwrap();
    let duration = start.elapsed();

    println!("\nMultiThreading Hotels Parsing Duration: {:#?}", duration);

    // Recursive approach
    // let mut reviews_map: HashMap<u32, Vec<Review>> = HashMap::new();
    // let reviews_dir_path = dir_path.clone();

    // let start = Instant::now();
    // traverse_dir(reviews_dir_path, &mut reviews_map);
    // let duration = start.elapsed();
    // println!("Recursive Duration: {:#?}", duration);
}