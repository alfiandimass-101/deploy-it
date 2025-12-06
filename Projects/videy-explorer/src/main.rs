use serde::{Deserialize, Serialize};
use serde_json;

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
        // Read session data from session.json
        let session_str =
            std::fs::read_to_string("session.json").expect("Failed to read session.json");
        let session_data: SessionData =
            serde_json::from_str(&session_str).expect("Failed to parse session.json");
        let last_path = session_data.last_path;
        // Use the last_path as base URL
        let url = format!("{}/{}.{}", TARGET_URL, last_path, extension);
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
