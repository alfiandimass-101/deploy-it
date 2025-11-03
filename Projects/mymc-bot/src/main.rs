use azalea::{app::{PluginGroup, PluginGroupBuilder}, auto_reconnect::AutoReconnectPlugin, auto_respawn::AutoRespawnPlugin, prelude::*};

// Modules
mod component;
mod handler;

// re-export
use component::BotComponent;
use handler::handle;

#[allow(dead_code)]
struct BotPlugins;

impl PluginGroup for BotPlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
        .add(AutoReconnectPlugin)
        .add(AutoRespawnPlugin)
    }
}

#[tokio::main]
async fn main() {
    let account = Account::offline("itzyuurzbot");
    ClientBuilder::new()
    .set_handler(handle)
    // .add_plugins(BotPlugins)
    .set_state(BotComponent::default())
    .start(account, "itzyuurz.aternos.me:11068")
    .await.unwrap();
}