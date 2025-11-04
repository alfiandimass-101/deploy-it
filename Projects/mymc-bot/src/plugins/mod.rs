//! Modul ini mendefinisikan plugin-plugin yang digunakan oleh bot.

use azalea::{app::{PluginGroup, PluginGroupBuilder}, auto_reconnect::AutoReconnectPlugin, auto_respawn::AutoRespawnPlugin};

pub mod core;

/// Grup plugin yang digunakan oleh bot.
#[allow(dead_code)]
pub struct BotPlugins;

impl PluginGroup for BotPlugins {
    /// Membangun grup plugin.
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
        // .add(AutoReconnectPlugin)
        // .add(AutoRespawnPlugin)
        .add(core::CoreLogicPlugin)
    }
}
