use std::env;
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
        "/Users/milton/Desktop/HotelDataParser/data/reviews/SF"
    );

    get_files(root_dir, &mut files);

    println!("Files: {:#?}", files);
}

// 1. get path
// 2. get dir contents
// 3. if is file add to files list
// 4. else check contents of dir

fn get_files(root_dir: &Path, files_list:  &mut Vec<String>) {
    let entries = fs::read_dir(root_dir).unwrap();
    for entry in entries {
        if entry.as_ref().unwrap().path().is_file() {
            let file_path = entry.as_ref().unwrap()
                                    .path().as_path().to_owned();
            let file_name = file_path.file_name().unwrap();
            let file_extension = file_path.extension().unwrap_or(
                OsStr::new("No Extension")
            );

            // println!("File Path: {:#?}", file_path);
            // println!("File Name: {:#?}", file_name);
            // println!("File Ext: {:#?}", file_extension);
    
            if file_extension == "json" {
                files_list.push(file_name.to_os_string().into_string().unwrap());
            }
        } else {
            println!("Not a file: {:#?}", entry.as_ref().unwrap().file_name());
        }
    }
}