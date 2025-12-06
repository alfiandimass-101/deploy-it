use serde::{Deserialize, Serialize};

pub struct RequestIter<'a> {
    url: String,
    extension: &'a str,
}

impl<'a> RequestIter<'a> {
    pub fn new(url: &'a str) -> RequestIter<'a> {
        RequestIter {
            url: url.to_string(),
            extension: "mp4",
        }
    }

    pub async fn valid_url(url: &'a str) -> bool {
        let req = reqwest::get(url).await.unwrap();

        req.status().is_success()
    }
}

impl<'a> Iterator for RequestIter<'a> {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {}
}

#[tokio::main]
pub async fn main() {
    println!("Hello, world!");
}
