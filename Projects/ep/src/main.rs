use reqwest::{
    Client,
    header::{HeaderMap, HeaderValue}, multipart::Part,
};

use crate::utils::{ServerSummary, UploaderJson};
use std::error::Error;
use tokio::{io::AsyncReadExt, process::Command};

pub mod utils;

// const PHPSESID: &'static str = "7rkskb8ils3s8su7jrrh83q354";
// const PAGE: &'static str = "https://magmanode.com";
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
    let _ = Client::new()
        .post(format!("{PANEL}/api/client/servers/{server_uuid}/power"))
        .body("{\"signal\": \"start\"}")
        .headers(headers)
        .send()
        .await?;
    println!("[AUTO START EXECUTED]");
    Ok(())
}

pub async fn get_server_magma_id() -> Result<u64, Box<dyn Error>> {
    let command_shell = "curl 'https://magmanode.com/services' --compressed -H 'User-Agent: Mozilla/5.0 (X11; Linux x86_64; rv:144.0) Gecko/20100101 Firefox/144.0' -H 'Accept: text/html,application/xhtml+xml,application/xml;q=0.9,*/*;q=0.8' -H 'Accept-Language: en-US,en;q=0.5' -H 'Accept-Encoding: gzip, deflate, br, zstd' -H 'Sec-GPC: 1' -H 'Connection: keep-alive' -H 'Cookie: PHPSESSID=7rkskb8ils3s8su7jrrh83q354;' -H 'Upgrade-Insecure-Requests: 1' -H 'Sec-Fetch-Dest: document' -H 'Sec-Fetch-Mode: navigate' -H 'Sec-Fetch-Site: none' -H 'Sec-Fetch-User: ?1' -H 'Priority: u=0, i' -H 'TE: trailers' | grep -oP 'server\\?id=\\K\\d+' | head -n 1";

    let output = Command::new("sh")
        .arg("-c")
        .arg(command_shell)
        .output()
        .await?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);

        eprintln!("Error saat menjalankan perintah curl: {}", stderr);

        return Err(Box::<dyn Error>::from(
            "Failed to execute curl command successfully",
        ));
    }

    let stdout = String::from_utf8_lossy(&output.stdout);

    let server_id_str = stdout.trim();

    if server_id_str.is_empty() {
        return Err(Box::<dyn Error>::from(
            "CANT FIND THE SERVER MAGMA ID: Output was empty after running curl/grep.",
        ));
    }

    let server_id = server_id_str.parse::<u64>().map_err(|e| {
        format!(
            "Failed to parse server ID '{}' as u64: {}",
            server_id_str, e
        )
    })?;

    println!(
        "ID Server yang Ditemukan (via curl child process): {}",
        server_id
    );

    Ok(server_id)
}

pub async fn remove_server(server_id: u64) -> Result<(), Box<dyn Error>> {
    let command_shell = format!(
        "curl 'https://magmanode.com/services'   --compressed   -X POST   -H 'User-Agent: Mozilla/5.0 (X11; Linux x86_64; rv:144.0) Gecko/20100101 Firefox/144.0'   -H 'Accept: */*'   -H 'Accept-Language: en-US,en;q=0.5'   -H 'Accept-Encoding: gzip, deflate, br, zstd'   -H 'Content-Type: application/x-www-form-urlencoded; charset=UTF-8'   -H 'X-Requested-With: XMLHttpRequest'   -H 'Origin: https://magmanode.com'   -H 'Sec-GPC: 1'   -H 'Connection: keep-alive'   -H 'Referer: https://magmanode.com/services'   -H 'Cookie: PHPSESSID=7rkskb8ils3s8su7jrrh83q354'   -H 'Sec-Fetch-Dest: empty'   -H 'Sec-Fetch-Mode: cors'   -H 'Sec-Fetch-Site: same-origin'   -H 'Priority: u=0'   --data-raw 'delete_server=true&server_id={server_id}'"
    );

    let output = Command::new("sh")
        .arg("-c")
        .arg(command_shell)
        .output()
        .await?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);

        eprintln!("Error saat menjalankan perintah curl: {}", stderr);

        return Err(Box::<dyn Error>::from(
            "Failed to execute curl command successfully",
        ));
    }
    Ok(())
}

pub async fn create_server() -> Result<(), Box<dyn Error>> {
    let command_shell = "curl 'https://magmanode.com/free_version'   -X POST   -H 'User-Agent: Mozilla/5.0 (X11; Linux x86_64; rv:144.0) Gecko/20100101 Firefox/144.0'   -H 'Accept: text/html,application/xhtml+xml,application/xml;q=0.9,*/*;q=0.8'   -H 'Accept-Language: en-US,en;q=0.5'   -H 'Accept-Encoding: gzip, deflate, br, zstd'   -H 'Content-Type: application/x-www-form-urlencoded'   -H 'Origin: https://magmanode.com'   -H 'Sec-GPC: 1'   -H 'Connection: keep-alive'   -H 'Referer: https://magmanode.com/free_version'   -H 'Cookie: PHPSESSID=7rkskb8ils3s8su7jrrh83q354'   -H 'Upgrade-Insecure-Requests: 1'   -H 'Sec-Fetch-Dest: document'   -H 'Sec-Fetch-Mode: navigate'   -H 'Sec-Fetch-Site: same-origin'   -H 'Sec-Fetch-User: ?1'   -H 'Priority: u=0, i'   -H 'TE: trailers'   --data-raw 'select=Select+Version&version_name=Paper&server_name=ItzWoow&version=1.8.8' ";

    let output = Command::new("sh")
        .arg("-c")
        .arg(command_shell)
        .output()
        .await?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);

        eprintln!("Error saat menjalankan perintah curl: {}", stderr);

        return Err(Box::<dyn Error>::from(
            "Failed to execute curl command successfully",
        ));
    }
    Ok(())
}

pub async fn make_upload_url(server_identifier: &str) -> anyhow::Result<String> {
    let mut headers = HeaderMap::new();

    let auth_value = format!("Bearer {}", AUTH_TOKEN);
    headers.insert("Authorization", HeaderValue::from_str(&auth_value).unwrap());

    headers.insert("Accept", HeaderValue::from_static("application/json"));

    let client = Client::new();
    let result = client.get(format!("https://panel.magmanode.com/api/client/servers/{server_identifier}/files/upload?directory=%2F"))
    .headers(headers)
    .send().await?;

    let url = match serde_json::from_str::<UploaderJson>(&result.text().await?) {
        Ok(data) => {
            data.attributes.url
        },
        Err(e) => panic!("NO UPLOAD URL: {e}"),
    };
    println!("[MAKE UPLOAD DONE]");
    Ok(url)
}

pub async fn upload_file(url: &str, path: &str) -> anyhow::Result<()> {
    let mut file = tokio::fs::File::open(path).await?;
    let mut plugss_buffer = Vec::new();
    file.read_to_end(&mut plugss_buffer).await?;
    
    let local_file_part = Part::bytes(plugss_buffer).file_name("plugss.zip");
    let directory_part = Part::text("/".to_string());
    
    let form = reqwest::multipart::Form::new()
        .part("files", local_file_part)
        .part("directory", directory_part);

    let client = Client::new();
    client.post(url)
    .multipart(form)
    .send().await?;

    println!("upload done");
    Ok(())
}

pub async fn decompress_plugss(server_identifier: &str) -> anyhow::Result<()> {
    let mut headers = HeaderMap::new();

    let auth_value = format!("Bearer {}", AUTH_TOKEN);
    headers.insert("Authorization", HeaderValue::from_str(&auth_value).unwrap());

    headers.insert("Accept", HeaderValue::from_static("application/json"));
    headers.insert("Content-Type", HeaderValue::from_static("application/json"));

    let body = r#"{
        "root": "/",
        "file": "backup.zip"
    }"#;

    let client = Client::new();
    client.post(format!("https://panel.magmanode.com/api/client/servers/{server_identifier}/files/decompress"))
        .body(body)
        .send().await?;

    Ok(())
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
        } else {
            let id = get_server_magma_id().await.unwrap();
            remove_server(id).await.unwrap();
            create_server().await.unwrap();
            tokio::time::sleep(tokio::time::Duration::from_mins(1)).await;
            let upload_url = make_upload_url(&server_data.data.first().unwrap().attributes.identifier).await?;
            upload_file(&upload_url, "/home/runner/work/deploy-it/deploy-it/Projects/ep/plugss.zip").await?;
            decompress_plugss(&server_data.data.first().unwrap().attributes.identifier).await?;
        }
        tokio::time::sleep(tokio::time::Duration::from_secs(30)).await;
    }
}
