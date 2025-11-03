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
        Task::DoChunkEater => {
            // Logika sedang berjalan di thread terpisah, jangan lakukan apa-apa di sini.
        }
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

                    if content == "chunk" {
                        // Hanya mulai jika tidak sedang melakukan tugas lain
                        let task = state.get_task();
                        let mut current_task = task.lock().unwrap();
                        if let Task::DoNothing = *current_task {
                            println!("Bot: Menerima perintah 'chunk', akan memulai tugas.");
                            *current_task = Task::DoChunkEater;

                            // Klon bot dan state untuk dipindahkan ke thread baru
                            let bot_clone = bot.clone();
                            let state_clone = state.clone();
                            // tokio::task::spawn_blocking(move || tasks::chunk_eater::run(bot_clone, state_clone));
                        }
                    }
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
