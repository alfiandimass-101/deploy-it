//! Bot Minecraft sederhana yang dibuat dengan Azalea.

use azalea::{prelude::*, app::App};

mod bot;
mod plugins;

use bot::component::BotComponent;
use plugins::BotPlugins;

#[tokio::main]
async fn main() {
    let account = Account::offline("itzyuurzbot");

    App::new()
        .add_plugins(BotPlugins)
        .insert_resource(BotComponent::default())
        .run(account, "OWCC.minehut.gg")
        .await
        .unwrap();
}