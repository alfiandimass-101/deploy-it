//! Logika untuk tugas Chunk Eater.

use std::{thread, time::Duration};
use azalea::Client;
use crate::bot::component::BotComponent;

/// Fungsi utama untuk menjalankan logika chunk eater.
/// Ini adalah fungsi yang akan kita jalankan di `spawn_blocking`.
///
/// # Arguments
/// * `bot` - Klon dari `Client` bot.
/// * `state` - Klon dari `BotComponent` state.
pub fn run(bot: Client, state: BotComponent) {
    println!("Bot: Memulai tugas Chunk Eater... (simulasi komputasi berat)");

    // Simulasi pekerjaan berat selama 10 detik.
    // Di sini Anda akan menempatkan logika untuk menghancurkan chunk.
    thread::sleep(Duration::from_secs(10));

    println!("Bot: Tugas Chunk Eater selesai.");
    state.set_task(super::super::component::Task::DoNothing);
}
