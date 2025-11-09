use reqwest::{Client, header::{HeaderMap, HeaderValue}};

use crate::utils::ServerSummary;

pub mod utils;

const PHPSESID: &'static str = "7rkskb8ils3s8su7jrrh83q354";
const PAGE: &'static str = "https://magmanode.com";
const PANEL: &'static str = "https://panel.magmanode.com";
const AUTH_TOKEN: &str = "ptlc_8JGKmhuz2JydQ0Ax8Ko7MKopPTeWln8mJi2cmZm0Uam";

pub async fn get_required_server_data() -> Result<ServerSummary, serde_json::Error> {
    let mut headers = HeaderMap::new();

    let auth_value = format!("Bearer {}", AUTH_TOKEN);
    headers.insert(
        "Authorization", 
        HeaderValue::from_str(&auth_value).unwrap()
    );

    headers.insert("Accept", HeaderValue::from_static("application/json"));
    headers.insert("Content-Type", HeaderValue::from_static("application/json"));

    let client = Client::new();
    let url = format!("{}/api/client", PANEL);

    let result = client
        .get(url)
        .headers(headers)
        .send()
        .await.unwrap();
    serde_json::from_str::<ServerSummary>(&result.text().await.unwrap())
}

pub async fn execute_auto_start() -> anyhow::Result<()> {
    
    Ok(())
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let (status, server_data) = match get_required_server_data().await {
        Ok(result) => (true, result),
        Err(_) => (false, ServerSummary::default())
    };
    if status {
        
    }
    Ok(())
}