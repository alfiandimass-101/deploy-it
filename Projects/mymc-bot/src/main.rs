use azalea::prelude::*;

// Modules
mod component;
mod handler;

// re-export
// use component::BotComponent;
use handler::handle;

#[tokio::main]
async fn main() {
    let account = Account::offline("itzbot");
    ClientBuilder::new()
    .set_handler(handle)
    .start(account, "in1.svrx.top:27674")
    .await.unwrap();
}