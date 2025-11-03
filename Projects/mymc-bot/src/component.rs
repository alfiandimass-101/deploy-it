use std::sync::{Arc, Mutex};
use azalea::prelude::*;

enum Task {
    Attack,
    DoNothing,
}

#[derive(Clone, Component)]
pub struct BotComponent {
    task: Arc<Mutex<Task>>,
}

impl Default for BotComponent {
    fn default() -> Self {
        BotComponent { 
            task: Arc::new(Mutex::new(Task::DoNothing)),
         }
    }
}