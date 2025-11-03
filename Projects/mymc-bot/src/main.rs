use azalea::{app::{PluginGroup, PluginGroupBuilder}, prelude::*};

// Modules
mod component;
mod handler;

// re-export
use component::BotComponent;
use handler::handle;

struct BotPlugins;

impl PluginGroup for BotPlugins {
    fn
}

#[tokio::main]
async fn main() {
    let plugins = PluginGroupBuilder::
    let account = Account::offline("itzbot");
    ClientBuilder::new()
    .set_handler(handle)
    .add_plugins(plugins)
    .set_state(BotComponent::default())
    .start(account, "in1.svrx.top:27674")
    .await.unwrap();
}