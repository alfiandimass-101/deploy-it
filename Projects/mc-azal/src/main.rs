#[tokio::main]
async fn main() {
    // The username for our bot.
    // For offline-mode servers, this can be anything.
    let account = Account::offline("bot");

    // The address of the server to join.
    // "localhost" works if you are running a server on your own computer.
    let address = "localhost";

    azalea::start(azalea::Options {
        account,
        address,
        state: State::default(),
        plugins: plugins![],
        handle,
    })
    .await
    .unwrap();
}

// This struct holds the custom state for our bot.
// For this simple example, we don't need any state.
#[derive(Default, Clone)]
struct State {}

// This function is called every tick and for every event.
async fn handle(bot: Client, event: Event, _state: State) -> anyhow::Result<()> {
    match event {
        Event::Chat(m) => {
            // m is a ChatPacket, we can get the message from it and print it
            println!("{}", m.message().to_ansi());
        }
        _ => {}
    }

    Ok(())
}
