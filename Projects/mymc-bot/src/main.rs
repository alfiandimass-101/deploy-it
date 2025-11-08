use azalea::{Account, ClientInformation, prelude::*};
use tracing::{info, warn};

mod utils;
pub use utils::*;

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
                    let processed = process_owner_command(&content);
                    if let Some(command) = processed {
                        info!(name: "OWNER COMMAND", "COMMAND: {command}, ARG: {arg}", command=command.0, arg=command.1);
                        match command.0 {
                            "!say" => bot.chat(command.1),
                            "!killaura" => match command.0.parse::<u8>()? {
                                1 => state.can_kill = true,
                                0 => state.can_kill = false,
                                _ => warn!("Cannot assign killaura to n'either 0 or 1"),
                            }
                            _ => {}
                        }
                    } else {
                        info!(name: "OWNER LOG", "OWNER SAID: {}", content);
                    }
                }
            }
        }
        _ => {}
    }
    Ok(())
}