use std::sync::{Arc, Mutex};
use azalea::prelude::*;

pub enum Task {
    Attack,
    DoNothing,
}

#[derive(Clone, Component)]
pub struct BotComponent {
    pub task: Arc<Mutex<Task>>,
}

impl Default for BotComponent {
    fn default() -> Self {
        BotComponent { 
            task: Arc::new(Mutex::new(Task::DoNothing)),
         }
    }
}

impl BotComponent {
    pub fn get_task(&self) -> Arc<Mutex<Task>> {
        self.task.clone()
    }
}