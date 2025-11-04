//! Modul ini menangani event-event yang diterima oleh bot.

use azalea::{ClientInformation, prelude::*};
use super::{component::{BotComponent, Task}};


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


/// Fungsi utama untuk menangani event-event yang diterima oleh bot.
pub async fn handle(mut bot: Client, mut event: Event, mut state: BotComponent) -> anyhow::Result<()> {
    match event {
        Event::Chat(msg) => {
        }

        Event::Init => {
            bot.set_client_information(ClientInformation {
                view_distance: 32,
                ..Default::default()
            });
        }

        Event::Tick => {
        }
        _ => {}
    }
    Ok(())
}
