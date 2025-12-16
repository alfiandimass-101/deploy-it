mod behavior;
mod chat;
mod handler;
mod state;

use azalea::{Account, ClientBuilder};
use handler::handle;

#[tokio::main]
async fn main() {
    unsafe {
        std::env::set_var("RUST_LOG", "off");
    }
    let account = Account::offline("botaqlepad");

    ClientBuilder::new()
        .add_plugins(azalea::pathfinder::PathfinderPlugin)
        .set_handler(handle)
        .start(account, "bakwanjagung.my.id")
        .await
        .unwrap();
}
