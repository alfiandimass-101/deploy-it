use azalea::{Account, ClientInformation, prelude::*};
use tracing_subscriber::util::SubscriberInitExt;
use uuid::uuid;

const OWNER_UUID: uuid::Uuid = uuid!("452cb59a-adf3-3ebe-814b-53015c4e4279");

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
async fn handler(mut bot: Client, mut event: Event, mut state: BotState) -> anyhow::Result<()> {
    match event {
        Event::Init => {
            bot.set_client_information(ClientInformation {
                view_distance: 32u8,
                ..Default::default()
            });
        }
        Event::Chat(content_packet) => {
            let uuid = content_packet.
        }
    }
}