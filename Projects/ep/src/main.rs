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

pub async fn get_server_magma_id(page_url: &str) -> Result<u64, Box<dyn Error>> {

    let mut headers = HeaderMap::new();

    headers.insert(USER_AGENT, HeaderValue::from_static("Mozilla/5.0 (X11; Linux x86_64; rv:144.0) Gecko/20100101 Firefox/144.0"));

    headers.insert(ACCEPT, HeaderValue::from_static("text/html,application/xhtml+xml,application/xml;q=0.9,*/*;q=0.8"));

    headers.insert(ACCEPT_LANGUAGE, HeaderValue::from_static("en-US,en;q=0.5"));

    headers.insert(ACCEPT_ENCODING, HeaderValue::from_static("gzip, deflate, br, zstd"));

    headers.insert("Sec-GPC", HeaderValue::from_static("1"));

    headers.insert(CONNECTION, HeaderValue::from_static("keep-alive"));

    headers.insert(COOKIE, HeaderValue::from_static("PHPSESSID=7rkskb8ils3s8su7jrrh83q354;"));

    headers.insert("Upgrade-Insecure-Requests", HeaderValue::from_static("1"));

    headers.insert("Sec-Fetch-Dest", HeaderValue::from_static("document"));

    headers.insert("Sec-Fetch-Mode", HeaderValue::from_static("navigate"));

    headers.insert("Sec-Fetch-Site", HeaderValue::from_static("none"));

    headers.insert("Sec-Fetch-User", HeaderValue::from_static("?1"));

    headers.insert("Priority", HeaderValue::from_static("u=0, i"));

    headers.insert("TE", HeaderValue::from_static("trailers"));


    let client = Client::builder()

        .default_headers(headers)

        .build()?;

    

    let url = format!("{}/services", page_url);


    let response_text = client.get(&url)

        .send()

        .await?

        .text()

        .await?;


    let re = Regex::new(r"server\?id=(\d+)")?; 


    match re.captures(&response_text) {

        Some(cap) => {

            let server_id_str = cap.get(1)

                .ok_or_else(|| Box::<dyn Error>::from("Regex match found but capture group 1 is missing"))?

                .as_str();

            

            println!("ID Server yang Ditemukan: {}", server_id_str);

            

            let server_id = server_id_str.parse::<u64>()

                .map_err(|e| format!("Failed to parse server ID '{}' as u64: {}", server_id_str, e))?;


            Ok(server_id)

        },

        None => {

            println!("ID Server tidak ditemukan.");

            Err(Box::<dyn Error>::from("CANT FIND THE SERVER MAGMA ID in response"))

        }

    }

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
            let id = get_server_magma_id().await.unwrap();
        }
        tokio::time::sleep(tokio::time::Duration::from_secs(30)).await;
    }
    Ok(())
}
