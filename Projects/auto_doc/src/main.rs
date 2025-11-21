use anyhow::{Context, Result};
use reqwest::Client;
use scraper::{Html, Selector};
use std::collections::{HashSet, VecDeque};
use std::env;
use tokio::fs::OpenOptions;
use tokio::io::AsyncWriteExt;
use url::Url;

#[tokio::main]
async fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <url>", args[0]);
        return Ok(());
    }

    let start_url_str = &args[1];
    let start_url = Url::parse(start_url_str).context("Failed to parse URL")?;

    // Infer crate name
    let crate_name = start_url
        .path_segments()
        .and_then(|mut segments| segments.next())
        .map(|s| s.to_string())
        .unwrap_or_else(|| "output".to_string());
    let output_filename = format!("{}_rs.txt", crate_name);

    println!("Starting scraper for {}", start_url);
    println!("Output will be saved to {}", output_filename);

    let client = Client::builder()
        .user_agent("Mozilla/5.0 (compatible; RustDocsScraper/0.1)")
        .build()?;

    let mut visited = HashSet::new();
    let mut queue = VecDeque::new();

    // Handle potential redirect on first request to establish true base URL
    let first_resp = client.get(start_url.clone()).send().await?;
    let base_url = first_resp.url().clone();

    // If the base_url is different (e.g. redirected from latest to specific version), use it.
    println!("Base URL resolved to: {}", base_url);

    queue.push_back(base_url.clone());
    visited.insert(base_url.clone());

    // Create/Truncate file
    tokio::fs::write(&output_filename, "").await?;

    let mut file = OpenOptions::new()
        .append(true)
        .open(&output_filename)
        .await?;

    let main_selector = Selector::parse("#main-content").unwrap();
    let a_selector = Selector::parse("a").unwrap();

    while let Some(url) = queue.pop_front() {
        println!("Scraping: {}", url);

        // We already fetched the first one to get the URL, but for simplicity in the loop, we'll fetch again
        // or we could have optimized. Let's just fetch. The overhead is one request.
        // Actually, we need the content.

        let resp = match client.get(url.clone()).send().await {
            Ok(r) => r,
            Err(e) => {
                eprintln!("Failed to fetch {}: {}", url, e);
                continue;
            }
        };

        if !resp.status().is_success() {
            eprintln!("Error status {} for {}", resp.status(), url);
            continue;
        }

        let html = match resp.text().await {
            Ok(t) => t,
            Err(e) => {
                eprintln!("Failed to get text for {}: {}", url, e);
                continue;
            }
        };

        let document = Html::parse_document(&html);

        if let Some(main) = document.select(&main_selector).next() {
            let text = main.text().collect::<Vec<_>>().join(" ");
            let clean_text = text.split_whitespace().collect::<Vec<_>>().join(" ");

            let header = format!("\n\n--- URL: {} ---\n\n", url);
            file.write_all(header.as_bytes()).await?;
            file.write_all(clean_text.as_bytes()).await?;
            file.write_all(b"\n").await?;
        }

        // Find links
        for element in document.select(&a_selector) {
            if let Some(href) = element.value().attr("href") {
                if let Ok(absolute_url) = url.join(href) {
                    let mut normalized = absolute_url.clone();
                    normalized.set_fragment(None);

                    if !visited.contains(&normalized) {
                        // Check if it is within the same scope
                        // We use the base_url as the prefix.
                        // e.g. base: https://docs.rs/azalea/0.1.0/azalea/
                        // link: https://docs.rs/azalea/0.1.0/azalea/struct.Bot.html -> OK
                        // link: https://docs.rs/azalea/0.1.0/azalea/prelude/index.html -> OK
                        // link: https://docs.rs/tokio/... -> NO

                        if normalized.as_str().starts_with(base_url.as_str()) {
                            visited.insert(normalized.clone());
                            queue.push_back(normalized);
                        }
                    }
                }
            }
        }

        // Sleep to be polite
        tokio::time::sleep(std::time::Duration::from_millis(20)).await;
    }

    println!("Scraping completed. {} pages visited.", visited.len());

    Ok(())
}
