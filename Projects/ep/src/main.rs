use reqwest::{
    Client,
    header::{HeaderMap, HeaderValue},
};

use crate::utils::ServerSummary;

pub mod utils;

const PHPSESID: &'static str = "7rkskb8ils3s8su7jrrh83q354";
const PAGE: &'static str = "https://magmanode.com";
const PANEL: &'static str = "https://panel.magmanode.com";
const AUTH_TOKEN: &str = "ptlc_8JGKmhuz2JydQ0Ax8Ko7MKopPTeWln8mJi2cmZm0Uam";

pub async fn get_required_server_data() -> Result<ServerSummary, serde_json::Error> {
    let mut headers = HeaderMap::new();

    let auth_value = format!("Bearer {}", AUTH_TOKEN);
    headers.insert("Authorization", HeaderValue::from_str(&auth_value).unwrap());

    headers.insert("Accept", HeaderValue::from_static("application/json"));
    headers.insert("Content-Type", HeaderValue::from_static("application/json"));

    let client = Client::new();
    let url = format!("{}/api/client", PANEL);

    let result = client.get(url).headers(headers).send().await.unwrap();
    let result_text = result.text().await.unwrap();
    println!("[REQUIRED SERVER DATA]: {}", &result_text);
    serde_json::from_str::<ServerSummary>(&result_text)
}

pub async fn execute_auto_start(server_uuid: &str) -> anyhow::Result<()> {
    let mut headers = HeaderMap::new();

    let auth_value = format!("Bearer {}", AUTH_TOKEN);
    headers.insert("Authorization", HeaderValue::from_str(&auth_value).unwrap());

    headers.insert("Accept", HeaderValue::from_static("application/json"));
    headers.insert("Content-Type", HeaderValue::from_static("application/json"));
    let client = Client::new()
        .post(format!("{PANEL}/api/client/servers/{server_uuid}/power"))
        .body("{\"signal\": \"start\"}")
        .headers(headers)
        .send()
        .await?;
    println!("[AUTO START EXECUTED]");
    Ok(())
}

pub async fn get_server_magma_id() -> Result<u64, &str> {
    let client = Client::new();
    let response = client.get(format!("{PAGE}/services"))
        .header("USER_AGENT", "Mozilla/5.0 (X11; Linux x86_64; rv:144.0) Gecko/20100101 Firefox/144.0")
        .header("ACCEPT", "text/html,application/xhtml+xml,application/xml;q=0.9,*/*;q=0.8")
        .header("ACCEPT_LANGUAGE", "en-US,en;q=0.5")
        .header("ACCEPT_ENCODING", "gzip, deflate, br, zstd")
        .header("Sec-GPC", "1")
        .header("CONNECTION", "keep-alive")
        .header("COOKIE", "PHPSESSID=7rkskb8ils3s8su7jrrh83q354;")
        .header("Upgrade-Insecure-Requests", "1")
        .header("Sec-Fetch-Dest", "document")
        .header("Sec-Fetch-Mode", "navigate")
        .header("Sec-Fetch-Site", "none")
        .header("Sec-Fetch-User", "?1")
        .header("Priority", "u=0, i")
        .header("TE", "trailers")
        .send()
        .await?
        .text()
        .await?;

    let re = regex::Regex::new(r"server\?id=(\d+)")?;

    Ok(match re.captures(&response) {
        Some(cap) => {
            let server_id = cap.get(1).map_or("", |m| m.as_str());
            println!("ID Server yang Ditemukan: {}", server_id);
            server_id.parse::<u64>()?
        },
        None => {
            println!("ID Server tidak ditemukan.");
            return Err("CANT FIND THE SERVER MAGMA ID");
        }
    })
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    loop {
        let (status, server_data) = match get_required_server_data().await {
            Ok(result) => (true, result),
            Err(_) => (false, ServerSummary::default()),
        };
        if status {
            execute_auto_start(&server_data.data.first().unwrap().attributes.uuid).await?;
        }
        tokio::time::sleep(tokio::time::Duration::from_secs(30)).await;
    }
    Ok(())
}
