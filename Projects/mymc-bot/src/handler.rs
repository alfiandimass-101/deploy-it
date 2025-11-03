use azalea::prelude::*;

async fn handle(bot: Client, event: Event, state: BotComponent) -> anyhow::Result<()> {
    match event {
        Event::Chat(msg) => {
            if let Some(uuid) = msg.sender_uuid() {
                if uuid == uuid::uuid!("452cb59a-adf3-3ebe-814b-53015c4e4279") {
                    println!("{}", msg.content());
                }
            } 
        }
        _ => {}
    }
    Ok(())
}
