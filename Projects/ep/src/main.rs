use reqwest::{Client, header::{HeaderMap, HeaderValue}};

const PHPSESID: &'static str = "7rkskb8ils3s8su7jrrh83q354";
const PAGE: &'static str = "https://magmanode.com";
const PANEL: &'static str = "https://panel.magmanode.com";

pub async fn get_server_data() -> Result<()> {
    let mut headers = HeaderMap::new();

    let auth_value = format!("Bearer {}", AUTH_TOKEN);
    headers.insert(
        "Authorization", 
        HeaderValue::from_str(&auth_value)?
    );

    headers.insert("Accept", HeaderValue::from_static("application/json"));
    headers.insert("Content-Type", HeaderValue::from_static("application/json"));

    let client = Client::new();
    let url = format!("{}/api/client", PANEL);

    let result = client
        .get(url)
        .headers(headers)
        .send()
        .await?;
    
    println!("Status Respons: {:?}", result.status());
    let body = result.text().await?;
    println!("Isi Respons:\n{}", body);

    Ok(())
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    get_server_data().await?;
    Ok(())
}