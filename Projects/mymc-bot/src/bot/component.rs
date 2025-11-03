//! Modul ini mendefinisikan status dan komponen bot.

use std::sync::{Arc, Mutex};
use azalea::prelude::*;

/// Tipe untuk menyimpan tugas bot dengan aman.
type TaskModel<T> = Arc<Mutex<T>>;

/// Enum yang merepresentasikan tugas yang dapat dilakukan oleh bot.
pub enum Task {
    /// Tugas untuk menyerang entitas.
    Attack,
    /// Tugas untuk tidak melakukan apa-apa.
    DoNothing,
}

/// Komponen yang menyimpan status bot.
#[derive(Clone, Component)]
pub struct BotComponent {
    /// Tugas yang sedang dijalankan oleh bot.
    pub task: TaskModel<Task>,
}

impl Default for BotComponent {
    /// Membuat `BotComponent` baru dengan tugas default `Task::DoNothing`.
    fn default() -> Self {
        BotComponent {
            task: Arc::new(Mutex::new(Task::DoNothing)),
         }
    }
}

impl BotComponent {
    /// Mendapatkan tugas bot saat ini.
    pub fn get_task(&self) -> TaskModel<Task> {
        self.task.clone()
    }

    /// Mengatur tugas bot ke status baru.
    pub fn set_task(&mut self, new_state: Task) {
        let mut task = self.task.lock().unwrap();
        *task = new_state;
    }
}