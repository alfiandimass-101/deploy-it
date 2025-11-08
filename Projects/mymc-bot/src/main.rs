use azalea::{Account, prelude::*};
use tracing_subscriber::util::SubscriberInitExt;

#[derive(Debug, Default, Component)]
pub struct BotState {

}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt().init();
    let account = Account::offline("itzbot");
    ClientBuilder::new()
    .reconnect_after(1)
    .set_handler(handler)
    Ok(())
}
