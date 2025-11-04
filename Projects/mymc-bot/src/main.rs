//! Bot Minecraft sederhana yang dibuat dengan Azalea.

use azalea::prelude::*;

mod bot;
mod plugins;

use bot::component::BotComponent;

use plugins::BotPlugins;



/// Fungsi utama untuk menjalankan bot.

#[tokio::main]

async fn main() {

    let account = Account::offline("itzyuurzbot");

    ClientBuilder::new()

    .add_plugins(BotPlugins)

    .set_state(BotComponent::default())

    .start(account, "itzyuurz.aternos.me:11068")

    .await.unwrap();

}
