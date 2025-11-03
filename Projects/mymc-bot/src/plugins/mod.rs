//! Modul ini mendefinisikan plugin-plugin yang digunakan oleh bot.
//! Modul untuk menampung semua logika tugas bot.

use azalea::{app::{PluginGroup, PluginGroupBuilder}, auto_reconnect::AutoReconnectPlugin, auto_respawn::AutoRespawnPlugin};

/// Grup plugin yang digunakan oleh bot.
#[allow(dead_code)]
pub struct BotPlugins;

impl PluginGroup for BotPlugins {
    /// Membangun grup plugin.
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
        .add(AutoReconnectPlugin)
        .add(AutoRespawnPlugin)
    }
}
pub mod chunk_eater;
