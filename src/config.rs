pub struct Config {
    pub hotels_path: String,
    pub reviews_path: String,
}

impl Config {
    pub fn build(mut args: impl Iterator<Item = String>
    ) -> Result<Config, &'static str> {
        args.next();
        
        if args.size_hint().0 != 2 {
            return Err("Wrong number of arguments, only need 2 arguments \
            (i.e. 'hotels' and 'reviews' directory paths)");
        }
        
        let hotels_path = match args.next() {
            Some(path) => path,
            None => return Err("Didn't get 'hotels' directory path"),
        };

        let reviews_path = match args.next() {
            Some(path) => path,
            None => return Err("Didn't get 'reviews' directory path"),
        };

        return Ok(Config {hotels_path, reviews_path});
    }
}