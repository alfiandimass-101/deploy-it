//! Modul ini menangani event-event yang diterima oleh bot.

use azalea::{ClientInformation, prelude::*};
use super::{component::{BotComponent, Task}};

/// UUID dari pemilik bot.
const OWNER_UUID: uuid::Uuid = uuid::uuid!("452cb59a-adf3-3ebe-814b-53015c4e4279");

/// Fungsi yang dijalankan setiap tick untuk memproses perintah.
pub fn tick_commands(_bot: &Client, _event: &Event, state: &mut BotComponent) {
    let task = state.get_task();
    let task = task.lock().unwrap();

    match *task {
        Task::Attack => {
            // TODO: Implement attack logic
        }
        Task::DoNothing => {
            // Do nothing
        }
    }
}

pub fn owner_commands(msg: &str, bot: &Client, state: &mut BotComponent) {
    let word_collection: Vec<&str> = msg.trim().split_whitespace().collect::<Vec<&str>>();
    match msg {
        let content = word_collection.iter().next().iter().map(|&word| {
            *word + " "
        }).collect::<&str>();
        "!say" => bot.chat(content),
    }
}

/// Fungsi utama untuk menangani event-event yang diterima oleh bot.
pub async fn handle(mut bot: Client, mut event: Event, mut state: BotComponent) -> anyhow::Result<()> {
    match event {
        Event::Chat(msg) => {
            if let Some(uuid) = msg.sender_uuid() {
                if uuid == OWNER_UUID {
                    let content = msg.content();
                    println!("Owner said: {}", content);
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
            tick_commands(&bot, &event, &mut state);
        }
        _ => {}
    }
    Ok(())
}
