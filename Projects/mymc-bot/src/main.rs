use tokio::sync::{Mutex};
use std::sync::Arc;
use azalea::{Account, ClientInformation, prelude::*};
use tracing::info;

const OWNER_UUID: uuid::Uuid = uuid::uuid!("452cb59a-adf3-3ebe-814b-53015c4e4279");

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum TaskState {
    #[default]
    DoNothing,
    Attack,
}

#[derive(Debug, Default, Component, Clone)]
pub struct BotState {
    pub can_kill: bool,
    task: Arc<Mutex<TaskState>>,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt().init();
    let account = Account::offline("itzbot");
    ClientBuilder::new()
    .set_handler(handler)
    .start(account, "itzyuurz.aternos.me:11068")
    .await?;
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
            if let Some(uuid) = content_packet.sender_uuid() {
                if uuid == OWNER_UUID {
                    let content = content_packet.content();
                    info!(name: "OWNER LOG", "OWNER SAID: {}", content);
                    
                }
            }
        }
        _ => {}
    }
    Ok(())
}