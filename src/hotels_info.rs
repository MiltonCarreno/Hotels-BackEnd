use std::collections::HashMap;
use chrono::prelude::*;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, sqlx::FromRow, Serialize, Deserialize)]
pub struct Hotel {
    pub hotel_id: i32,
    pub name: String,
    pub address: String,
    pub city: String,
    pub province: String,
    pub country: String,
}

#[derive(Debug, Clone)]
pub struct Review {
    pub hotel_id: i32,
    pub review_id: String,
    pub rating: i32,
    pub author: String,
    pub title: String,
    pub text: String,
    pub time: DateTime<Utc>,
}

#[derive(Debug)]
pub struct HotelsInfo {
    hotels_map: HashMap<i32, Hotel>,
    reviews_map: HashMap<i32, Vec<Review>>,
}

impl HotelsInfo {
    pub fn new() -> HotelsInfo {
        let hotels: HashMap<i32, Hotel> = HashMap::new();
        let reviews: HashMap<i32, Vec<Review>> = HashMap::new();
        return HotelsInfo { hotels_map: hotels, reviews_map: reviews };
    }

    pub fn get_hotels(&self) -> HashMap<i32, Hotel> {
        self.hotels_map.clone()
    }

    pub fn get_reviews(&self) -> HashMap<i32, Vec<Review>> {
        self.reviews_map.clone()
    }

    pub fn add_hotels(&mut self, hotels: HashMap<i32, Hotel>) {
        self.hotels_map.extend(hotels);
    }

    pub fn add_reviews(&mut self, hotel_id: i32, reviews: Vec<Review>) {
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

    pub fn search_hotels(&self, hotel_id: i32) -> Option<Hotel> {
        return match self.hotels_map.contains_key(&hotel_id) {
            true => Some(self.hotels_map.get(&hotel_id).unwrap().clone()),
            false => None,
        }
    }

    pub fn search_reviews(&self, hotel_id: i32) -> Option<Vec<Review>> {
        return match self.reviews_map.contains_key(&hotel_id) {
            true => {
                let mut reviews: Vec<Review> = vec![];
                for r in self.reviews_map.get(&hotel_id).unwrap() {
                    reviews.push(r.clone());
                }
                Some(reviews)
            },
            false => None,
        }
    }
}