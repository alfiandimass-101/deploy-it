use serde::{Deserialize, Serialize};
use std::fs;
const TARGET_URL: &'static str = "https://cdn.videy.co";

#[derive(Serialize, Deserialize)]
pub struct SessionData {
    pub last_path: String,
}

pub struct RequestIter<'a> {
    url: String,
    extension: &'a str,
}

impl<'a> RequestIter<'a> {
    pub fn new(extension: &'a str) -> RequestIter<'a> {
        let last_path = SessionData::deserialize(deserializer);
        let url = format!("{}{}", TARGET_URL, extension);
        RequestIter { url, extension }
    }

    pub async fn valid_url(url: &'a str) -> bool {
        let req = reqwest::get(url).await.unwrap();

        req.status().is_success()
    }
}

// impl<'a> Iterator for RequestIter<'a> {
//     type Item = String;

//     fn next(&mut self) -> Option<Self::Item> {
//         if
//     }
// }

#[tokio::main]
pub async fn main() {
    println!("Hello, world!");
}
