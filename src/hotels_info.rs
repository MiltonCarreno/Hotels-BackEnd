use std::collections::HashMap;
use chrono::prelude::*;

#[derive(Debug)]
pub struct Hotel {
    pub hotel_id: u32,
    pub name: String,
    pub address: String,
    pub city: String,
    pub province: String,
    pub country: String,
}

impl Hotel {
    fn copy(&self) -> Hotel {
        let hotel_id = self.hotel_id.clone();
        let name = self.name.clone();
        let address = self.address.clone();
        let city = self.city.clone();
        let province = self.province.clone();
        let country = self.country.clone();
        return Hotel { hotel_id, name, address, city, province, country };
    }
}

#[derive(Debug)]
pub struct Review {
    pub hotel_id: u32,
    pub review_id: String,
    pub rating: u32,
    pub author: String,
    pub title: String,
    pub text: String,
    pub time: DateTime<Utc>,
}

impl Review {
    fn copy(&self) -> Review {
        let hotel_id = self.hotel_id.clone();
        let review_id = self.review_id.clone();
        let rating = self.rating.clone();
        let author = self.author.clone();
        let title = self.title.clone();
        let text = self.text.clone();
        let time = self.time.clone();
        return Review { hotel_id, review_id, rating,
                        author, title, text, time };
    }
}

pub struct HotelsInfo {
    hotels_map: HashMap<u32, Hotel>,
    reviews_map: HashMap<u32, Vec<Review>>,
}

impl HotelsInfo {
    pub fn new() -> HotelsInfo {
        let hotels: HashMap<u32, Hotel> = HashMap::new();
        let reviews: HashMap<u32, Vec<Review>> = HashMap::new();
        return HotelsInfo { hotels_map: hotels, reviews_map: reviews };
    }

    pub fn add_hotels(&mut self, hotels: HashMap<u32, Hotel>) {
        self.hotels_map.extend(hotels);
    }

    pub fn add_reviews(&mut self, hotel_id: u32, reviews: Vec<Review>) {
        match self.reviews_map.contains_key(&hotel_id) {
            true => {
                self.reviews_map.get_mut(&hotel_id)
                    .unwrap().extend(reviews);
            }
            false => {
                self.reviews_map.insert(hotel_id, reviews);
            }
        };
    }

    pub fn search_hotels(&self, hotel_id: u32) -> Option<Hotel> {
        return match self.hotels_map.contains_key(&hotel_id) {
            true => Some(self.hotels_map.get(&hotel_id).unwrap().copy()),
            false => None,
        }
    }

    pub fn search_reviews(&self, hotel_id: u32) -> Option<Vec<Review>> {
        return match self.reviews_map.contains_key(&hotel_id) {
            true => {
                let mut reviews: Vec<Review> = vec![];
                for r in self.reviews_map.get(&hotel_id).unwrap() {
                    reviews.push(r.copy());
                }
                Some(reviews)
            },
            false => None,
        }
    }
}