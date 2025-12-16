mod behavior;
mod chat;
mod handler;
mod state;

use azalea::{Account, ClientBuilder};
use handler::handle;

#[tokio::main]
async fn main() {
    let account = Account::offline("botaqlepad");

    ClientBuilder::new()
        .set_handler(handle)
        .start(account, "bakwanjagung.my.id")
        .await;
}
