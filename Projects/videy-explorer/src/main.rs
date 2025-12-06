use serde::{Deserialize, Serialize};
use serde_json;

const TARGET_URL: &'static str = "https://cdn.videy.co";

#[derive(Serialize, Deserialize)]
pub struct SessionData {
    pub last_path: String,
}

const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";

fn increment_path(path: &str) -> Option<String> {
    let mut chars: Vec<char> = path.chars().collect();
    let mut i = chars.len();

    while i > 0 {
        i -= 1;
        let c = chars[i];
        if let Some(pos) = CHARSET.iter().position(|&x| x as char == c) {
            if pos + 1 < CHARSET.len() {
                chars[i] = CHARSET[pos + 1] as char;
                return Some(chars.into_iter().collect());
            } else {
                chars[i] = CHARSET[0] as char;
            }
        } else {
            // If character not in charset, maybe just keep it or error? 
            // For now, let's assume valid charset or just return None to stop weirdness.
            return None;
        }
    }
    
    // If we wrapped around completely (e.g. 999 -> 000), we might want to return None or start over.
    // For fixed length, this means we exhausted the space.
    None
}

pub struct RequestIter {
    current_path: String,
    extension: String,
}

impl RequestIter {
    pub fn new(extension: String) -> RequestIter {
        let last_path = if let Ok(session_str) = std::fs::read_to_string("session.json") {
             if let Ok(session_data) = serde_json::from_str::<SessionData>(&session_str) {
                 session_data.last_path
             } else {
                 "AAAAAAAAA".to_string()
             }
        } else {
            "AAAAAAAAA".to_string()
        };

        RequestIter { 
            current_path: last_path,
            extension,
        }
    }

    pub async fn check_url(url: &str) -> bool {
        if let Ok(req) = reqwest::get(url).await {
            req.status().is_success()
        } else {
            false
        }
    }
}

impl Iterator for RequestIter {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        let url = format!("{}/{}.{}", TARGET_URL, self.current_path, self.extension);
        
        // Prepare next path for the next call
        if let Some(next_p) = increment_path(&self.current_path) {
            self.current_path = next_p;
        } else {
            return None; // Stop if overflow
        }

        Some(url)
    }
}

use std::sync::Arc;
use tokio::sync::{mpsc, Semaphore};
use tokio::io::AsyncWriteExt;

#[tokio::main]
pub async fn main() {
    let mut iter = RequestIter::new("mp4".to_string());
    let mut count = 0;

    println!("Starting exploration from: {}", iter.current_path);

    // Channel for writing found URLs to file
    let (tx, mut rx) = mpsc::channel::<String>(100);

    // Spawn a separate task to handle file writing
    tokio::spawn(async move {
        if let Ok(mut file) = tokio::fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open("list_videos.txt")
            .await 
        {
            while let Some(url) = rx.recv().await {
                println!("FOUND: {}", url);
                if let Err(e) = file.write_all(format!("{}\n", url).as_bytes()).await {
                    eprintln!("Failed to write to file: {}", e);
                }
            }
        }
    });

    // Limit concurrent requests
    const MAX_CONCURRENT_REQUESTS: usize = 50;
    let semaphore = Arc::new(Semaphore::new(MAX_CONCURRENT_REQUESTS));

    while let Some(url) = iter.next() {
        let permit = semaphore.clone().acquire_owned().await.unwrap();
        let tx_clone = tx.clone();
        
        tokio::spawn(async move {
            if RequestIter::check_url(&url).await {
                let _ = tx_clone.send(url).await;
            }
            drop(permit);
        });

        count += 1;
        if count % 10 == 0 {
            let session = SessionData {
                last_path: iter.current_path.clone(),
            };
            if let Ok(json) = serde_json::to_string(&session) {
                let _ = std::fs::write("session.json", json);
                println!("Saved progress at {}", iter.current_path);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_increment_path() {
        assert_eq!(increment_path("AAAAAAAAA"), Some("AAAAAAAAB".to_string()));
        assert_eq!(increment_path("AAAAAAAAB"), Some("AAAAAAAAC".to_string()));
        // Check boundary of charset
        // CHARSET: A...Z a...z 0...9
        // 'Z' -> 'a'
        assert_eq!(increment_path("AAAAAAAAZ"), Some("AAAAAAAAa".to_string()));
        // 'z' -> '0'
        assert_eq!(increment_path("AAAAAAAAz"), Some("AAAAAAAA0".to_string()));
        // '9' -> carry over to next char
        assert_eq!(increment_path("AAAAAAA9"), Some("AAAAAABA".to_string()));
        
        // Test carry over multiple
        assert_eq!(increment_path("AAAAAA99"), Some("AAAAABAA".to_string()));
    }
}
