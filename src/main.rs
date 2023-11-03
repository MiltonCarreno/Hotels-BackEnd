use std::env;
use std::path::PathBuf;
use std::process;
use std::fs;
use std::path::Path;
use std::ffi::OsStr;

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
    let args: Vec<String> = env::args().collect();
    println!("\nArgs Len: {} \nPath 1: {} \nPath 2: {}",
    args.len(), args[1], args[2]);

    let config = Config::build(&args);
    if let Err(e) = config {
        println!("Error: {e}");
        process::exit(1);
    }

    let mut files: Vec<String> = Vec::new();
    let root_dir = Path::new(
        "/Users/milton/Desktop/HotelDataParser/data/reviews"
    );

    get_files(root_dir, &mut files);

    println!("Total num files: {}", files.len());
    println!("Files: {:#?}", files);
}

// 1. get path
// 2. get dir contents
// 3. if is file add to files list
// 4. else check contents of dir

fn get_files(root_dir: &Path, files_list:  &mut Vec<String>) {
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
        get_files(&dir, files_list);
    }
}