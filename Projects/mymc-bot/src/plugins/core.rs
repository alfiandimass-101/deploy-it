use azalea::{app::{App, Plugin, Startup}, prelude::*};

/// Plugin untuk logika inti bot.
pub struct CoreLogicPlugin;

impl Plugin for CoreLogicPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, startup_message);
    }
}

/// Sistem yang berjalan sekali saat startup untuk menampilkan pesan.
fn startup_message() {
    println!("CoreLogicPlugin berhasil dimuat!");
}