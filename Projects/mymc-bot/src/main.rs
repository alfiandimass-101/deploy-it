use tracing_subscriber::util::SubscriberInitExt;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let m = tracing_subscriber::fmt::fmt();
    Ok(())
}
