use azalea::{app::{App, Plugin, Startup, Update}, ecs::system::Query, prelude::*};
use crate::bot::{component::{BotComponent, Task}, handler::{handle_chat, handle_init}};

/// Plugin untuk logika inti bot.
pub struct CoreLogicPlugin;

impl Plugin for CoreLogicPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, startup_message)
           .add_systems(Update, (handle_tick, handle_chat, handle_init));
    }
}

/// Sistem yang berjalan sekali saat startup untuk menampilkan pesan.
fn startup_message() {
    println!("CoreLogicPlugin berhasil dimuat!");
}

/// Sistem yang menangani logika setiap tick.
fn handle_tick(mut query: Query<(&mut BotComponent)>) {
    for (mut state) in query.iter_mut() {
        state.tick += 1;
        if state.tick >= 20 {
            state.tick = 0;
        }

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
}