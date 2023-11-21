pub const CREATE_USERS_TABLE: &str = "\
    create table if not exists users(id INT AUTO_INCREMENT, \
    username VARCHAR(15) NOT NULL, email VARCHAR(255) NOT NULL, \
    PRIMARY KEY(id));";

pub const CREATE_HOTELS_TABLE: &str = "\
    create table if not exists hotels(hotel_id INT NOT NULL, \
    name VARCHAR(200) NOT NULL, address VARCHAR(200) NOT NULL, \
    city VARCHAR(100) NOT NULL, province VARCHAR(100) NOT NULL, \
    country VARCHAR(100) NOT NULL, PRIMARY KEY(hotel_id));";

pub const CREATE_REVIEWS_TABLE: &str = "\
    create table if not exists reviews(review_id VARCHAR(100) NOT NULL, \
    hotel_id INT NOT NULL, rating INT NOT NULL, \
    author VARCHAR(100), title VARCHAR(200), text VARCHAR(5000), \
    time VARCHAR(100) NOT NULL, PRIMARY KEY(review_id), \
    FOREIGN KEY(hotel_id) REFERENCES hotels(hotel_id))";

pub const CREATE_USER_REVIEWS_TABLE: &str = "\
    create table if not exists user_reviews(review_id INT AUTO_INCREMENT, \
    user_id INT NOT NULL, hotel_id INT NOT NULL, title VARCHAR(200), text VARCHAR(5000), \
    PRIMARY KEY(review_id), FOREIGN KEY(user_id) REFERENCES users(id), \
    FOREIGN KEY(hotel_id) REFERENCES hotels(hotel_id))";

pub const INSERT_USERS: &str = "insert into users(username, email) values \
    ('charles', 'cMingus@moaning.ca'), ('willie', 'wColon@miSueno.co'), \
    ('chet', 'cBaker@fallingTooEasily.us'), ('dave', 'dBrubeck@take5.po'), \
    ('bill', 'bEvans@foolishHeart.es'), ('hector', 'hLavoe@elCantante.com');";

pub const INSERT_HOTEL: &str = "insert into hotels(hotel_id, name, address, \
    city, province, country) values (?, ?, ?, ?, ?, ?);";

pub const INSERT_REVIEW: &str = "insert into reviews(review_id, hotel_id, \
    rating, author, title, text, time) values (?, ?, ?, ?, ?, ?, ?);";

pub const INSERT_USER_REVIEW: &str = "insert into user_reviews(user_id, \
    hotel_id, title, text) values (?, ?, ?, ?);";

pub const INSERT_USER: &str = "insert into users(username, email) values \
    (?, ?);";

pub const SELECT_USER: &str = "select * from users where id = ?";

pub const SELECT_HOTEL: &str = "select * from hotels where hotel_id = ?";

pub const SELECT_HOTEL_REVIEWS: &str = "select * from reviews where \
    hotel_id = ?";

pub const SELECT_LIKE_HOTELS: &str = "select * from hotels where name like ?";

pub const SELECT_ALL_USERS: &str = "select * from users";

pub const SELECT_ALL_HOTELS: &str = "select * from hotels";

pub const SELECT_ALL_USER_REVIEWS: &str = "select * from user_reviews";

pub const DELETE_USER: &str = "delete from users where id = ?";

pub const DELETE_USER_REVIEW: &str = "delete from user_reviews where \
    review_id = ?";

pub const UPDATE_USER: &str = "update users set username = ?,\
    email = ? where id = ?";

pub const DROP_USERS_TABLE: &str = "drop table users;";

pub const DROP_HOTELS_TABLE: &str = "drop table hotels;";

pub const DROP_REVIEWS_TABLE: &str = "drop table reviews;";