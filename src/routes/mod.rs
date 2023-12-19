pub mod hotels;
pub mod reviews;
pub mod users;
pub mod user_reviews;
mod utils;
mod auth_token;

pub use hotels::*;
pub use reviews::*;
pub use users::*;
pub use user_reviews::*;

pub async fn root() -> String {
    "Server is up and running".to_string()
}

// user: add, update, delete
// hotels: get, get like
// reviews: get by hotel
// user_reviews: add, update (to do), get all, get by hotel (to do), delete