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
    args.next();
    let dir_path = args.next().unwrap();
    
    // Multithreading approach
    let mut reviews_map: Arc<Mutex<HashMap<u32, Vec<Review>>>> = 
        Arc::new(Mutex::new(HashMap::new()));
    let reviews_dir_path = dir_path.clone();    
    let files_c = reviews_map.clone();

    let start = Instant::now();
    let handle = thread::spawn(
        move || {mt_traverse_dir(reviews_dir_path, files_c)}
    );
    handle.join().unwrap();
    let duration = start.elapsed();

    println!("\nMultiThreading Duration: {:#?}", duration);

    // Recursive approach
    let mut reviews_map: HashMap<u32, Vec<Review>> = HashMap::new();
    let reviews_dir_path = dir_path.clone();

    let start = Instant::now();
    traverse_dir(reviews_dir_path, &mut reviews_map);
    let duration = start.elapsed();
    println!("Recursive Duration: {:#?}", duration);
}