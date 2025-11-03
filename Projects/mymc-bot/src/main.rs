use azalea::{app::{PluginGroup, PluginGroupBuilder}, auto_reconnect::AutoReconnectPlugin, auto_respawn::AutoRespawnPlugin, prelude::*};

// Modules
mod component;
mod handler;

// re-export
use component::BotComponent;
use handler::handle;

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
    let account = Account::offline("itzbot");
    ClientBuilder::new()
    .set_handler(handle)
    .add_plugins(BotPlugins)
    .set_state(BotComponent::default())
    .start(account, "in1.svrx.top:27674")
    .await.unwrap();
}