use azalea::prelude::*;

#[tokio::main]
async fn main() {
    let account = Account::offline("itzbot");
    let client = ClientBuilder::new()
    .set_handler(handle)
    .start(account, "in1.svrx.top:27674")
    .await.unwrap();

}

#[derive(Debug, Default, Clone, Component)]
pub struct BotComponent {
}

async fn handle(bot: Client, event: Event, state: BotComponent) -> anyhow::Result<()> {
    match event {
        Event::Chat(msg) => {
            if let Some(uuid) = msg.sender() {
                println!("{:?}: {:?}: {}", msg.sender(), msg.sender_uuid(), msg.content());
                if uuid.as_str() == "452cb59a-adf3-3ebe-814b-53015c4e4279" {
                    println!("{}", msg.content());
                }
            } 
        }
        _ => {}
    }
    Ok(())
}
