use std::env;
use std::path::PathBuf;
use std::process;
use std::fs;
use std::path::Path;
use std::ffi::OsStr;
use std::sync::{ Arc, Mutex };
use std::thread;

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
    
    let mut files: Arc<Mutex<Vec<String>>> = Arc::new(
        Mutex::new(Vec::new())
    );
    
    let mut handles = vec![];

    for dir in args {
        let files_c = files.clone();
        let handle = thread::spawn(
            move || {mt_get_files(dir, files_c)}
        );
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Total num files: {}", files.lock().unwrap().len());
    println!("Files: {:#?}", files.lock().unwrap());

    // let mut files: Vec<String> = vec![];
    // get_files(root_dir, &mut files);
    // println!("Total Files: {:#?}", files);

}

fn get_files(root_dir: String, files_list:  &mut Vec<String>) {
    let mut dirs: Vec<PathBuf> = Vec::new();
    let entries = fs::read_dir(root_dir).unwrap();
    for entry in entries {
        let entry_path = entry.as_ref().unwrap()
                                    .path().as_path().to_owned();
        if entry.as_ref().unwrap().path().is_file() {
            let file_extension = entry_path.extension().unwrap_or(
                OsStr::new("No Extension")
            );
            
            if file_extension == "json" {
                files_list.push(
                    entry_path.file_name().unwrap()
                    .to_os_string().into_string().unwrap()
                );
            }
        } else {
            println!("Dir Name: {:#?}", entry.as_ref().unwrap().file_name());
            dirs.push(entry_path);
        }
    }

    println!("Num of files: {}", files_list.len());
    for dir in dirs {
        println!("\nGoing into dir: {:#?}", dir);
        get_files(dir.into_os_string().into_string().unwrap()
        , files_list);
    }
}

fn mt_get_files(root_dir: String, files_list:  Arc<Mutex<Vec<String>>>) {
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
                move || {mt_get_files(root_dir, files_list)}
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