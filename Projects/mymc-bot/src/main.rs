use tracing_subscriber::util::SubscriberInitExt;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::registry().init();
    Ok(())
}
