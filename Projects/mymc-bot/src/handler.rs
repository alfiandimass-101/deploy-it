use azalea::{ClientInformation, prelude::*};
use crate::component::BotComponent;

pub fn tick_commands(bot: &Client, event: &Event, state: &mut BotComponent) -> impl FnMut() {
    let mut task = state.get_task();
    // if task.lock()
}

pub async fn handle(mut bot: Client, mut event: Event, mut state: BotComponent) -> anyhow::Result<()> {
    match event {
        Event::Chat(msg) => {
            if let Some(uuid) = msg.sender_uuid() {
                if uuid == uuid::uuid!("452cb59a-adf3-3ebe-814b-53015c4e4279") {
                    println!("{}", msg.content());
                }
            }
        }

        Event::Init => {
            bot.set_client_information(ClientInformation {
                view_distance: 32,
                ..Default::default()
            });
        }

        Event::Tick => {
            let mut tick_fn = tick_commands(&bot, &event, &mut state);
            tick_fn();
        }
        _ => {}
    }
    Ok(())
}
