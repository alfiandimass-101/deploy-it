use reqwest::{Client, header::HeaderMap};

const PHPSESID: &'static str = "7rkskb8ils3s8su7jrrh83q354";
const PAGE: &'static str = "https://magmanode.com";
const PANEL: &'static str = "https://panel.magmanode.com";

async fn get_server_data() -> anyhow::Result<()> {
    let mut ptero_info_header = HeaderMap::new();
    ptero_info_header.insert("Authorization", "Bearer ptlc_8JGKmhuz2JydQ0Ax8Ko7MKopPTeWln8mJi2cmZm0Uam".parse()?);
    ptero_info_header.insert("Accept", "application/json".parse()?);
    ptero_info_header.insert("application/json", "application/json".parse()?);
    let client = Client::new()
    .get(format!("{PANEL}/api/client"))
    .headers(ptero_info_header);
    let result = client.send().await?;
    println!("{:?}", result);
    Ok(())
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    Ok(())
}