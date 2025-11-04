//! Modul ini menangani event-event yang diterima oleh bot.

use azalea::{prelude::*, swarm::event::chat::ChatEvent, ClientInformation};

/// Sistem untuk menangani event chat.
pub fn handle_chat(event: Event, mut bot: Client) {
    if let Event::Chat(msg) = event {
        // Logika untuk menangani pesan chat
    }
}

/// Sistem untuk menangani event inisialisasi bot.
pub fn handle_init(mut bot: Client) {
    bot.set_client_information(ClientInformation {
        view_distance: 32,
        ..Default::default()
    });
}
